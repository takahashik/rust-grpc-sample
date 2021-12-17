extern crate sample_server;

use dotenv::dotenv;
use std::env;
use tonic::transport::Server;

use sample_server::create_server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    env_logger::init();
    let addr = env::var("GRPC_SERVER_ADDRESS")
        .expect("Can't get service address")
        .parse()
        .unwrap();
    log::info!("server listening on: {}", addr);

    let svc = create_server();

    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}
