extern crate prost_build;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  prost_build::compile_protos(
    &["../../proto/item.proto", "../../proto/utils.proto"],
    &["../../proto"],
  )?;
  Ok(())
}
