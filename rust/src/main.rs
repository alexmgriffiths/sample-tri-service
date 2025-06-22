use tonic::{transport::Server, Request, Response, Status};

// Pull in the generated modules
use shared_protos::echo::echo_service_server::{EchoService, EchoServiceServer};
use shared_protos::echo::{EchoRequest, EchoResponse};

#[derive(Default)]
pub struct MyEcho;

#[tonic::async_trait]
impl EchoService for MyEcho {
    async fn echo(&self, req: Request<EchoRequest>) -> Result<Response<EchoResponse>, Status> {
        let msg = req.into_inner().message;
        Ok(Response::new(EchoResponse { message: msg }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50052".parse()?;
    Server::builder()
        .add_service(EchoServiceServer::new(MyEcho::default()))
        .serve(addr)
        .await?;
    Ok(())
}
