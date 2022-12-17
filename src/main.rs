use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::env;
use string_builder::Builder;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = match env::var("PORT") {
        Ok(val) => val.parse().unwrap(),
        Err(_) => 8100,
    };

    println!("listen 0.0.0.0:{}", port);

    HttpServer::new(|| App::new().service(metrics))
        .bind(("0.0.0.0", port))?
        .run()
        .await
}

#[derive(Deserialize, Debug)]
struct MetricsQuery {
    target: String,
}

#[get("/metrics")]
async fn metrics(query: web::Query<MetricsQuery>) -> impl Responder {
    let box_name = match get_box_name(&query.target).await {
        Ok(x) => x,
        Err(_) => {
            return HttpResponse::BadRequest().body(format!("failed to connect {}", query.target))
        }
    };
    let hdd_info = match get_hdd_info(&query.target).await {
        Ok(x) => x,
        Err(_) => {
            return HttpResponse::BadRequest().body(format!("failed to connect {}", query.target))
        }
    };

    let mut str_builder = Builder::default();
    str_builder.append("# TYPE nasne_total_volume_size gauge\n");
    str_builder.append(format!(
        "nasne_total_volume_size{{name=\"{}\"}} {}\n\n",
        box_name.name.clone(),
        hdd_info.hdd.total_volume_size
    ));
    str_builder.append("# TYPE nasne_free_volume_size gauge\n");
    str_builder.append(format!(
        "nasne_free_volume_size{{name=\"{}\"}} {}\n\n",
        box_name.name.clone(),
        hdd_info.hdd.free_volume_size
    ));
    str_builder.append("# TYPE nasne_used_volume_size gauge\n");
    str_builder.append(format!(
        "nasne_used_volume_size{{name=\"{}\"}} {}\n\n",
        box_name.name.clone(),
        hdd_info.hdd.used_volume_size
    ));

    HttpResponse::Ok().body(str_builder.string().unwrap())
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
