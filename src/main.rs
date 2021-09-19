use std::collections::HashMap;

use clap::{Arg, App};
use log::{error, info, debug};
use log4rs;

extern crate google_sheets4 as sheets4;
extern crate yup_oauth2 as oauth2;
use sheets4::api::ValueRange;
use sheets4::Error;
use sheets4::Sheets;
use yup_oauth2::read_service_account_key;

#[macro_use]
extern crate lazy_static;

mod cmc;
mod eod;
mod error;

use cmc::CMCResponse;
use eod::EODResponse;
use error::OneError;

lazy_static! {
    static ref SHEET_ID: &'static str = "1JuF7MpFIkZSixwnmuvgH5KN5iPzcH9Xd05hTL0glya0";
    static ref SECRET_PATH: &'static str = "secret.json";
}

#[tokio::main]
async fn main() -> Result<(), OneError> {
    dotenv::dotenv().ok();
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    let matches = App::new("OnteTutorial")
    .version("1.0")
    .author("Bastian G. <code@recv.online>")
    .about("Learn Rust in one go")
    .arg(Arg::new("currency_list")
        .long("currencies")
        .about("Pass the list of currencies you want to query")
        .min_values(1)
        .required(true))
    .arg(Arg::new("etfs")
        .long("etfs")
        .about("Pass the ETF symbols to fetch prices for")
        .takes_value(true)
        .required(true))
    .get_matches();

    let currency_list = matches.value_of("currency_list").expect("No currencies were being passed");  
    let etfs = matches.value_of("etfs").expect("No ETF symbol passed");

    debug!("Querying the following currencies: {:?}", currency_list);

    let cmc_pro_api_key = dotenv::var("CMC_PRO_API_KEY").expect("CMC key not set");
    let eod_token = dotenv::var("EOD_TOKEN").expect("EOD token not set");
    
    if cmc_pro_api_key.is_empty() {
        error!("Empty CMC API KEY provided! Please set one via the .env file!");
        return Err(OneError::NoAPIKey);
    }

    let mut params = HashMap::new();
    params.insert("symbol", currency_list.to_string());
    
    let client = reqwest::Client::new();
    let resp = client
        .get(" https://pro-api.coinmarketcap.com/v1/cryptocurrency/quotes/latest")
        .header("X-CMC_PRO_API_KEY", cmc_pro_api_key)
        .query(&params)
        .send()
        .await?;

    let currencies = resp.json::<CMCResponse>().await?;

    let etf = client
        .get(format!("https://eodhistoricaldata.com/api/real-time/{}?api_token={}&fmt=json", etfs, eod_token))
        .send()
        .await?;

    let amundi_etf =etf.json::<EODResponse>().await?;
    
    debug!("Fetched ETF: {}", amundi_etf.close);

    let coins = ValueRange {
        major_dimension: Some("COLUMNS".to_string()),
        range: Some(format!("{}!{}2:{}4", "Crypto", "C", "C").to_owned()),
        values: Some(vec![vec![
            currencies.data.get(&"BTC".to_owned()).unwrap().quote.0.get("USD").unwrap().price.to_string(),
            currencies.data.get(&"ETH".to_owned()).unwrap().quote.0.get("USD").unwrap().price.to_string(),
            currencies.data.get(&"DOGE".to_owned()).unwrap().quote.0.get("USD").unwrap().price.to_string(),
        ]]),
    };
    
    let etfs = ValueRange {
        major_dimension: Some("COLUMNS".to_string()),
        range: Some(format!("{}!{}2:{}2", "ETFs", "C", "C").to_owned()),
        values: Some(vec![vec![
            amundi_etf.close.to_string(),
        ]])
    };

    update_google_sheet(&SECRET_PATH, coins).await;
    update_google_sheet(&SECRET_PATH, etfs).await;
   
    Ok(())
}

async fn update_google_sheet(
    secret_path: &str,
    values: ValueRange,
) {
    let authenticator = yup_oauth2::ServiceAccountAuthenticator::builder(
        read_service_account_key(secret_path).await.unwrap(),
    )
    .build()
    .await
    .expect("failed to create authenticator");

    let hub = Sheets::new(
        hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()),
        authenticator,
    );

    let range = values.clone().range.unwrap();
    let result = hub
         .spreadsheets()
         .values_update(values.clone(), &SHEET_ID, &values.range.unwrap())
         .value_input_option("USER_ENTERED")
         .doit()
         .await;

     match result {
         Err(e) => match e {
             // The Error enum provides details about what exactly happened.
             // You can also just use its `Debug`, `Display` or `Error` traits
             Error::HttpError(_)
             | Error::Io(_)
             | Error::MissingAPIKey
             | Error::MissingToken(_)
             | Error::Cancelled
             | Error::UploadSizeLimitExceeded(_, _)
             | Error::Failure(_)
             | Error::BadRequest(_)
             | Error::FieldClash(_)
             | Error::JsonDecodeError(_, _) => eprintln!("{}", e),
         },
         Ok((_, _)) => info!(
             "{} Updated range: {}",
             chrono::offset::Utc::now(),
             range
         ),
     }
}
