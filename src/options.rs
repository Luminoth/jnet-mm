use std::path::PathBuf;

use clap::Parser;

/// Website API
#[derive(Parser, Debug)]
pub struct Options {
    /// address to bind to
    #[arg(long, default_value = "0.0.0.0")]
    pub host: String,

    /// port to liten on
    #[arg(short, long, default_value_t = 8000)]
    pub port: u16,

    /// data prefix
    #[arg(default_value = ".")]
    prefix: PathBuf,

    /// set this when running in production
    #[arg(long, default_value_t = false)]
    pub prod: bool,

    /// enable tokio tracing
    #[arg(long, default_value_t = false)]
    pub tracing: bool,
}

impl Options {
    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
