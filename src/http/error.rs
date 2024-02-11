use thiserror::Error as thisError;

#[derive(Debug, thisError)]
pub enum Error {
    #[error("An error has occured while sending a request to the server. {error}")]
    HttpError { error: reqwest::Error },
    #[error("An error has occured while reading the response from the server. {error}")]
    ReadingError { error: reqwest::Error },
    #[error("An error has occured while creating the http client. Couldn't set header value (header: {header}, value: {value}):\n{error}")]
    InvalidHeaderValue {
        header: String,
        value: String,
        error: reqwest::header::InvalidHeaderValue,
    },
    #[error("An error has occured while creating the http client. {error}")]
    HttpClientBuilderError {
        error: reqwest::Error
    },
    #[error("An error has occured while parsing the response from the server. {error}\nNear: {surroundings:?}")]
    ParsingError { error: serde_json::Error, surroundings: Option<String>, body: Option<String> },
}




