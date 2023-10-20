fn main() -> Result<(), Box<dyn std::error::Error>> {
  tonic_build::compile_protos("../proto/item.proto")?;
  tauri_build::build();
  Ok(())
}
