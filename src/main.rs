macro_rules! skip_fail {
    ($res:expr) => {
        match $res {
            Ok(val) => val,
            Err(e) => {
                println!("An error: {}; skipped.", e);
                continue;
            }
        }
    };
}

use anyhow::Result;
use metrics::gauge;
use metrics_exporter_prometheus::PrometheusBuilder;
use metrics_util::MetricKindMask;
use serde::{Deserialize, Serialize};
use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::Duration;

#[tokio::main]
async fn main() {
    let device_ip_addrs: Vec<String> = env::var("DEVICE_IP_ADDRS")
        .expect("require DEVICE_IP_ADDRS")
        .split(",")
        .map(|s| s.to_string())
        .collect();

    if device_ip_addrs.len() == 0 {
        panic!("require DEVICE_IP_ADDRS");
    }

    let builder = PrometheusBuilder::new();
    builder
        .idle_timeout(
            MetricKindMask::COUNTER | MetricKindMask::HISTOGRAM,
            Some(Duration::from_secs(10)),
        )
        .with_http_listener(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 8100))
        .install()
        .expect("failed to install Prometheus recorder");

    println!("start server 0.0.0.0:8100");

    loop {
        for ip_addr in &device_ip_addrs {
            let box_name = skip_fail!(get_box_name(&ip_addr).await);
            let hdd_info = skip_fail!(get_hdd_info(&ip_addr).await);

            gauge!(
                "nasne_total_volume_size",
                hdd_info.hdd.total_volume_size as f64,
                "name" => box_name.name.clone(),
            );
            gauge!(
                "nasne_free_volume_size",
                hdd_info.hdd.free_volume_size as f64,
                "name" => box_name.name.clone(),
            );
            gauge!(
                "nasne_used_volume_size",
                hdd_info.hdd.used_volume_size as f64,
                "name" => box_name.name.clone(),
            );
        }

        std::thread::sleep(Duration::from_secs(10));
    }
}

async fn do_get(ip_addr: &str, endpoint: &str) -> Result<String> {
    Ok(
        reqwest::get(format!("http://{}:64210{}", ip_addr, endpoint))
            .await?
            .text()
            .await?,
    )
}

#[derive(Serialize, Deserialize, Debug)]
struct BoxName {
    #[serde(rename = "errorcode")]
    error_code: u32,
    name: String,
}

async fn get_box_name(ip_addr: &str) -> Result<BoxName> {
    let result = do_get(ip_addr, "/status/boxNameGet").await?;
    let box_name: BoxName = serde_json::from_str(&result)?;

    Ok(box_name)
}

#[derive(Serialize, Deserialize, Debug)]
struct HDD {
    #[serde(rename = "totalVolumeSize")]
    total_volume_size: u64,
    #[serde(rename = "freeVolumeSize")]
    free_volume_size: u64,
    #[serde(rename = "usedVolumeSize")]
    used_volume_size: u64,
    #[serde(rename = "serialNumber")]
    serial_number: String,
    #[serde(rename = "id")]
    id: u32,
    #[serde(rename = "internalFlag")]
    internal_flag: u32,
    #[serde(rename = "mountStatus")]
    mount_status: u32,
    #[serde(rename = "registerFlag")]
    register_flag: u32,
    #[serde(rename = "format")]
    format: String,
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "vendorID")]
    vendor_id: String,
    #[serde(rename = "productID")]
    product_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct HDDInfo {
    #[serde(rename = "errorcode")]
    error_code: u32,
    #[serde(rename = "HDD")]
    hdd: HDD,
}

async fn get_hdd_info(ip_addr: &str) -> Result<HDDInfo> {
    let result = do_get(ip_addr, "/status/HDDInfoGet?id=0").await?;
    let hdd_info: HDDInfo = serde_json::from_str(&result)?;

    Ok(hdd_info)
}
