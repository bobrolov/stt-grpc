mod model;

use crate::model::{New, Transcript};
use crate::stt_model::recognition_server::{Recognition, RecognitionServer};
use crate::stt_model::{RecognitionRequest, RecognitionResponse};
use deepspeech::Model;
use std::sync::Mutex;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

mod stt_model {
    include!("../proto/stalin.rs");
}
struct GrpcServer {
    deepspeech: Model,
}

#[tonic::async_trait]
impl Recognition for GrpcServer {
    async fn recognize(
        &mut self,
        request: Request<RecognitionRequest>,
    ) -> Result<Response<RecognitionResponse>, Status> {
        let mutex = Mutex::new(self);

        let mut text = String::new();
        tokio::spawn(async move {
            let _guard = mutex.lock().await;
            async {
                text = self
                    .deepspeech
                    .transcript(request.into_inner().snippet)
                    .unwrap();
            }
            .await;
        });

        let response = stt_model::RecognitionResponse { text };
        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let server_address = std::env::var("SERVER_ADDRESS")?.parse()?;
    let grpc_server = GrpcServer {
        deepspeech: Model::new()?,
    };
    Server::builder()
        .add_service(RecognitionServer::new(grpc_server))
        .serve(server_address)
        .await?;
    Ok(())
}

#[cfg(test)]
mod tests {}
