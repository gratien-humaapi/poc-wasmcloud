use std::{any::Any, borrow::Borrow, collections::HashMap};

use serde::de::IntoDeserializer;
use serde_json::Value;

use crate::{
    models::{NodeResult, WorkflowData},
    nodes::{AddNode, Node, PrintNode},
    wasi,
};

fn get_node_params(workflow_data: WorkflowData, node_id: &str) -> Option<HashMap<String, Value>> {
    for nodes_data in workflow_data.nodes.iter() {
        if nodes_data.id == node_id {
            return nodes_data.parameters.clone();
        }
    }
    None
}

fn get_input_data<'a>(
    workflow_data: &'a WorkflowData,
    execution_results: &'a HashMap<String, Value>,
    node_id: &str,
) -> Option<&'a Value> {
    for connection in &workflow_data.connections {
        if let Some(result) = execution_results.get(&connection.from) {
            // let next_node = workflow_data.nodes
            //     .iter()
            //     .find(|n| n.id == connection.to)
            //     .unwrap();

                if connection.to == node_id {
                    return Some(result);
                }

        }
    }

    // for nodes_data in workflow_data.nodes.iter() {
    //     if nodes_data.id == node_id {
    //         return nodes_data.parameters.clone();
    //     }
    // }

    None
}

pub struct WorkflowRunner {}

impl WorkflowRunner {
    pub fn run(workflow_data: WorkflowData, all_nodes: HashMap<&str, Box<dyn Node>>) {
        let mut execution_results: HashMap<String, Value> = HashMap::new();

        
        for node_data in workflow_data.nodes.clone().into_iter() {
            let a = node_data.name.as_str();

            // let prev_output = workflow_data.connections.get(&node_data.id).and_then(|prev_ids| {
            //     prev_ids
            //         .iter()
            //         .find_map(|prev_id| execution_results.get(prev_id))
            // });
            // wasi::logging::logging::log(
            //     wasi::logging::logging::Level::Info,
            //     "",
            //     &format!("prev_output = {prev_output:?}"),
            // );

            if let Some(node_box) = all_nodes.get(&a) {

               let input_data: Option<&Value> = get_input_data(&workflow_data, &execution_results, &node_data.id);

                wasi::logging::logging::log(
                    wasi::logging::logging::Level::Info,
                    "",
                    &format!("input_data = {input_data:?}"),
                );

                let g = AsRef::<dyn Node>::as_ref(node_box);

                let result = g.execute(node_data.parameters.clone(), input_data);

                execution_results.insert(
                    node_data.id.clone(),
                    serde_json::json!(format!("{:?}", result)),
                );

                wasi::logging::logging::log(
                    wasi::logging::logging::Level::Info,
                    "",
                    &format!("execution_results = {execution_results:?}"),
                );
            } else {
                wasi::logging::logging::log(
                    wasi::logging::logging::Level::Info,
                    "",
                    &format!("Node type {} not found in all_nodes", node_data.name),
                );
            }
        }
    }
}
