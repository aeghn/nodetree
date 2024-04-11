use std::{collections::HashMap, hash::Hash};

pub fn sort_vecs_by_first<T, S, SV>(vec_vec: &mut Vec<Vec<T>>, s_func: S)
where
    SV: Ord,
    S: Fn(&T) -> SV,
{
    vec_vec.sort_by(|e1, e2| s_func(&e1[0]).cmp(&s_func(&e2[0])))
}

pub fn sort_with_precessors<T, F, P, K, S, SV>(
    vec: Vec<T>,
    k_func: F,
    p_func: P,
    s_func: S,
) -> Vec<T>
where
    K: Eq + Hash + Clone,
    SV: Ord,
    F: Fn(&T) -> K,
    P: Fn(&T) -> Option<K>,
    S: Fn(&T) -> SV,
{
    if vec.is_empty() {
        return vec;
    }

    let mut t_map: HashMap<K, T> = vec.into_iter().map(|e| (k_func(&e), e)).collect();
    let mut map_not_empty = true;
    let mut vec_vec: Vec<Vec<T>> = vec![];

    while map_not_empty {
        let ks = {
            t_map
                .iter()
                .take(1)
                .map(|e| e.0.clone())
                .collect::<Vec<K>>()
        };

        let key = &ks[0];
        let any = t_map.remove(&key).unwrap();
        let mut prev = p_func(&any);

        let mut prev_vec = vec![any];
        while let Some(p) = prev {
            if let Some(p) = t_map.remove(&p) {
                prev = p_func(&p);
                prev_vec.insert(0, p);
            } else {
                prev = None;
            }
        }

        vec_vec.push(prev_vec);
        map_not_empty = !t_map.is_empty();
    }

    sort_vecs_by_first(&mut vec_vec, s_func);

    vec_vec.into_iter().flatten().collect()
}
