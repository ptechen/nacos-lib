use serde::{Deserialize, Serialize};
use to_url::ToUrl;
use serde_json::Value;
use crate::client::{Client, CLIENT, REQ_CLIENT};
use crate::get_instance::GetInstance;
use crate::parse_url::ParseUrl;
use crate::result::Result;

const REGISTER: &str = "/nacos/v1/ns/instance";

/// 注册实例
#[allow(non_snake_case)]
#[derive(Debug, Default, Clone, Deserialize, Serialize, ToUrl)]
pub struct Register {
    /// 服务名
    pub serviceName: String,
    /// 服务实例IP
    pub ip: String,
    /// 服务实例port
    pub port: String,
    /// 命名空间ID
    pub namespaceId: Option<String>,
    /// 权重
    pub weight: Option<f64>,
    /// 是否上线
    pub enabled: Option<bool>,
    /// 是否健康
    pub healthy: Option<bool>,
    /// 扩展信息
    pub metadata: Option<String>,
    /// 集群名
    pub clusterName: Option<String>,
    /// 分组名
    pub groupName: Option<String>,
    /// 是否临时实例
    pub ephemeral: Option<bool>,
}

impl ParseUrl for Register{}

impl Register {
    pub async fn register(&self) -> Result<String> {
        let url = self.parse_url(REGISTER);
        let ok = REQ_CLIENT.post(url).send().await?.text().await?;
        let read = CLIENT.read();
        let ins = read.get_instance().get_instance().await?;
        let beat = read.get_beat(&ins.instanceId);
        beat.beat().await?;
        Ok(ok)
    }
}