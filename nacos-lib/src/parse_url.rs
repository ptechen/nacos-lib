use to_url::ToUrl;
use async_trait::async_trait;
use crate::client::CLIENT;

#[async_trait]
pub trait ParseUrl: ToUrl {
    async fn parse_url(&self, uri: &str) -> String {
        let params = self.to_url();
        format!("{}{}?{}", CLIENT.read().await.url, uri, params)
    }
}