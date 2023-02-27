use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use to_url::ToUrl;
use crate::parse_url::ParseUrl;
use crate::result::Result;
use serde_json::Value;
use crate::client::REQ_CLIENT;

const SERVICE_LIST: &str = "/nacos/v1/ns/service/list";

/// 查询服务列表
#[allow(non_snake_case)]
#[derive(Debug, Default, Clone, Deserialize, Serialize, ToUrl)]
pub struct GetServiceList {
    /// 当前页码
    pub pageNo: u64,
    /// 分页大小
    pub pageSize: u64,
    /// 分组名
    pub groupName: Option<String>,
    /// 命名空间ID
    pub namespaceId: Option<String>
}

#[async_trait]
impl ParseUrl for GetServiceList {}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Services {
    pub count: i64,
    pub doms: Vec<String>,
}

impl GetServiceList {
    pub async fn service_list(&self) -> Result<Services> {
        let url = self.parse_url(SERVICE_LIST).await;
        let data:Services = REQ_CLIENT.get(&url).send().await?.json().await?;
        Ok(data)
    }
}