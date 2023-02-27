use std::collections::HashMap;
use std::sync::Arc;
use once_cell::sync::Lazy;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::time::sleep;
use std::sync::atomic::{AtomicUsize, Ordering};
use crate::client::CLIENT;
use crate::result::Error::CustomError;
use crate::result::Result;

pub type ServiceName = String;

pub static SERVICE_ADDRESSES: Lazy<Arc<RwLock<HashMap<ServiceName, (Vec<String>, AtomicUsize)>>>> = Lazy::new(||{
    tokio::spawn(get_list());
    Arc::new(RwLock::new(HashMap::new()))
});

async fn get_list() -> Result<()> {
    let flag = true;
    while flag {
        let services;
        let mut ins_list;
        {
            let read = CLIENT.read().await;
            services = read.get_service_list(1, 1000).service_list().await?;
            ins_list = read.get_instance_list("", true);
        }
        let mut map = HashMap::new();
        let mut flag = false;
        for service_name in services.doms {
            ins_list.serviceName = service_name.to_owned();
            let ins = ins_list.instance_list().await.expect("instance_list error");
            let mut hosts = vec![];
            for i in ins.hosts {
                let host = format!("{}:{}", i.ip, i.port);
                hosts.push(host);
            }
            if !flag {
                if let Some((data, _index)) = SERVICE_ADDRESSES.read().await.get(&service_name) {
                    if data.len() != hosts.len() {
                        flag = true;
                    } else {
                        for host in data {
                            if !hosts.contains(&host) {
                                flag = true;
                            }
                        }
                    }
                } else {
                    flag = true;
                }
            }
            map.insert(service_name, (hosts, AtomicUsize::new(0)));
        }
        if flag{
            let mut w = SERVICE_ADDRESSES.write().await;
            *w = map;
        }
        sleep(Duration::from_secs(CLIENT.read().await.client_beat_interval)).await;
    };
    Ok(())
}

pub async fn get_address(service_name: &str) -> Result<String> {
    if let Some((data, index)) = SERVICE_ADDRESSES.read().await.get(service_name) {
        let idx = index.load(Ordering::Relaxed);
        let length = data.len();
        if length > 1 {
            let cur_idx = idx + 1;
            if cur_idx < length - 1 {
                index.store(idx + 1, Ordering::Relaxed);
            } else {
                index.store(0, Ordering::Relaxed);
            }
        }
        let addr = data.get(idx).unwrap().to_string();
        return Ok(addr)
    }
    Err(CustomError("not found address".to_string()))
}