fn main() -> Result<(), Box<dyn std::error::Error>> {
  tonic_build::compile_protos("../../proto/utils.proto")?;
  tonic_build::compile_protos("../../proto/item.proto")?;
  Ok(())
}
