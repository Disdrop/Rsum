use std::fs;
use std::io::Result;
use std::path::Path;

pub fn init_repo() -> Result<()> {
  let rsum_dir = Path::new(".rsum");

  if !rsum_dir.exists() {
    fs::create_dir(rsum_dir)?;
    println!("Initialized empty rsum repository in .rsum/");
  } else {
    println!("The repository already exists.");
  }

  Ok(())
}
