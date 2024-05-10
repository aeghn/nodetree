use axum::{
    extract::State,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use ntcore::{
    mapper::{
        node::{
            NodeDeleteReq, NodeMoveReq, NodeRenameReq, NodeUpdateContentReq, NodeUpdateReadonlyReq,
        },
        nodefilter::{NodeFetchReq, NodeFilter},
    },
    model::node::Node,
    /*     parser::toent::timestamp::guess_tss,
     */
};
use serde::Deserialize;
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
        .route("/api/guess-toent", post(guess_toent))
        .route("/api/update-node-content", post(update_node_content))
        .route("/api/update-node-readonly", post(update_node_readonly))
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
    let rest = state
        .mapper
        .query_nodes(&req)
        .await
        .map(|e| crate::adapter::node_with_children::nodes_with_childrens(e));

    let rest = match rest {
        Ok(Ok(o)) => Ok(o),
        Ok(err) => err,
        Err(err) => Err(err),
    };

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

async fn update_node_content(
    state: State<WebAppState>,
    Json(req): Json<NodeUpdateContentReq>,
) -> impl IntoResponse {
    let res = state.mapper.update_node_content(&req).await;
    print_and_trans_to_response(res)
}

async fn update_node_readonly(
    state: State<WebAppState>,
    Json(req): Json<NodeUpdateReadonlyReq>,
) -> impl IntoResponse {
    let res = state.mapper.update_node_readonly(&req).await;
    print_and_trans_to_response(res)
}

#[derive(Clone, Debug, Deserialize)]
struct TimeGuessReq {
    input: String,
}

async fn guess_toent(
    _state: State<WebAppState>,
    Json(req): Json<TimeGuessReq>,
) -> impl IntoResponse {
    info!(
        "parse_time: {:?}, {}",
        req, _state.config.common.asset_base_dir
    );

    let rest = Ok(ntcore::parser::toent::Toent::guess(req.input.as_str()));
    print_and_trans_to_response(rest)
}

#[cfg(test)]
mod test {
    use ntcore::model::node::{ContentParsedInfo, MagicNodeId, Node};
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
            MagicNodeId::Empty
        } else {
            MagicNodeId::Id(
                (pid.clone() + "." + (this - 1).to_string().as_str())
                    .to_string()
                    .into(),
            )
        };

        let cur = chrono::Utc::now();

        Node {
            id: id.clone().into(),
            delete_time: None,
            name: id.clone(),
            content: String::new(),
            domain: String::new(),
            parent_id: pid.into(),
            prev_sliding_id: prev.into(),
            version_time: cur.clone(),
            initial_time: cur.clone(),
            parsed_info: ContentParsedInfo::default(),
            node_type: ntcore::model::node::NodeType::TiptapV1,
            readonly: false,
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
