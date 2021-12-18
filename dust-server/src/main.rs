use std::error::Error;

use tonic::transport::Server;

use dust_protocol::MyGreeter;
use dust_protocol::proto::greeter_server::GreeterServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::default();

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
