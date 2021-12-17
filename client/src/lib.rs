use std::env;

use actix_web::web::Query;
use serde::Deserialize;
use tonic::Code;

use generated::hoge::GetHogeRequest;
use generated::sample_service::sample_service_client::SampleServiceClient;

mod generated;

#[derive(Deserialize)]
pub struct Info {
    id: String,
}

pub async fn index(info: Query<Info>) -> String {
    let id = &info.id.clone();
    let url = env::var("GRPC_SERVER_ADDRESS").expect("cannot get env");
    let mut client = SampleServiceClient::connect::<String>(url).await.unwrap();
    let result = client.get_hoge(GetHogeRequest { id: id.clone() }).await;
    match result {
        Ok(r) => {
            format!("id={} hoge is: {}", id, r.get_ref().name)
        }
        Err(status) => match status.code() {
            Code::NotFound => {
                log::debug!("hoge is not found: id={}", id);
                format!("hoge is notfound: id={}", id)
            }
            _ => {
                log::error!("get hoge error from grpc server. message:{}", &status);
                format!("get hoge error id={}, error={}", id, &status)
            }
        },
    }
}
