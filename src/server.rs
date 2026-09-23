use std::{
    future::Future,
    io,
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
};

use axum::Router;
use rmcp::transport::streamable_http_server::{
    StreamableHttpServerConfig, StreamableHttpService, session::never::NeverSessionManager,
};
use tokio::net::TcpListener;
use tokio_util::sync::CancellationToken;

use crate::handler::FinanceMcp;

pub const MCP_PATH: &str = "/mcp";

const ALLOWED_HOSTS: [&str; 2] = ["localhost", "127.0.0.1"];

pub async fn bind(port: u16) -> io::Result<TcpListener> {
    TcpListener::bind(SocketAddr::from((Ipv4Addr::LOCALHOST, port))).await
}

pub async fn serve(
    listener: TcpListener,
    shutdown: impl Future<Output = ()> + Send + 'static,
) -> io::Result<()> {
    let cancellation = CancellationToken::new();
    let router = Router::new().nest_service(MCP_PATH, mcp_service(cancellation.child_token()));

    axum::serve(listener, router)
        .with_graceful_shutdown(async move {
            shutdown.await;
            cancellation.cancel();
        })
        .await
}

fn mcp_service(
    cancellation: CancellationToken,
) -> StreamableHttpService<FinanceMcp, NeverSessionManager> {
    let config = StreamableHttpServerConfig::default()
        .with_legacy_session_mode(false)
        .with_json_response(true)
        .with_allowed_hosts(ALLOWED_HOSTS)
        .enforce_origin_validation()
        .with_cancellation_token(cancellation);

    StreamableHttpService::new(
        || Ok(FinanceMcp::new()),
        Arc::new(NeverSessionManager::default()),
        config,
    )
}
