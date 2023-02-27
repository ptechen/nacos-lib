use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use to_url::ToUrl;
use serde_json::Value;
use crate::client::REQ_CLIENT;
use crate::parse_url::ParseUrl;
use crate::result::Result;

const INSTANCE_LIST:&str = "/nacos/v1/ns/instance/list";

#[allow(non_snake_case)]
#[derive(Debug, Default, Clone, Deserialize, Serialize, ToUrl)]
pub struct GetInstanceList {
    /// 服务名
    pub serviceName: String,
    /// 分组名
    pub groupName: Option<String>,
    /// 命名空间ID
    pub namespaceId: Option<String>,
    /// 集群名称,多个集群用逗号分隔
    pub clusters: Option<String>,
    /// 是否只返回健康实例, 默认为false
    pub healthyOnly: Option<bool>
}

#[async_trait]
impl ParseUrl for GetInstanceList {}

#[allow(non_snake_case)]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Instance {
    pub hosts: Vec<Host>,
    pub dom: String,
    pub name: String,
    pub cacheMillis: i64,
    pub lastRefTime: i64,
    pub checksum: String,
    pub useSpecifiedURL: bool,
    pub clusters: String,
    pub env: String,
    pub metadata: Metadata,
}

#[allow(non_snake_case)]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Host {
    pub ip: String,
    pub port: i64,
    pub valid: bool,
    pub healthy: bool,
    pub marked: bool,
    pub instanceId: String,
    pub metadata: Metadata,
    pub enabled: bool,
    pub weight: f64,
    pub clusterName: String,
    pub serviceName: String,
    pub ephemeral: bool,
}

#[allow(non_snake_case)]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
}

impl GetInstanceList {
    pub async fn instance_list(&self) ->Result<Instance> {
        let url = self.parse_url(INSTANCE_LIST).await;
        let data: Instance  = REQ_CLIENT.get(url).send().await?.json().await?;
        Ok(data)
    }
}