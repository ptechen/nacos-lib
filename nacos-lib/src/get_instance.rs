use serde::{Deserialize, Serialize};
use serde_json::Value;
use to_url::ToUrl;
use crate::client::{CLIENT, REQ_CLIENT};
use crate::parse_url::ParseUrl;
use crate::result::Result;

const GET_INSTANCE:&str = "/nacos/v1/ns/instance";

#[allow(non_snake_case)]
#[derive(Debug, Default, Clone, Deserialize, Serialize, ToUrl)]
pub struct GetInstance{
    /// 服务实例IP
    pub ip: String,
    /// 服务实例port
    pub port: String,
    /// 命名空间ID
    pub namespaceId: Option<String>,
    /// 服务名
    pub serviceName: String,
    /// 分组名
    pub groupName: Option<String>,
    /// 集群名称
    pub cluster: Option<String>,
    /// 是否健康
    pub healthyOnly: bool,
    /// 是否临时实例
    pub ephemeral: Option<bool>,
}

impl ParseUrl for GetInstance {}

#[allow(non_snake_case)]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Instance {
    pub metadata: Metadata,
    pub instanceId: String,
    pub port: i64,
    pub service: String,
    pub healthy: bool,
    pub ip: String,
    pub clusterName: String,
    pub weight: f64,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {}

impl GetInstance {
    pub async fn get_instance(&self) -> Result<Instance> {
        let url = self.parse_url(GET_INSTANCE);
        let data: Instance = REQ_CLIENT.get(url).send().await?.json().await?;
        Ok(data)
    }
}