use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use to_url::ToUrl;
use crate::client::REQ_CLIENT;
use crate::parse_url::ParseUrl;
use crate::result::Result;

const GET_CONFIG:&str = "/nacos/v1/cs/config";

#[allow(non_snake_case)]
#[derive(Debug, Default, Clone, Deserialize, Serialize, ToUrl)]
pub struct GetConfig {
    /// 命名空间ID
    pub namespaceId: Option<String>,
    /// 配置分组名
    pub group: String,
    /// 配置名
    pub dataId: String,
    /// 集群名称
    pub tag: Option<String>,
}

#[async_trait]
impl ParseUrl for GetConfig {}

impl GetConfig {
    pub async fn get_config(&self) -> Result<String> {
        let url = self.parse_url(GET_CONFIG).await;
        let data = REQ_CLIENT.get(url).send().await?.text().await?;
        Ok(data)
    }

    pub fn set_data_id(&mut self, data_id: &str) -> Self {
        self.dataId = data_id.to_string();
        self.clone()
    }

    pub fn set_tag(&mut self, tag: &str) -> Self {
        self.tag = Some(tag.to_string());
        self.clone()
    }

    pub fn set_group(&mut self, group: &str) -> Self {
        self.group = group.to_string();
        self.clone()
    }
}