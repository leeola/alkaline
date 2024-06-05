mod log {
    use clap::Parser;
    use tracing::{level_filters::LevelFilter, subscriber};
    use tracing_subscriber::EnvFilter;

    #[derive(Parser, Debug, Default, Clone)]
    pub struct LogConfig {}
    impl LogConfig {
        pub fn init(&self) -> anyhow::Result<()> {
            subscriber::set_global_default(
                tracing_subscriber::FmtSubscriber::builder()
                    .with_env_filter(
                        EnvFilter::builder()
                            .with_default_directive(LevelFilter::INFO.into())
                            .with_env_var("RUST_LOG")
                            .from_env_lossy(),
                    )
                    .finish(),
            )?;
            Ok(())
        }
    }
}
pub mod config {
    use crate::log::LogConfig;
    use alkaline::{
        alkaline::{local::Local, Alkaline, Connection},
        storage::{memory::MemoryDb, DatabaseStorage},
    };
    use clap::Parser;

    #[derive(Parser, Debug, Default)]
    #[command(author, version, about, long_about = None)]
    pub struct Cli {
        #[command(flatten)]
        pub log: LogConfig,
        #[command(flatten)]
        pub storage: StorageConfig,
        pub expr: Option<String>,
    }
    impl Cli {
        pub fn alkaline(&self) -> Alkaline {
            let conn: Box<dyn Connection> = Box::new(Local::new(Box::<MemoryDb>::default()));
            Alkaline::new(conn)
        }
    }

    #[derive(Parser, Debug, Default, Clone)]
    pub struct StorageConfig {}
    impl StorageConfig {
        pub fn storage(&self) -> Box<dyn DatabaseStorage> {
            Box::<MemoryDb>::default()
        }
    }
}
