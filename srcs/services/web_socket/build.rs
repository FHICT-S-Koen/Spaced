extern crate prost_build;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  prost_build::compile_protos(
    &["../../proto/item.proto", "../../proto/utils.proto"],
    &["../../proto"],
  )?;
  if std::env::var_os("DOCS_RS").is_some() {
    println!("cargo:rustc-env=SQLX_OFFLINE=true");
  }
  Ok(())
}
