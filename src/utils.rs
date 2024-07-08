use std::error::Error;
use crate::models::WorkflowData;


pub fn parse_workflow_data(body: &str) -> Result<WorkflowData, Box<dyn Error>> {
    let workflow_data: WorkflowData = serde_json::from_str(&body)?;
    Ok(workflow_data)
}
