use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub struct Workflow {}

pub struct Node {}

#[derive(Serialize, Deserialize, Debug)]
pub struct NodeData {
   pub id: String,
   pub name: String,
   pub node_type: String,
   pub parameters: Option<HashMap<String, String>>,
   pub next_node: Option<String>,
}

impl NodeData {
    pub fn new(
        id: String,
        name: String,
        node_type: String,
        parameters: Option<HashMap<String, String>>,
        next_node: Option<String>,
    ) -> NodeData {
        NodeData {
            id,
            name,
            next_node,
            node_type,
            parameters,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkflowData {
    pub name: String,
    pub nodes: Vec<NodeData>,
    pub connections: HashMap<String, Vec<String>>,
    pub meta_data: Option<HashMap<String, String>>,
}

impl WorkflowData {
    pub fn new(
        name: String,
        nodes: Vec<NodeData>,
        connections: HashMap<String, Vec<String>>,
        meta_data: Option<HashMap<String, String>>,
    ) -> WorkflowData {
        WorkflowData {
            name,
            connections,
            meta_data,
            nodes,
        }
    }
}
