use std::{cell::RefCell, collections::HashMap, fmt::Debug, hash::Hash};

use chin_tools::log_and_err;

fn sort_node_pointers<T, S, SV>(vec_vec: &mut Vec<&NodePointer<T>>, s_func: S)
where
    SV: Ord,
    S: Fn(&NodePointer<T>) -> SV,
{
    vec_vec.sort_by(|e1, e2| s_func(e1).cmp(&s_func(&e2)))
}

#[derive(Debug)]
struct NodePointer<'v, T> {
    node: T,
    next: RefCell<Option<&'v NodePointer<'v, T>>>,
}

impl<'v, T> NodePointer<'v, T> {
    fn new(node: T) -> Self {
        Self {
            node,
            next: RefCell::new(None),
        }
    }

    fn put_next<K: Debug>(&self, next: &'v NodePointer<'v, T>, k: &K) -> anyhow::Result<()> {
        let n = self.next.borrow_mut().replace(next);
        if n.is_none() {
            Ok(())
        } else {
            log_and_err!("one node has many successors, {:?}", k)
        }
    }
}

pub fn sort_with_precessors<T, F, P, K, S, SV>(
    vec: Vec<T>,
    k_func: F,
    p_func: P,
    s_func: S,
) -> anyhow::Result<Vec<T>>
where
    T: Clone + Debug,
    K: Eq + Hash + Clone + Debug,
    SV: Ord,
    F: Fn(&T) -> K,
    P: Fn(&T) -> Option<K>,
    S: Fn(&T) -> SV,
{
    if vec.is_empty() {
        return Ok(vec);
    }

    let nodes: Vec<NodePointer<T>> = vec.into_iter().map(|e| NodePointer::new(e)).collect();

    let node_map: HashMap<K, &NodePointer<T>> =
        nodes.iter().map(|e| (k_func(&e.node), e)).collect();

    let mut heads: Vec<&NodePointer<T>> = vec![];

    for n in nodes.iter() {
        if let Some(Some(nd)) = p_func(&n.node).map(|k| node_map.get(&k)) {
            nd.put_next(n, &k_func(&nd.node))?;
        } else {
            heads.push(&n);
        }
    }

    sort_node_pointers(&mut heads, |e| s_func(&e.node));

    if heads.is_empty() {
        anyhow::bail!("Ther are no heads.");
    }

    let mut sorted = vec![];
    for ele in heads {
        let mut point = Some(ele);
        while let Some(n) = point {
            sorted.push(n.node.clone());
            point = *n.next.borrow();
        }
    }

    Ok(sorted)
}
