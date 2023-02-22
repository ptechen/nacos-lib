use std::time::Duration;
use tokio::time::sleep;
use nacos_lib::client::{Client, CLIENT};
use nacos_lib::register::Register;
use nacos_lib::result::Result;
use nacos_lib::unregister::UnRegister;

#[tokio::test]
async fn test() -> Result<()> {
    Client::init_client("http://1.13.3.254:8848", 5);
    Client::set_ip_port( "127.0.0.1", "8080");
    Client::set_service_name("test");
    let ok = CLIENT.read().get_register().register().await?;
    println!("{}", ok);

    // sleep(Duration::from_secs(15)).await;
    //
    // let data = reg.get_instance().get_instance().await?;
    // println!("{:?}", data);
    //
    // let ok = reg.get_unregister().unregister().await?;
    // println!("{}", ok);
    Ok(())
}