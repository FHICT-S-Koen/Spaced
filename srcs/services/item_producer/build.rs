fn main() -> Result<(), Box<dyn std::error::Error>> {
  tonic_build::compile_protos("../../proto/item.proto")?;
  tonic_build::compile_protos("../../proto/bounding.proto")?;
  Ok(())
}
