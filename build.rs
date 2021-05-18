fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .out_dir("./proto")
        .compile(
            &["../rasr_worker/batch_recognition.proto"],
            &["../rasr_worker"],
        )?;
    Ok(())
}
//proto