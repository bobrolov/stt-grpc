use crate::stt_model::recognition_server::{Recognition, RecognitionServer};
use crate::stt_model::{RecognitionRequest, RecognitionResponse};
use tonic::transport::Server;
use tonic::{Request, Response, Status};

mod stt_model {
    include!("../proto/stalin.rs");
}
struct GrpcServer {}

#[tonic::async_trait]
impl Recognition for GrpcServer {
    async fn recognize(
        &self,
        request: Request<RecognitionRequest>,
    ) -> Result<Response<RecognitionResponse>, Status> {
        let response = stt_model::RecognitionResponse{
            text: "Ok".to_string()
        };
        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let server_address = std::env::var("SERVER_ADDRESS")?.parse()?;
    let grpc_server = GrpcServer{};
    Server::builder()
        .add_service(RecognitionServer::new(grpc_server))
        .serve(server_address)
        .await?;
    Ok(())
}

#[cfg(test)]
mod tests {}
