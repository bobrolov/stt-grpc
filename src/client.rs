use crate::stt_model::recognition_client::RecognitionClient;
use crate::stt_model::RecognitionRequest;

mod stt_model {
    include!("../proto/stalin.rs");
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let address = std::env::var("SERVER_ADDRESS")?;

    let mut client = RecognitionClient::connect(format!("http://{}", address)).await?;

    let request = tonic::Request::new(RecognitionRequest { snippet: vec![] });
    let response = client.recognize(request).await?;

    println!("Works {}", response.into_inner().text);
    Ok(())
}
