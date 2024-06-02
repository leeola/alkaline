use alk::config::Cli;
use alkaline::alkaline::Alkaline;
use clap::Parser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    cli.log.init()?;
    let storage = cli.storage.storage();
    let _alkaline = Alkaline::new(storage);
    Ok(())
}
