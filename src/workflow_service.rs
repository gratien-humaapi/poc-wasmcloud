use std::{any::Any, collections::HashMap};

use crate::{models::WorkflowData, nodes::Node, workflow_runner::WorkflowRunner};

pub struct WorkflowService {}

impl WorkflowService {
    pub fn execute_manually(workflow_data: WorkflowData, all_nodes: HashMap<&str, Box<dyn Node>>) {
        WorkflowRunner::run(workflow_data, all_nodes);
    }
}
