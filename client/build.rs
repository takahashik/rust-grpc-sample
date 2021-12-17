fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .include_file("mod.rs")
        .build_client(true)
        .build_server(false)
        .out_dir("src/generated")
        .compile(&["../proto/service.proto"], &["../proto"])?;
    Ok(())
}
