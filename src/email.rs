use sendmail::email;

use lazy_static::lazy_static;

pub struct EMail {
    pub body: String,
}

lazy_static! {
    static ref FROM_ADDRESS: &'static str = "onetutorial@rustmeetup.com";
    static ref TO_ADDRESS: Vec<&'static str> = vec!["byteadventures1@gmail.com"];
}

pub trait HTML {
    fn to_email_body(&self) -> String;
}

impl EMail {
    pub fn new<C: ?Sized>(components: Vec<&C>) -> Self 
        where C: HTML 
        {

            let mut body = String::from("");

            for c in components {
                body.push_str(&c.to_email_body());
            }

            Self {
                body,
            }
        }

    pub fn send(&self) -> Result<(), std::io::Error>{
        email::send(
            // From Address
            &FROM_ADDRESS,
            // To Address
            <Vec<&str> as AsRef<[&str]>>::as_ref(&TO_ADDRESS.to_vec()),
            // Subject
            &format!("Personal Finance Newsletter"),
            // Body
            &self.body,
        )
    }

}
