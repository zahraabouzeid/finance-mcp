use std::io::{self, IsTerminal};

use anyhow::Context;
use clap::Parser;
use finance_mcp::{
    config::Args,
    server::{self, MCP_PATH},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    init_logging();

    let listener = server::bind(args.port)
        .await
        .with_context(|| format!("failed to bind 127.0.0.1:{}", args.port))?;
    let address = listener.local_addr()?;
    tracing::info!("listening on http://{address}{MCP_PATH}");

    server::serve(listener, shutdown_signal())
        .await
        .context("server stopped with an error")
}

fn init_logging() {
    tracing_subscriber::fmt()
        .with_writer(io::stderr)
        .with_ansi(io::stderr().is_terminal())
        .init();
}

async fn shutdown_signal() {
    if let Err(error) = tokio::signal::ctrl_c().await {
        tracing::error!("cannot listen for Ctrl+C: {error}");
        std::future::pending::<()>().await;
    }
    tracing::info!("shutting down");
}
