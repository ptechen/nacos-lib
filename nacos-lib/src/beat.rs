use std::time::Duration;
use serde_json::Value;
use chrono::Utc;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use tokio::time::sleep;
use crate::client::{CLIENT, REQ_CLIENT};
use crate::result::Result;
use std::str::FromStr;
use from_value_derive::From;
use serde_json::Map;
use to_url::ToUrl;
use crate::parse_url::ParseUrl;

const BEAT: &str = "/nacos/v1/ns/instance/beat";

/// 发送实例心跳
#[allow(non_snake_case)]
#[derive(Debug, Default, Clone, Deserialize, Serialize, ToUrl)]
pub struct Beat{
    /// 服务名
    pub serviceName: String,
    /// 服务实例IP
    pub ip: String,
    /// 服务实例port
    pub port: String,
    /// 命名空间ID
    pub namespaceId: Option<String>,
    /// 分组名
    pub groupName: Option<String>,
    /// 是否临时实例
    pub ephemeral: Option<bool>,
    /// 实例心跳内容
    pub beat: Option<BeatInfo>,
}

impl ParseUrl for Beat {}

impl Beat {
    pub async fn beat(&self) -> Result<String> {
        let url = self.parse_url(BEAT);
        tokio::spawn(Self::send_beat(url));
        Ok("ok".to_string())
    }

    async fn send_beat(url: String) -> Result<()>{
        let interval;
        {
            interval = CLIENT.read().client_beat_interval;
        };
        loop {
            sleep(Duration::from_secs(interval)).await;
            let _ok:BeatRes = REQ_CLIENT.put(&url).send().await?.json().await?;
        }
    }
}


#[allow(non_snake_case)]
#[derive(Debug, Default, Clone, Deserialize, Serialize, From)]
pub struct BeatInfo{
    /// 服务名
    pub serviceName: String,
    /// 服务实例IP
    pub ip: String,
    /// 服务实例port
    pub port: String,
    /// 命名空间ID
    pub namespaceId: Option<String>,
    /// 集群名
    pub cluster: Option<String>,
    /// 权重
    pub weight: Option<f64>,
    /// 扩展信息
    pub metadata: Option<String>,
    pub period: u64,
    pub scheduled: bool,
    pub instanceId: String,
}

#[allow(non_snake_case)]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct BeatRes {
    pub clientBeatInterval: i64,
    pub code: i64,
    pub lightBeatEnabled: bool,
}
