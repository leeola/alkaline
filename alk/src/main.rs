use clap::Parser;
use tracing::{info, metadata::LevelFilter, subscriber};
use tracing_subscriber::EnvFilter;

#[derive(Parser, Debug, Default)]
#[command(author, version, about, long_about = None)]
pub struct CliConfig {}

#[tokio::main]
async fn main() {
    // TODO: Move logging init to a core utility, for ease of test setup.
    subscriber::set_global_default(
        tracing_subscriber::FmtSubscriber::builder()
            .with_env_filter(
                EnvFilter::builder()
                    .with_default_directive(LevelFilter::INFO.into())
                    .with_env_var("RUST_LOG")
                    .from_env_lossy(),
            )
            .finish(),
    )
    .unwrap();

    info!("Hello, world!");
}
