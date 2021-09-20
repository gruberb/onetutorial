use std::collections::HashMap;

use sheets4::Error;
use log::info;
use serde::{Serialize, Deserialize};
use lazy_static::lazy_static;
use google_sheets4::{Sheets, api::ValueRange};
use yup_oauth2::read_service_account_key;

lazy_static! {
    static ref CRYPTO_TOKEN_PRICES: String = dotenv::var("CRYPTO_TOKEN_PRICES").expect("CRYPTO_TOKEN_PRICES not set in ENV");
    static ref ETF: String = dotenv::var("ETF").expect("ETF not set in ENV");
    static ref CRYPTO: String = dotenv::var("CRYPTO").expect("CRYPTO not set in ENV");
    static ref NET_WORTH: String = dotenv::var("NET_WORTH").expect("NET_WORTH not set in ENV");
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Spreadsheet {
    pub id: String,
    pub secret_path: String,
    pub values: Option<HashMap<String, Vec<String>>>,
}

impl Spreadsheet {
    pub fn new(id: &str, secret_path: &str) -> Self {
        Spreadsheet {
            id: id.to_owned(),
            secret_path: secret_path.to_owned(),
            values: None,
        }
    }

    fn create_values(&self, values: Vec<ValueRange>) -> HashMap<String, Vec<String>> {
        let mut res: HashMap<String, Vec<String>> = HashMap::new();

        for v in values {    
            if v.range == Some(CRYPTO_TOKEN_PRICES.to_owned()) {
                res.insert("CRYPTO_TOKEN_PRICES".to_string(), v.values.clone().unwrap()[0].to_owned());
            }
    
            if v.range == Some(ETF.to_owned()) {
                res.insert("ETF".to_string(), v.values.clone().unwrap()[0].to_owned());
            }

            if v.range == Some(CRYPTO.to_owned()) {
                res.insert("CRYPTO".to_string(), v.values.clone().unwrap()[0].to_owned());
            }
    
            if v.range == Some(NET_WORTH.to_owned()) {
                res.insert("NET_WORTH".to_string(), v.values.clone().unwrap()[0].to_owned());
            }
    
        }

        res
    }

    pub async fn update_sheet(&self, values: ValueRange) {
        let authenticator = yup_oauth2::ServiceAccountAuthenticator::builder(
            read_service_account_key(&self.secret_path).await.unwrap(),
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
             .values_update(values.clone(), &self.id, &values.range.unwrap())
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
    
    pub async fn fetch_latest_values(&mut self) -> &mut Self {
        let authenticator = yup_oauth2::ServiceAccountAuthenticator::builder(read_service_account_key(&self.secret_path).await.unwrap())
            .build()
            .await
            .expect("failed to create authenticator");

        let sheets = Sheets::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), authenticator);

        let res = sheets.spreadsheets().values_batch_get(&self.id)
            .add_ranges(&CRYPTO_TOKEN_PRICES)
            .add_ranges(&ETF)
            .add_ranges(&CRYPTO)
            .add_ranges(&NET_WORTH)
            .major_dimension("COLUMNS")
            .doit().await;
        
        let (_, values) = res.unwrap();
        
        self.values = Some(self.create_values(values.value_ranges.unwrap()));
       
        self
    }

    pub fn get_values(&self, value_range: &str) -> Vec<String> {
        self.values.as_ref()
            .expect("No spreadsheet data is initalised yet")
            .get(value_range)
            .expect(&format!("{} not initialised yet", value_range)).to_vec()
    }
}
