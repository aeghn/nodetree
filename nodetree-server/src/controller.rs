use std::{ops::Deref, sync::Arc};

use axum::{
    extract::{DefaultBodyLimit, State},
    http::{
        header::{
            ACCEPT, ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS,
            ACCESS_CONTROL_ALLOW_ORIGIN, AUTHORIZATION, CONTENT_TYPE,
        },
        HeaderValue, Response, StatusCode,
    },
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use ntcore::{
    mapper::Mapper,
    model::{
        node::{Node, NodeMoveResult},
        nodefilter::NodeFilter,
    },
};
use serde::Serialize;
use tower_http::{
    cors::{Any, CorsLayer},
    set_header::SetResponseHeaderLayer,
    trace::{self, TraceLayer},
};
use tracing::{error, info, Level};

#[derive(Clone)]
pub struct WebAppState {
    mapper: Arc<dyn Mapper>,
}

pub async fn serve(mapper: Arc<dyn Mapper>, ip: &str, port: &u16) {
    let state = WebAppState { mapper };

    let cors_layer = CorsLayer::new()
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE])
        .allow_methods(Any)
        .allow_origin(Any);

    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(trace::DefaultMakeSpan::new().level(Level::DEBUG))
        .on_response(trace::DefaultOnResponse::new().level(Level::DEBUG))
        .on_request(|req: &axum::http::Request<axum::body::Body>, _: &_| {
            info!("request: {:?}", req);
        });

    let app = Router::new()
        .layer(trace_layer)
        .layer(cors_layer)
        .route("/api/insert-node", post(insert_node))
        .route("/api/fetch-nodes", post(fetch_nodes))
        .route("/api/fetch-all-nodes", get(fetch_all_nodes))
        .with_state(state)
        .layer(SetResponseHeaderLayer::<_>::overriding(
            ACCESS_CONTROL_ALLOW_ORIGIN,
            HeaderValue::from_static("*"),
        ))
        .layer(SetResponseHeaderLayer::<_>::overriding(
            ACCESS_CONTROL_ALLOW_METHODS,
            HeaderValue::from_static("*"),
        ))
        .layer(SetResponseHeaderLayer::<_>::overriding(
            ACCESS_CONTROL_ALLOW_HEADERS,
            HeaderValue::from_static("*"),
        ))
        .layer(DefaultBodyLimit::disable());

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

#[derive(Serialize)]
struct NodeMoveResultW(NodeMoveResult);

impl IntoResponse for NodeMoveResultW {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

impl Deref for NodeMoveResultW {
    type Target = NodeMoveResult;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

async fn insert_node(state: State<WebAppState>, Json(node): Json<Node>) -> impl IntoResponse {
    info!("begin to insert node: {:?}", node);
    state
        .mapper
        .insert_and_move(&node)
        .await
        .map(|e| NodeMoveResultW(e))
        .map_err(|e| ferr(e))
}

async fn fetch_nodes(
    state: State<WebAppState>,
    Json(node_filter): Json<NodeFilter>,
) -> impl IntoResponse {
    info!("begin to query node: {:?}", node_filter);
    let rest = state.mapper.query_nodes(&node_filter).await.unwrap();
    (StatusCode::OK, VecNodeWrapper(rest))
}

async fn fetch_all_nodes(state: State<WebAppState>) -> impl IntoResponse {
    fetch_nodes(state, Json(NodeFilter::All)).await
}

#[cfg(test)]
mod test {
    use axum::http::request;
    use clap::builder::Str;
    use ntcore::model::node::Node;
    use regex::Regex;
    use reqwest::header::HeaderMap;

    fn this(vec: &Vec<i32>, this: i32) -> Node {
        let re = Regex::new(r"\.$").unwrap(); // Define the regex pattern to match digits
        let id: String = vec.iter().map(|e| e.to_string() + ".").collect::<String>();
        let id = re.replace(&id, "").to_string();
        let pid: String = vec
            .iter()
            .take(vec.len() - 1)
            .map(|e| e.to_string() + ".")
            .collect();
        let pid = re.replace(&pid, "").to_string();

        let prev = if this == 1 {
            None
        } else {
            Some(pid.clone() + "." + (this - 1).to_string().as_str())
        };

        let cur = chrono::NaiveDateTime::from_timestamp(0, 0);

        Node {
            id: id.clone().into(),
            version: 0,
            is_current: true,
            delete_time: None,
            name: id.clone(),
            content: String::new(),
            user: String::new(),
            todo_status: None,
            tags: vec![],
            parent_id: pid.into(),
            prev_sliding_id: prev.map(|e| e.into()),
            create_time: cur.clone(),
            first_version_time: cur.clone(),
        }
    }

    #[tokio::test]
    async fn test_insert() {
        for i in 1..=100000 {
            insert_nodes(i).await;
        }
    }

    async fn insert_nodes(prefix: i32) {
        let mut vec = vec![];
        vec.push(prefix);
        let mut nodes = vec![];
        for i in 1..=4 {
            vec.push(i);
            nodes.push(this(&vec, i));
            for j in 1..3 {
                vec.push(j);
                nodes.push(this(&vec, j));
                for k in 1..4 {
                    vec.push(k);
                    nodes.push(this(&vec, k));
                    let len = vec.len() - 1;
                    vec = vec.into_iter().take(len).collect();
                }
                let len = vec.len() - 1;
                vec = vec.into_iter().take(len).collect();
            }
            let len = vec.len() - 1;
            vec = vec.into_iter().take(len).collect();
        }

        let client = reqwest::Client::new();
        for n in &nodes {
            let mut headers = HeaderMap::new();
            headers.insert("Content-Type", "application/json".parse().unwrap());

            println!("insert {:?}", serde_json::to_string(&n).unwrap());

            let v = client
                .post("http://localhost:3011/api/insert-node")
                .json(&n)
                .send()
                .await
                .unwrap();
            println!("insert node result: {:?}", v)
        }
    }
}
