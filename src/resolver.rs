use crate::youtube;
use config::{
    Config,
    File,
};
pub struct Resolver {
    config: Config,
    // These probably could be more generic types but I'm not worried about that right now.
    http_client: reqwest::Client,
    youtube_client: youtube::YoutubeClient,
}

impl Resolver {
    pub fn youtube(&self) -> &impl youtube::Youtube {
        &self.youtube_client
    }
}

impl Default for Resolver {
    fn default() -> Self {
        let mut c = Config::new();
        c.merge(File::with_name("config/default.toml")).unwrap();
        // Add in a local configuration file
        // This file shouldn't be checked in to git
        c.merge(File::with_name("config/local.toml").required(false))
            .unwrap();
        let youtube_key = c.get_str("youtube.key").unwrap();
        Resolver {
            config: c,
            http_client: reqwest::Client::default(),
            youtube_client: youtube::YoutubeClient::new(youtube_api::YoutubeApi::new(youtube_key)),
        }
    }
}
