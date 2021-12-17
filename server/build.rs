fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .include_file("mod.rs")
        .build_client(false)
        .build_server(true)
        .out_dir("src/generated")
        .compile(&["../proto/service.proto"], &["../proto"])?;
    Ok(())
}
