use serde::Deserialize;
use serde_json::Value;
use to_url::ToUrl;
use crate::client::{Client, CLIENT, REQ_CLIENT};
use crate::parse_url::ParseUrl;
use crate::result::Result;

const UNREGISTER: &str = "/nacos/v1/ns/instance";

/// 注销实例
#[allow(non_snake_case)]
#[derive(Debug, Default, Clone, Deserialize, ToUrl)]
pub struct UnRegister {
    /// 服务实例IP
    pub ip: String,
    /// 服务实例port
    pub port: String,
    /// 命名空间ID
    pub namespaceId: Option<String>,
    /// 集群名
    pub clusterName: Option<String>,
    /// 服务名
    pub serviceName: String,
    /// 分组名
    pub groupName: Option<String>,
    /// 是否临时实例
    pub ephemeral: Option<bool>,
}

impl ParseUrl for UnRegister{}

impl UnRegister {
    pub async fn unregister(&self) -> Result<String> {
        let url = self.parse_url(UNREGISTER);
        let ok = REQ_CLIENT.delete(url).send().await?.text().await?;
        Ok(ok)
    }
}