use std::collections::{HashMap, HashSet};

pub struct Graph {
    pub deps: HashMap<String, HashSet<String>>,
}

impl Graph {
    pub fn new() -> Self {
        Self { deps: HashMap::new() }
    }

    pub fn add_dep(&mut self, cell: String, dep: String) {
        self.deps.entry(cell).or_default().insert(dep);
    }

    pub fn detect_cycle(&self) -> bool {
        fn dfs(node: &str, graph: &Graph, visiting: &mut HashSet<String>, visited: &mut HashSet<String>) -> bool {
            if visiting.contains(node) {
                return true;
            }
            if visited.contains(node) {
                return false;
            }

            visiting.insert(node.to_string());
            if let Some(neighbors) = graph.deps.get(node) {
                for n in neighbors {
                    if dfs(n, graph, visiting, visited) {
                        return true;
                    }
                }
            }
            visiting.remove(node);
            visited.insert(node.to_string());
            false
        }

        let mut visiting = HashSet::new();
        let mut visited = HashSet::new();

        for node in self.deps.keys() {
            if dfs(node, self, &mut visiting, &mut visited) {
                return true;
            }
        }
        false
    }
}
