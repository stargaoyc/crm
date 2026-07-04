use anyhow::Result;
use crm::pb::{
    CreateUserRequest, GetUserRequest, User,
    user_service_server::{UserService, UserServiceServer},
};
use tonic::{Request, Response, Status, async_trait, transport::Server};

#[derive(Default)]
pub struct UserServer {}

#[async_trait]
impl UserService for UserServer {
    async fn create_user(&self, req: Request<CreateUserRequest>) -> Result<Response<User>, Status> {
        let input = req.into_inner();
        println!("Create User: {:?}", input);
        let user = User::new(1, &input.name, &input.email);
        Ok(Response::new(user))
    }
    async fn get_user(&self, req: Request<GetUserRequest>) -> Result<Response<User>, Status> {
        let input = req.into_inner();
        println!("Get User: {:?}", input);
        Ok(Response::new(User::default()))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "[::1]:50051".parse()?;
    let server = UserServer::default();

    println!("Server is running on {}", addr);

    Server::builder()
        .add_service(UserServiceServer::new(server))
        .serve(addr)
        .await?;

    Ok(())
}
