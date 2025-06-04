use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Response<T> {
    #[serde(alias = "statusCode")]
    pub status_code: usize,

    pub message: String,

    pub body: T,
}
