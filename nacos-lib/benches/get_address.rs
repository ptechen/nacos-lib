use tokio::time::{sleep, Duration};
use criterion::{criterion_group, criterion_main, Criterion};
use chrono::Utc;
use nacos_lib::client::{Client, CLIENT};
use nacos_lib::service_address::{get_address, SERVICE_ADDRESSES};
use nacos_lib::result::Result;

fn get_address_benchmark(c: &mut Criterion) {
    tokio::runtime::Runtime::new().unwrap().block_on(tt()).unwrap();
    c.bench_function("get_address", |b| {
        b.iter(|| {
            futures::executor::block_on(get_address("test")).unwrap();
        })
    });
}

async fn tt() -> Result<()> {
    Client::default().init_client("http://1.13.3.254:8848", 5)
        .set_ip_port("127.0.0.1", "8080")
        .set_service_name("test")
        .set_space_name("test")
        .build().await;
    // println!("{:?}", CLIENT.read().await);
    CLIENT.read().await.get_register().register().await.unwrap();
    // SERVICE_ADDRESSES.read().await;
    sleep(Duration::from_secs(5)).await;
    // let start = Utc::now().timestamp_millis();
    Ok(())
}

criterion_group!(benches, get_address_benchmark);
criterion_main!(benches);

