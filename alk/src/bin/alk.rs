use alk::config::Cli;
use clap::Parser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    cli.log.init()?;
    let _alkaline = cli.alkaline();
    Ok(())
}
