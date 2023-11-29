use crate::api::api_routes;
use crate::error::MyResult;
use crate::federation::routes::federation_routes;
use crate::utils::generate_activity_id;
use activitypub_federation::config::FederationMiddleware;
use axum::{Router, Server};
use diesel::Connection;
use diesel::PgConnection;
use federation::federation_config;
use std::net::ToSocketAddrs;
use tracing::info;

pub mod api;
pub mod database;
pub mod error;
pub mod federation;
mod utils;

pub async fn start(hostname: &str) -> MyResult<()> {
    let config = federation_config(hostname).await?;

    info!("Listening with axum on {hostname}");
    let config = config.clone();
    let app = Router::new()
        .nest("", federation_routes())
        .nest("/api/v1", api_routes())
        .layer(FederationMiddleware::new(config));

    let addr = hostname
        .to_socket_addrs()?
        .next()
        .expect("Failed to lookup domain name");
    let server = Server::bind(&addr).serve(app.into_make_service());

    tokio::spawn(server);

    Ok(())
}

pub fn establish_db_connection() -> MyResult<PgConnection> {
    // TODO: read from config file
    let database_url = "postgres://fediwiki:password@localhost:5432/fediwiki";
    Ok(PgConnection::establish(&database_url)?)
}
