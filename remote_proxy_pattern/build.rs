fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/gumball_machine_service.proto")?;
    Ok(())
}
