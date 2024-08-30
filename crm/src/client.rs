use anyhow::Result;
use crm::pb::{user_service_client::UserServiceClient, CreateUserRequest};
use tonic::Request;

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = UserServiceClient::connect("http://[::1]:50051").await?;

    let request = Request::new(CreateUserRequest {
        name: "Tonic".into(),
        email: "tonic@example.com".into(),
    });

    let user = client.create_user(request).await?.into_inner();

    println!("RESPONSE={:?}", user);

    Ok(())
}
