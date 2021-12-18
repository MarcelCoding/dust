use tonic::Request;

use dust_protocol::proto::greeter_client::GreeterClient;
use dust_protocol::proto::HelloRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    let request = Request::new(HelloRequest { name: "Tonic".into() });

    let response = client.say_hello(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
