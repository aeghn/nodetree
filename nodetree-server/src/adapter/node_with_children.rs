use std::collections::{HashMap, HashSet};

use ntcore::{
    model::node::{Node, NodeId},
    utils::colutils::sort_with_precessors,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct NodeWithChildren {
    #[serde(flatten)]
    node: Node,
    children: Vec<Box<NodeWithChildren>>,
}

fn to_children(
    relation_map: &mut HashMap<&NodeId, (&Node, Vec<&Node>)>,
    nid: &NodeId,
) -> NodeWithChildren {
    let (n, children) = relation_map.remove(nid).unwrap();
    let children = children
        .iter()
        .map(|c| Box::new(to_children(relation_map, &c.id)))
        .collect();
    let children = sort_with_precessors(
        children,
        |e| e.node.id.clone(),
        |e| e.node.prev_sliding_id.clone().map(|e| e),
        |e| e.node.version_time.clone(),
    );
    NodeWithChildren {
        node: n.clone(),
        children,
    }
}

pub fn nodes_with_childrens(nodes: Vec<Node>) -> Vec<NodeWithChildren> {
    let mut relation_map: HashMap<&NodeId, (&Node, Vec<&Node>)> = HashMap::new();
    let mut top_lvl_ids: HashSet<&NodeId> = HashSet::new();

    nodes.iter().for_each(|e| {
        relation_map.insert(&e.id, (e, vec![]));
    });

    let mut nodes_ref: Vec<&Node> = vec![];
    relation_map
        .values()
        .for_each(|(node, _)| nodes_ref.push(node));

    nodes_ref.iter().for_each(|e| match &e.parent_id {
        Some(parent_id) => {
            let v = relation_map.get_mut(&parent_id);
            match v {
                Some(n2) => {
                    n2.1.push(e);
                }
                None => {
                    top_lvl_ids.insert(&e.id);
                }
            }
        }
        None => {
            top_lvl_ids.insert(&e.id);
        }
    });

    sort_with_precessors(
        top_lvl_ids
            .iter()
            .map(|nid| to_children(&mut relation_map, nid))
            .collect(),
        |e| e.node.id.clone(),
        |e| e.node.prev_sliding_id.clone().map(|e| e),
        |e| e.node.version_time.clone(),
    )
}

#[cfg(test)]
mod test {

    use std::vec;

    use chrono::Utc;
    use ntcore::model::node::{ContentParsedInfo, Node};

    use crate::adapter::node_with_children::nodes_with_childrens;

    use super::NodeWithChildren;

    #[test]
    fn test() {
        let node1 = Node {
            id: "a".to_string().into(),
            delete_time: None,
            name: "a".to_string(),
            content: "a".to_string(),
            user: "a".to_string(),
            parsed_info: ContentParsedInfo::default(),
            parent_id: None,
            prev_sliding_id: None,
            version_time: Utc::now(),
            initial_time: Utc::now(),
            node_type: ntcore::model::node::NodeType::TiptapV1,
        };
        let node = NodeWithChildren {
            node: node1.clone(),
            children: vec![],
        };

        nodes_with_childrens(vec![node1]);

        println!("{:?}", serde_json::to_string(&node).unwrap())
    }
}
