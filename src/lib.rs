pub(crate) mod core;

pub fn init() {
  match core::init::init_repo() {
    Ok(_) => println!("Repository initialized successfully."),
    Err(e) => eprintln!("Failed to initialize repository: {}", e),
  }
}
