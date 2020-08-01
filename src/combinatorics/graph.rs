use std::collections::HashMap;

/// edge list undirected graph 
pub struct Graph<T> {
    pub nodes: HashMap<String,T>,
    /// Hashmap<edge,(node,node)>
    pub edges: HashMap<String,(String,String)>,
}

impl<T> Graph<T> where {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, label: String, node: T ) -> Option<T> {
        self.nodes.insert(label,  node)
    }

    pub fn connect(&mut self, label: String, a: String, b: String) -> Result<Option<(String,String)>,GraphConnectError>{
        if self.nodes.contains_key(&a) && self.nodes.contains_key(&b) {
            Ok(self.edges.insert(label,(a,b)))
        } else {
            Err(GraphConnectError::NodeNotExit)
        }
    }
}

use std::collections::HashSet;
/// adjacency list durected graph
pub struct DGraph<T> {
    pub nodes: HashMap<String,T>,
    pub edges: HashMap<String,HashSet<String>>,
}

impl<T> DGraph<T> {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, label: String, node: T) -> Option<T> {
        self.nodes.insert(label, node)
    }

    pub fn connect(&mut self, a: String, b: String) -> Result<bool,GraphConnectError>{
        if self.nodes.contains_key(&a) && self.nodes.contains_key(&b) {
            if let Some(next_nodes) = self.edges.get_mut(&a) {
                Ok(next_nodes.insert(b))
            } else {
                self.edges.insert(a.clone(), HashSet::new());
                Ok(self.edges.get_mut(&a).unwrap().insert(b))
            }
        } else {
            Err(GraphConnectError::NodeNotExit)
        }
    }

    pub fn acyclic_connect(&mut self, a: String, b: String) -> Result<bool,GraphConnectError>{
        if self.nodes.contains_key(&a) && self.nodes.contains_key(&b) {
            if self.connect_acyclic_test(&a, &b) {
                if let Some(next_nodes) = self.edges.get_mut(&a) {
                    Ok(next_nodes.insert(b))
                } else {
                    self.edges.insert(a.clone(), HashSet::new());
                    Ok(self.edges.get_mut(&a).unwrap().insert(b))
                }
            } else {
                Err(GraphConnectError::CyclicConnect)
            }
        } else {
            Err(GraphConnectError::NodeNotExit)
        }
    }

    fn connect_acyclic_test(&self, a: &String, b: &String) -> bool {
        if a == b {
            false
        } else if let Some(next_nodes) = self.edges.get(b) {
            if next_nodes.contains(a) {
                false
            } else {
                for i in next_nodes.iter() {
                    if self.connect_acyclic_test(a, i) {
                        return true
                    }
                }
                false
            }
        } else {
            true
        }
    }
}

#[derive(Debug,PartialEq)]
pub enum GraphConnectError {
    NodeNotExit,
    CyclicConnect,
}