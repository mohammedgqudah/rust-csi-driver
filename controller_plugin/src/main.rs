use tonic::transport::Server;

use controller::{ControllerPlugin, ControllerServer};
use identity::{ControllerIdentity, IdentityServer};

mod controller;
mod identity;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let addr = "[::1]:50051".parse().unwrap();
    let identity = ControllerIdentity::default();
    let controller = ControllerPlugin::default();

    println!("Controller listening on {}", addr);

    Server::builder()
        .add_service(IdentityServer::new(identity))
        .add_service(ControllerServer::new(controller))
        .serve(addr)
        .await?;

    Ok(())
}
