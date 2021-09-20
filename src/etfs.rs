use crate::email::HTML;

pub struct ETF {
    pub name: String,
    pub price: String,
}

pub struct ETFs(Vec<ETF>);

impl ETFs {
    pub fn new(prices: Vec<String>) -> Self {
        let etfs = vec![
            ETF { name: String::from("WSRI-PA"), price: prices[0].to_owned() },
        ];

        Self {
            0: etfs,
        }
    }
}

impl HTML for ETFs {
    fn to_email_body(&self) -> String {
        let mut res = String::new();

        for e in &self.0 {
            let s = format!("
                <tr>
                    <td>{}</td>
                    <td align='right'>{}</td>
                </tr>
                ",
                e.name,
                e.price,
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
