use anyhow::Result;
use crm::pb::{
    user_service_server::{UserService, UserServiceServer},
    CreateUserRequest, GetUsersRequest, User,
};
use tonic::{async_trait, transport::Server, Request, Response, Status};

#[derive(Default)]
pub struct UserServer {}

#[async_trait]
impl UserService for UserServer {
    async fn get_users(&self, request: Request<GetUsersRequest>) -> Result<Response<User>, Status> {
        let input = request.into_inner();
        println!("get user: {:?}", input);
        Ok(Response::new(User::default()))
    }

    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<User>, Status> {
        let input = request.into_inner();
        println!("create user: {:?}", input);
        let user = User::new(1, &input.name, &input.email);
        Ok(Response::new(user))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "[::1]:50051".parse().unwrap();
    let svc = UserServer::default();

    println!("UserService listening on {}", addr);

    Server::builder()
        .add_service(UserServiceServer::new(svc))
        .serve(addr)
        .await?;

    Ok(())
}
