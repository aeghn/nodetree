use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use ntcore::node::Node;
use ntmapper::Mapper;

#[derive(Clone)]
pub struct WebAppState {
    mapper: Arc<dyn Mapper>,
}

pub async fn serve(mapper: Arc<dyn Mapper>, ip: &str, port: &u16) {
    let state = WebAppState { mapper };
    let app = Router::new()
        .route("/insertNode", post(insert_node))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", ip, port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn insert_node(state: State<WebAppState>, Json(node): Json<Node>) -> impl IntoResponse {
    (
        StatusCode::CREATED,
        state.mapper.update_or_insert_node(&node).await.unwrap(),
    )
}
