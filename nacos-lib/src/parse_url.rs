use to_url::ToUrl;
use crate::client::CLIENT;

pub trait ParseUrl: ToUrl {
    fn parse_url(&self, uri: &str) -> String {
        let params = self.to_url();
        format!("{}{}?{}", CLIENT.read().url, uri, params)
    }
}