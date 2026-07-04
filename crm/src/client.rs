use anyhow::Result;
use crm::pb::{CreateUserRequest, user_service_client::UserServiceClient};
use tonic::Request;

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = UserServiceClient::connect("http://[::1]:50051").await?;

    let req = Request::new(CreateUserRequest {
        name: "stargaoyc".to_string(),
        email: "stargaoyc@example.com".to_string(),
    });

    let resp = client.create_user(req).await?;
    println!("Create User: {:?}", resp.into_inner());
    Ok(())
}
