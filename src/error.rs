use std::fmt;

#[derive(Debug)]
pub enum OneError {
    NoAPIKey,
    CSV(csv::Error),
    IO(std::io::Error),
    Reqwest(reqwest::Error),
}

impl std::error::Error for OneError {}


impl fmt::Display for OneError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      OneError::NoAPIKey => write!(f, "No API key is set via the .env variable."),
      OneError::CSV(err) => write!(f, "Error while writing the CSV file {}", err),
      OneError::IO(err) => write!(f, "Error while flushing the file {}", err),
      OneError::Reqwest(err) => write!(f, "Error while fetching data {}", err),
    }
  }
}

impl From<reqwest::Error> for OneError {
    fn from(err: reqwest::Error) -> OneError {
        OneError::Reqwest(err)
    }
}

impl From<csv::Error> for OneError {
    fn from(err: csv::Error) -> OneError {
        OneError::CSV(err)
    }
}

impl From<std::io::Error> for OneError {
    fn from(err: std::io::Error) -> OneError {
        OneError::IO(err)
    }
}


