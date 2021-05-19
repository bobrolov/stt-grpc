use crate::stt_model::recognition_client::RecognitionClient;
use crate::stt_model::RecognitionRequest;
use tokio::io::AsyncReadExt;

mod stt_model {
    include!("../proto/stalin.rs");
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let address = std::env::var("SERVER_ADDRESS")?;

    let mut snippet = Vec::new();
    let mut client = RecognitionClient::connect(format!("http://{}", address)).await?;
    let mut path = std::env::var("WAV_FILE_PATH").unwrap();
    let mut file = tokio::fs::File::open(path.as_str()).await?;
    file.read_to_end(&mut snippet).await?;

    let request = tonic::Request::new(RecognitionRequest { snippet });
    let response = client.recognize(request).await?;

    println!("Inference results: \"{}\"", response.into_inner().text);
    Ok(())
}
