use async_trait::async_trait;
use youtube_api::{
    models::SearchRequestBuilder,
    YoutubeApi,
};
use crate::error::ApiError;

#[async_trait]
pub trait Youtube {
    fn new(key: &str) -> Self;
    async fn query(&self, q: &str) -> Result<YoutubeResult, ApiError>;
}

pub struct YoutubeResult {}

pub struct YoutubeClient {
    // TODO does this need to be behind an ARC?
    client: YoutubeApi,
}

impl YoutubeClient {
    pub fn new(client: YoutubeApi) -> Self { Self { client } }
}


#[async_trait]
impl Youtube for YoutubeClient {
    fn new(key: &str) -> Self {
        Self {
            client: youtube_api::YoutubeApi::new(key),
        }
    }

    async fn query(&self, q: &str) -> Result<YoutubeResult,ApiError> {
        let request = SearchRequestBuilder {
            query: Some(q.into()),
            channel_id: None,
        };
        match self.client.search(request).await {
            Ok(_r) => {
                return Ok(YoutubeResult{});
            },
            Err(e) => {
                return Err(ApiError::SearchError{query: q.into(), source: e.into()});
            }

        }
    }
}
