#![deny(warnings)]

mod options;

use std::net::SocketAddr;

use axum::{
    http::{HeaderValue, Method},
    routing::get,
    Router,
};
use clap::Parser;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing::{info, warn, Level};
use tracing_subscriber::FmtSubscriber;

use options::Options;

fn init_logging() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    Ok(())
}

// TODO: make the hosts in here configurable
fn init_cors(options: &Options) -> CorsLayer {
    let mut cors = CorsLayer::new()
        .allow_origin("http://somehost".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::OPTIONS, Method::HEAD, Method::GET])
        .allow_headers(Any);

    if !options.prod {
        warn!("Allowing localhost...");
        cors = cors.allow_origin("http://localhost:4200".parse::<HeaderValue>().unwrap());
    }

    cors
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let options = Options::parse();

    // TODO: make this not mutually exclusive
    if options.tracing {
        console_subscriber::init();
    } else {
        init_logging()?
    };

    let app = Router::new()
        .route("/", get(root))
        .layer(init_cors(&options))
        // TODO: don't log AWS health checks
        // TODO: resolve forwarding if TraceLayer doesn't already do it
        .layer(TraceLayer::new_for_http());

    let addr = options
        .address()
        .parse::<SocketAddr>()
        .unwrap_or_else(|_| panic!("Invalid address: {}", options.address()));
    info!("Listening on {}...", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn root() -> &'static str {
    "Hello, World!"
}
