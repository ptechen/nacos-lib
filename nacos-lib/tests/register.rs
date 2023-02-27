use std::time::Duration;
use tokio::time::sleep;
use nacos_lib::client::{Client, CLIENT};
use nacos_lib::register::Register;
use nacos_lib::result::Result;
use nacos_lib::service_address::{get_address, SERVICE_ADDRESSES};
use nacos_lib::unregister::UnRegister;
// use nacos_lib::service_address::init;

#[tokio::test]
async fn test() -> Result<()> {
    Client::default()
        .init_client("http://1.13.3.254:8848", 5)
        .set_ip_port("127.0.0.1", "8080")
        .set_service_name("test")
        .set_space_name("test")
        .build().await;

    // println!("{:?}", CLIENT.read().await);
    let ok = CLIENT.read().await.get_register().register().await?;
    println!("{}", ok);
    // // init().await;
    // CLIENT.read().await.get_instance_list()
    println!("{:?}", SERVICE_ADDRESSES.read().await);
    // sleep(Duration::from_secs(15)).await;
    // // let data = CLIENT.read().await.get_instance().get_instance().await?;
    // // println!("{:?}", data);
    //
    // // let ok = CLIENT.read().await.get_unregister().unregister().await?;
    // // println!("{}", ok);
    //
    // println!("{}", get_address("test").await);
    Ok(())
}