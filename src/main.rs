use rustway::run;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let config_path = PathBuf::from("gateway.yaml");
    run(config_path).await
}
