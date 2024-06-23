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
        backend::{ephemeral_reference::EphemeralReferenceBackend, Connection},
        client::Alkaline,
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
        pub fn connection(&self) -> Box<dyn Connection> {
            // TODO: config for persist vs ephem impl
            Box::new(EphemeralReferenceBackend::new())
        }
        pub fn alkaline(&self) -> Alkaline {
            let conn = self.connection();
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
