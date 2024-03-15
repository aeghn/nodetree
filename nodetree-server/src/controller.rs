use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::post, Json, Router};

use ntcore::{
    mapper::Mapper,
    model::{node::Node, nodefilter::NodeFilter},
};
use serde::Serialize;

#[derive(Clone)]
pub struct WebAppState {
    mapper: Arc<dyn Mapper>,
}

pub async fn serve(mapper: Arc<dyn Mapper>, ip: &str, port: &u16) {
    let state = WebAppState { mapper };
    let app = Router::new()
        .route("/insertNode", post(insert_node))
        .route("/getNodes", post(query_nodes))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", ip, port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn insert_node(state: State<WebAppState>, Json(node): Json<Node>) -> impl IntoResponse {
    (
        StatusCode::CREATED,
        state.mapper.insert_and_move(&node).await.unwrap(),
    )
}

#[derive(Serialize)]
struct VecNodeWrapper(Vec<Node>);

impl IntoResponse for VecNodeWrapper {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

async fn query_nodes(
    state: State<WebAppState>,
    Json(node_filter): Json<NodeFilter>,
) -> impl IntoResponse {
    let rest = state.mapper.query_nodes(&node_filter).await.unwrap();
    (StatusCode::OK, VecNodeWrapper(rest))
}
