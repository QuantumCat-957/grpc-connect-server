fn main() -> Result<(),Box<dyn std::error::Error>>{
    tonic_build::compile_protos("./proto/grpc_connect.proto")?;
    tonic_build::compile_protos("./proto/hmessage_relation.proto")?;
    tonic_build::compile_protos("./proto/hmessage_core.proto")?;
    tonic_build::compile_protos("./proto/hmessage_message.proto")?;
    Ok(())
}