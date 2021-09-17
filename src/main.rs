use std::collections::HashMap;
use std::fmt;
use serde::{Serialize, Deserialize};
use clap::{Arg, App};
use csv::Writer;

#[derive(Serialize, Deserialize, Debug)]
struct CMCResponse {
    data: HashMap<String, Currency>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Currency {
    name: String,
    symbol: String,
    quote: Quotes,
}

#[derive(Serialize, Deserialize, Debug)]
struct Quotes(HashMap<String, Quote>);

#[derive(Serialize, Deserialize, Debug)]
struct Quote {
    price: f64,
    percent_change_7d: f64,
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Name: {}, Symbol: {} Price: {} Change(7d): {}%", 
            self.name, 
            self.symbol, 
            self.quote.0.get("USD").unwrap().price.to_string(), 
            self.quote.0.get("USD").unwrap().percent_change_7d.to_string()
        )
    }
}

impl CMCResponse {
    fn get_currency(&self, currency: &str) -> Option<&Currency> {
        self.data.get(currency)
    }
}
 
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
     dotenv::dotenv().ok();

    let matches = App::new("OnteTutorial")
    .version("1.0")
    .author("Bastian G. <code@recv.online>")
    .about("Learn Rust in one go")
    .arg(Arg::new("currency_list")
        .long("currencies")
        .about("Pass the list of currencies you want to query")
        .min_values(1)
        .required(true))
    .get_matches();

    let currencies = matches.value_of("currency_list").expect("No currencies were being passed");  
    
    let cmc_pro_api_key = dotenv::var("CMC_PRO_API_KEY").expect("CMC key not set");

    let mut params = HashMap::new();
    params.insert("symbol", currencies.to_string());
    
    let client = reqwest::Client::new();
    let resp = client
        .get(" https://pro-api.coinmarketcap.com/v1/cryptocurrency/quotes/latest")
        .header("X-CMC_PRO_API_KEY", cmc_pro_api_key)
        .query(&params)
        .send()
        .await?;
    
    let currencies = resp.json::<CMCResponse>().await?;
   
    let mut wtr = Writer::from_path("prices.csv")?;
    wtr.write_record(&["Name", "Symbol", "Price", "7DayChange"])?;
    
    for (symbol, currency) in currencies.data.into_iter() {
        wtr.write_record(&[currency.name, symbol.to_owned(), currency.quote.0.get("USD").unwrap().price.to_string(), currency.quote.0.get("USD").unwrap().percent_change_7d.to_string()])?;
    }

    wtr.flush()?;
    
    Ok(())
}
