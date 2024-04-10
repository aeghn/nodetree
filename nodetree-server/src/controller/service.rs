use axum::{
    extract::State,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use ntcore::{
    mapper::{
        node::{NodeDeleteReq, NodeMoveReq, NodeRenameReq},
        nodefilter::{NodeFetchReq, NodeFilter},
    },
    model::node::Node,
};
use tracing::info;

use crate::controller::print_and_trans_to_response;

use super::WebAppState;

pub fn routes() -> Router<WebAppState> {
    Router::new()
        .route("/api/insert-node", post(insert_node))
        .route("/api/insert-node-only", post(insert_node_only))
        .route("/api/fetch-nodes", post(fetch_nodes))
        .route("/api/fetch-all-nodes", get(fetch_all_nodes))
        .route("/api/move-node", post(move_node))
        .route("/api/delete-node", post(delete_node))
        .route("/api/update-node-name", post(update_node_name))
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

async fn fetch_nodes(
    state: State<WebAppState>,
    Json(req): Json<NodeFetchReq>,
) -> impl IntoResponse {
    info!("fetch_nodes: {:?}", req);
    let rest = state.mapper.query_nodes(&req).await;
    print_and_trans_to_response(rest)
}

async fn fetch_all_nodes(state: State<WebAppState>) -> impl IntoResponse {
    fetch_nodes(
        state,
        Json(NodeFetchReq {
            selection: None,
            filter: Some(NodeFilter::All),
        }),
    )
    .await
}

async fn move_node(state: State<WebAppState>, Json(req): Json<NodeMoveReq>) -> impl IntoResponse {
    info!("move_node: {:?}", req);
    let rest = state.mapper.move_nodes(&req).await;
    print_and_trans_to_response(rest)
}

async fn delete_node(
    state: State<WebAppState>,
    Json(req): Json<NodeDeleteReq>,
) -> impl IntoResponse {
    info!("delete_node: {:?}", req);
    let rest = state.mapper.delete_node(&req).await;
    print_and_trans_to_response(rest)
}

async fn update_node_name(
    state: State<WebAppState>,
    Json(req): Json<NodeRenameReq>,
) -> impl IntoResponse {
    info!("rename_node: {:?}", req);
    let rest = state.mapper.update_node_name(&req).await;
    print_and_trans_to_response(rest)
}

#[cfg(test)]
mod test {
    use ntcore::model::node::{ContentParsedInfo, Node};
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

        let cur = chrono::Utc::now();

        Node {
            id: id.clone().into(),
            delete_time: None,
            name: id.clone(),
            content: String::new(),
            user: String::new(),
            parent_id: Some(pid.into()),
            prev_sliding_id: prev.map(|e| e.into()),
            create_time: cur.clone(),
            first_version_time: cur.clone(),
            parsed_info: ContentParsedInfo::default(),
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
