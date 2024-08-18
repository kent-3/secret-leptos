#[derive(thiserror::Error, serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum Error {
    #[error("An error occurred: {0}")]
    GenericError(String),

    #[error("A Secret error occurred: {0}")]
    SecretError(String),

    #[error(transparent)]
    KeplrError(#[from] ::keplr::Error),

    #[error("Keplr is not enabled!")]
    KeplrDisabled,
}

impl From<rsecret::Error> for Error {
    fn from(error: rsecret::Error) -> Self {
        Error::SecretError(error.to_string())
    }
}

impl Error {
    pub fn generic(message: impl ToString) -> Self {
        let message = message.to_string();
        Error::GenericError(message)
    }
}
