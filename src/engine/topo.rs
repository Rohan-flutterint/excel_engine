use std::collections::{HashMap, HashSet, VecDeque};

pub fn topo_sort(graph: &HashMap<String, HashSet<String>>) -> Result<Vec<String>, ()> {
    let mut indegree: HashMap<String, usize> = HashMap::new();

    for (node, deps) in graph {
        indegree.entry(node.clone()).or_insert(0);
        for d in deps {
            *indegree.entry(d.clone()).or_insert(0) += 1;
        }
    }

    let mut queue = VecDeque::new();
    for (k, v) in &indegree {
        if *v == 0 {
            queue.push_back(k.clone());
        }
    }

    let mut result = vec![];

    while let Some(n) = queue.pop_front() {
        result.push(n.clone());
        if let Some(deps) = graph.get(&n) {
            for d in deps {
                if let Some(v) = indegree.get_mut(d) {
                    *v -= 1;
                    if *v == 0 {
                        queue.push_back(d.clone());
                    }
                }
            }
        }
    }

    if result.len() != indegree.len() {
        Err(())
    } else {
        Ok(result)
    }
}
