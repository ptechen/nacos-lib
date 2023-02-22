use serde::{Deserialize, Serialize};
use to_url::ToUrl;
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

impl ParseUrl for GetInstanceList {}

#[allow(non_snake_case)]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Instance {
    pub name: String,
    pub groupName: String,
    pub clusters: String,
    pub cacheMillis: i64,
    pub hosts: Vec<Host>,
    pub lastRefTime: i64,
    pub checksum: String,
    pub allIPs: bool,
    pub reachProtectionThreshold: bool,
    pub valid: bool,
}

#[allow(non_snake_case)]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Host {
    pub instanceId: String,
    pub ip: String,
    pub port: i64,
    pub weight: i64,
    pub healthy: bool,
    pub enabled: bool,
    pub ephemeral: bool,
    pub clusterName: String,
    pub serviceName: String,
    pub metadata: Metadata,
    pub instanceHeartBeatInterval: i64,
    pub instanceIdGenerator: String,
    pub instanceHeartBeatTimeOut: i64,
    pub ipDeleteTimeout: i64,
}

#[allow(non_snake_case)]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
}

impl GetInstanceList {
    pub async fn instance_list(&self) ->Result<Instance> {
        let url = self.parse_url(INSTANCE_LIST);
        let data: Instance = REQ_CLIENT.get(url).send().await?.json().await?;
        Ok(data)
    }
}