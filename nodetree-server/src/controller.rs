use std::sync::Arc;

use axum::{
    extract::State,
    handler::Handler,
    http::{Response, StatusCode},
    response::IntoResponse,
    routing::post,
    Json, Router,
};

use ntcore::{
    mapper::Mapper,
    model::{node::Node, nodefilter::NodeFilter},
};
use serde::Serialize;
use tracing::{error, info};

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
    info!("server: {}:{}", ip, port);

    axum::serve(listener, app).await.unwrap();
}

fn ferr(s: impl ToString) -> Response<String> {
    error!("{}", s.to_string());
    Response::builder().status(500).body(s.to_string()).unwrap()
}

#[derive(Serialize)]
struct VecNodeWrapper(Vec<Node>);

impl IntoResponse for VecNodeWrapper {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

async fn insert_node(state: State<WebAppState>, Json(node): Json<Node>) -> impl IntoResponse {
    info!("begin to insert node: {:?}", node);
    state
        .mapper
        .insert_and_move(&node)
        .await
        .map_err(|e| ferr(e))
}

async fn query_nodes(
    state: State<WebAppState>,
    Json(node_filter): Json<NodeFilter>,
) -> impl IntoResponse {
    info!("begin to query node: {:?}", node_filter);
    let rest = state.mapper.query_nodes(&node_filter).await.unwrap();
    (StatusCode::OK, VecNodeWrapper(rest))
}
