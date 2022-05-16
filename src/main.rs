use std::{fs, io};
use std::net::SocketAddr;
use std::str::FromStr;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Router;
use axum::routing::get_service;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use crate::file_utils::file_data;
use crate::toml_parsing::parse;

#[path = "./parsing/toml-parser.rs"]
mod toml_parsing;

#[path = "./structs/core.rs"]
mod core_structs;

#[path = "./util/file.rs"]
mod file_utils;

#[path = "./util/option.rs"]
mod option_utils;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get_service(ServeDir::new("./public")).handle_error(handle_error))
        .layer(TraceLayer::new_for_http());
    let service = axum::Server::bind(&SocketAddr::from_str("127.0.0.1:8181").unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

}

async fn handle_error(_err: io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}