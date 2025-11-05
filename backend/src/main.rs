use backend::Cli;
use clap::Parser;
#[cfg(debug_assertions)]
use dotenv::dotenv;

#[tokio::main]
async fn main() {
  #[cfg(debug_assertions)]
  dotenv().ok();

  let cli = Cli::parse();
  cli.run().await;
}
