

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("src/bin/proto/helloworld.proto")?;
    tonic_build::compile_protos("src/bin/proto/job.proto")?;
    Ok(())
}