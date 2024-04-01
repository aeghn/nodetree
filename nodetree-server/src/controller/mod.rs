pub mod asset;

use std::{fmt::Debug, sync::Arc};

use axum::{
    extract::{DefaultBodyLimit, State},
    http::{
        header::{
            ACCEPT, ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS,
            ACCESS_CONTROL_ALLOW_ORIGIN, AUTHORIZATION, CONTENT_TYPE,
        },
        HeaderValue, StatusCode,
    },
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use ntcore::{
    mapper::{node::NodeMoveReq, nodefilter::NodeFilter, Mapper},
    model::node::Node,
};
use serde::Serialize;
use tower_http::{
    cors::{Any, CorsLayer},
    set_header::SetResponseHeaderLayer,
    trace::{self, TraceLayer},
};
use tracing::{debug, error, info, Level};

use crate::config::Config;

#[derive(Clone)]
pub struct WebAppState {
    mapper: Arc<dyn Mapper>,
    config: Arc<Config>,
}

fn routes() -> Router<WebAppState> {
    Router::new()
        .route("/api/insert-node", post(insert_node))
        .route("/api/insert-node-only", post(insert_node_only))
        .route("/api/fetch-nodes", post(fetch_nodes))
        .route("/api/fetch-all-nodes", get(fetch_all_nodes))
        .route("/api/move-node", post(move_node))
        .merge(asset::routes())
}

pub async fn serve(mapper: Arc<dyn Mapper>, config: Config) {
    let state = WebAppState {
        mapper,
        config: Arc::new(config.clone()),
    };

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
        .merge(routes())
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
        .layer(DefaultBodyLimit::disable())
        // https://stackoverflow.com/questions/73498537/axum-router-rejecting-cors-options-preflight-with-405-even-with-corslayer/
        .layer(cors_layer)
        .layer(trace_layer);

    let server_url = format!("{}:{}", "0.0.0.0", &config.server.port);

    let listener = tokio::net::TcpListener::bind(&server_url).await.unwrap();
    info!("server: {}", server_url);

    axum::serve(listener, app).await.unwrap();
}

fn print_and_trans_to_response<T>(result: anyhow::Result<T>) -> (StatusCode, Response)
where
    T: Serialize + Debug,
{
    match result {
        Ok(r) => {
            let j = Json(r);
            debug!("return result: {:?}", j);
            (StatusCode::OK, j.into_response())
        }
        Err(err) => {
            let err_str = err.to_string();
            error!("{}", err_str);
            (StatusCode::INTERNAL_SERVER_ERROR, err_str.into_response())
        }
    }
}

async fn insert_node(state: State<WebAppState>, Json(node): Json<Node>) -> impl IntoResponse {
    info!("insert_node: {:?}", node);
    let rest = state.mapper.insert_and_move(&node).await;
    print_and_trans_to_response(rest)
}

async fn insert_node_only(state: State<WebAppState>, Json(node): Json<Node>) -> impl IntoResponse {
    info!("insert_node: {:?}", node);
    let rest = state.mapper.insert_node_only(&node).await;
    print_and_trans_to_response(rest)
}

async fn fetch_nodes(state: State<WebAppState>, Json(req): Json<NodeFilter>) -> impl IntoResponse {
    info!("fetch_nodes: {:?}", req);
    let rest = state.mapper.query_nodes(&req).await;
    print_and_trans_to_response(rest)
}

async fn fetch_all_nodes(state: State<WebAppState>) -> impl IntoResponse {
    fetch_nodes(state, Json(NodeFilter::All)).await
}

async fn move_node(state: State<WebAppState>, Json(req): Json<NodeMoveReq>) -> impl IntoResponse {
    info!("move_node: {:?}", req);
    let rest = state.mapper.move_nodes(&req).await;
    print_and_trans_to_response(rest)
}

#[cfg(test)]
mod test {
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
