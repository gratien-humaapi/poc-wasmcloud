use serde_json::Value;
use std::{collections::HashMap, error::Error};

use crate::models::WorkflowData;

pub fn parse_workflow_data(body: &str) -> Result<WorkflowData, Box<dyn Error>> {
    let workflow_data: WorkflowData = serde_json::from_str(&body)?;
    Ok(workflow_data)
}

pub fn resolve_references(
    params: &HashMap<String, Value>,
    node_results: &HashMap<String, Value>,
) -> HashMap<String, String> {
    params
        .iter()
        .map(|(k, v)| {
            let mut value = String::new();

            if node_results.is_empty() {
                return (k.clone(), v.clone().to_string());
            }

            for (node_id, result) in node_results {
                let placeholder = format!("{{Node{}Result}}", node_id);
                value = v
                    .clone()
                    .to_string()
                    .replace(&placeholder, result.as_str().unwrap());
            }

            (k.clone(), value)
        })
        .collect()
}


  // Fonction pour construire le graphe des dépendances
pub fn build_dependency_graph(workflow_data: &WorkflowData) -> HashMap<String, Vec<String>> {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    for node_data in &workflow_data.nodes {
        if let Some(next_node) = node_data.clone().next_node {
            graph.entry(node_data.id.clone()).or_default().push(next_node);
        }
    }
    graph
}

// Fonction pour effectuer un tri topologique
pub fn topological_sort(workflow_data: &WorkflowData, graph: &HashMap<String, Vec<String>>) -> Vec<String> {
    let mut in_degree = HashMap::new();
    for node_data in workflow_data.nodes.clone() {
        in_degree.insert(node_data.id, 0);
    }

    for nodes in graph.values() {
        for node_id in nodes.clone() {
            *in_degree.entry(node_id).or_default() += 1;
        }
    }

    let mut stack: Vec<String> = Vec::new();
    for (node_id, degree) in in_degree.clone() {
        if degree == 0 {
            stack.push(node_id);
        }
    }

    let mut sorted_order: Vec<String> = Vec::new();
    while let Some(node_id) = stack.pop() {
        sorted_order.push(node_id.clone());
        if let Some(neighbors) = graph.get(&node_id) {
            for neighbor in neighbors {
                let entry = in_degree.entry(neighbor.clone()).or_default();
                *entry -= 1;
                if *entry == 0 {
                    stack.push(neighbor.clone());
                }
            }
        }
    }

    if sorted_order.len() == workflow_data.nodes.len() {
        sorted_order
    } else {
        vec![]
    }
}
