use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("failed to query for '{query}'")]
    SearchError {
        query: String,
        source: Box<dyn std::error::Error>,
    },
}
