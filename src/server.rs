mod model;

use crate::model::{new_model, Transcript};
use crate::stt_model::recognition_server::{Recognition, RecognitionServer};
use crate::stt_model::{RecognitionRequest, RecognitionResponse};
use deepspeech::{Model, Stream};
use std::sync::{Arc, Mutex};
use tonic::transport::Server;
use tonic::{Request, Response, Status};

mod stt_model {
    include!("../proto/stalin.rs");
}
struct GrpcServer {
    // deepspeech: Arc<Mutex<Model>>,
}

#[tonic::async_trait]
impl Recognition for GrpcServer {
    async fn recognize(
        &self,
        request: Request<RecognitionRequest>,
    ) -> Result<Response<RecognitionResponse>, Status> {
        let snippet = request.into_inner().snippet;
        let mut model = new_model().unwrap();
        let text = model.transcript(snippet).unwrap();
        let response = stt_model::RecognitionResponse { text };
        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let server_address = std::env::var("SERVER_ADDRESS")?.parse()?;
    // let mut model = new_model().unwrap();
    let grpc_server = GrpcServer {
         // deepspeech: Arc::new(Mutex::new(model)),
    };
    Server::builder()
        .add_service(RecognitionServer::new(grpc_server))
        .serve(server_address)
        .await?;
    Ok(())
}

#[cfg(test)]
mod tests {}
