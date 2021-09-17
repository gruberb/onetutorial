use std::collections::HashMap;
use std::fmt;

use serde::{Serialize, Deserialize};

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
    
    let cmc_pro_api_key = dotenv::var("CMC_PRO_API_KEY").expect("CMC key not set");

    let mut params = HashMap::new();
    params.insert("symbol", "BTC");
    
    let client = reqwest::Client::new();
    let resp = client
        .get(" https://pro-api.coinmarketcap.com/v1/cryptocurrency/quotes/latest")
        .header("X-CMC_PRO_API_KEY", cmc_pro_api_key)
        .query(&params)
        .send()
        .await?;
    
    let prices = resp.json::<CMCResponse>().await?;
   
    if let Some(bitcoin) = prices.get_currency("BTC") {
        println!("{}", bitcoin);
    } else {
        println!("Bitcoin is not in the list");
    }

    Ok(())
}
