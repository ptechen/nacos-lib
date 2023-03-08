use std::sync::Arc;
use std::time::Duration;
use once_cell::sync::Lazy;
use serde::Deserialize;
use tokio::sync::RwLock;
use crate::beat::{Beat, BeatInfo};
use crate::get_config::GetConfig;
use crate::get_instance::GetInstance;
use crate::instance_list::GetInstanceList;
use crate::register::Register;
use crate::service_address::SERVICE_ADDRESSES;
use crate::service_list::GetServiceList;
use crate::unregister::UnRegister;

pub static CLIENT: Lazy<Arc<RwLock<Client>>> = Lazy::new(|| {
    Arc::new(RwLock::new(Client::default()))
});


pub static REQ_CLIENT: Lazy<Arc<reqwest::Client>> = Lazy::new(|| {
    Arc::new(reqwest::Client::builder().connect_timeout(Duration::from_secs(5)).build().unwrap())
});

#[allow(non_snake_case)]
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Client {
    /// nacos链接
    pub url: String,
    /// 心跳间隔 默认5s
    pub client_beat_interval: u64,
    /// 服务名
    pub serviceName: String,
    /// 服务实例IP
    pub ip: Option<String>,
    /// 服务实例port
    pub port: Option<String>,
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

/// example:
/// Client::init_client("http://127.0.0.1:8848", 5)

impl Client {
    pub async fn build(&self) {
        let mut client = CLIENT.write().await;
        *client = self.to_owned();
    }

    pub async fn build_listen(&self) {
        let mut client = CLIENT.write().await;
        *client = self.to_owned();
        let _ = SERVICE_ADDRESSES.read().await;
    }

    pub fn init_client(mut self, url: &str, client_beat_interval: u64) -> Self{
        self.url = url.trim().to_string();
        self.client_beat_interval = client_beat_interval;
        self
    }

    pub fn set_ip_port(mut self, ip: &str, port: &str) -> Self {
        self.ip = Some(ip.trim().to_owned());
        self.port = Some(port.trim().to_owned());
        self
    }

    pub fn set_service_name(mut self, service_name: &str) -> Self {
        self.serviceName = service_name.to_owned();
        self
    }

    pub fn set_group_name(mut self, group_name: &str) -> Self {
        self.groupName = Some(group_name.to_owned());
        self
    }

    pub fn set_space_name(mut self, space_name: &str) -> Self {
        self.namespaceId = Some(space_name.to_owned());
        self
    }

    pub fn get_service_list(&self, page_no: u64, page_size: u64) -> GetServiceList {
        GetServiceList {
            pageNo: page_no,
            pageSize: page_size,
            groupName: self.groupName.to_owned(),
            namespaceId: self.namespaceId.to_owned(),
        }
    }

    pub fn get_instance_list(&self, service_name: &str, healthy_only: bool) -> GetInstanceList {
        GetInstanceList{
            serviceName: service_name.to_owned(),
            groupName: self.groupName.to_owned(),
            namespaceId: self.namespaceId.to_owned(),
            clusters: self.clusterName.to_owned(),
            healthyOnly: Some(healthy_only),
        }
    }

    pub fn get_beat_info(&self, ins_id: &str) -> BeatInfo {
        BeatInfo{
            serviceName: self.serviceName.to_owned(),
            ip: self.ip.to_owned().unwrap(),
            port: self.port.to_owned().unwrap(),
            namespaceId: Some(self.namespaceId.to_owned().unwrap_or(String::new())),
            cluster: Some(self.clusterName.to_owned().unwrap_or(String::new())),
            weight: Some(self.weight.unwrap_or(1.0)),
            metadata: self.metadata.to_owned(),
            period: 5000,
            scheduled: true,
            instanceId: ins_id.to_string(),
        }
    }

    pub fn get_beat(&self, ins_id: &str) -> Beat {
        Beat {
            serviceName: self.serviceName.to_owned(),
            ip: self.ip.to_owned().unwrap(),
            port: self.port.to_owned().unwrap(),
            namespaceId: self.namespaceId.to_owned(),
            groupName: self.groupName.to_owned(),
            ephemeral: self.ephemeral,
            beat: Some(self.get_beat_info(ins_id)),
        }
    }


    pub fn get_instance(&self) -> GetInstance {
        GetInstance{
            ip: self.ip.to_owned().unwrap(),
            port: self.port.to_owned().unwrap(),
            namespaceId: self.namespaceId.to_owned(),
            serviceName: self.serviceName.to_owned(),
            groupName: self.groupName.to_owned(),
            cluster: self.clusterName.to_owned(),
            healthyOnly: false,
            ephemeral: self.ephemeral,
        }
    }

    pub fn get_unregister(&self) -> UnRegister {
        UnRegister{
            ip: self.ip.to_owned().unwrap(),
            port: self.port.to_owned().unwrap(),
            namespaceId: self.namespaceId.to_owned(),
            clusterName: self.clusterName.to_owned(),
            serviceName: self.serviceName.to_owned(),
            groupName: self.groupName.to_owned(),
            ephemeral: self.ephemeral,
        }
    }

    pub fn get_register(&self) -> Register {
        Register{
            ip: self.ip.to_owned().unwrap(),
            port: self.port.to_owned().unwrap(),
            namespaceId: self.namespaceId.to_owned(),
            weight: self.weight.to_owned(),
            enabled: self.enabled.to_owned(),
            healthy: self.healthy,
            metadata: self.metadata.to_owned(),
            clusterName: self.clusterName.to_owned(),
            serviceName: self.serviceName.to_owned(),
            groupName: self.groupName.to_owned(),
            ephemeral: self.ephemeral,
        }
    }

    pub fn get_config(&self) -> GetConfig {
        GetConfig{
            namespaceId: self.namespaceId.to_owned(),
            group: self.groupName.to_owned().unwrap_or_default(),
            dataId: "".to_string(),
            tag: self.clusterName.to_owned(),
        }
    }
}


