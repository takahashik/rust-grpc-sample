extern crate sample_client;

use std::env;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;

use sample_client::index;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let address = env::var("SERVICE_ADDRESS").expect("Can't get service address");
    log::info!("server listening on: {}", address);

    HttpServer::new(move || App::new().service(web::resource("/").to(index)))
        .bind(address)?
        .run()
        .await
}
