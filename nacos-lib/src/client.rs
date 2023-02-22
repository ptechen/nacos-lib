use std::sync::Arc;
use std::time::Duration;
use once_cell::sync::Lazy;
use serde::Deserialize;
use parking_lot::RwLock;
use crate::beat::{Beat, BeatInfo};
use crate::get_instance::GetInstance;
use crate::register::Register;
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
    pub fn init(&self) {
        let mut client = CLIENT.write();
        *client = self.to_owned();
    }

    pub fn init_client(url: &str, client_beat_interval: u64){
        let mut client = CLIENT.write();
        client.url = url.trim().to_owned();
        client.client_beat_interval = client_beat_interval;
    }

    pub fn set_ip_port(ip: &str, port: &str) {
        let mut client = CLIENT.write();
        client.ip = Some(ip.trim().to_owned());
        client.port = Some(port.trim().to_owned());
    }

    pub fn set_service_name(service_name: &str) {
        let mut client = CLIENT.write();
        client.serviceName = service_name.to_owned();
    }

    pub fn get_service_list(&self, page_no: u64, page_size: u64) -> GetServiceList {
        GetServiceList {
            pageNo: page_no,
            pageSize: page_size,
            groupName: self.groupName.to_owned(),
            namespaceId: self.namespaceId.to_owned(),
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
}


