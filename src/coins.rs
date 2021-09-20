use crate::email::HTML;

pub struct Coin {
    pub name: String,
    pub price: String,
}

pub struct Coins(Vec<Coin>);

impl Coins {
    pub fn new(prices: Vec<String>) -> Self {
        let coins = vec![
            Coin { name: String::from("BTC"), price: prices[0].to_owned() },
            Coin { name: String::from("ETH"), price: prices[1].to_owned() },
            Coin { name: String::from("DOGE"), price: prices[2].to_owned() },
        ];

        Self {
            0: coins,
        }
    }
}

impl HTML for Coins {
    fn to_email_body(&self) -> String {
        let mut res = String::new();

        for c in &self.0 {
            let s = format!("
                <tr>
                    <td>{}</td>
                    <td align='right'>{}</td>
                </tr>
                ",
                c.name,
                c.price,
            );

            res.push_str(&s);
        }

        format!("
            <table style='border-spacing: 10px;'>
                <tr>
                    <th align='left'>Ticker</th>
                    <th align='right'>Price</th>
                </tr>
                {}
            </table>
        ", res)
    }
}
