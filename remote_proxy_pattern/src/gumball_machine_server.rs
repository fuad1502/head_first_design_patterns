use tonic::transport::Server;

use gumball_machine_service::gumball_machine_service_server::GumballMachineServiceServer;
use remote_proxy_pattern::gumball_machine_service;
use remote_proxy_pattern::gumball_machine_skeleton::GumballMachineSkeleton;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let service =
        GumballMachineServiceServer::new(GumballMachineSkeleton::new(0, "Casa 39".to_string()));

    println!("Gumball Machine Server listening on: {}", addr);

    Server::builder().add_service(service).serve(addr).await?;

    Ok(())
}
