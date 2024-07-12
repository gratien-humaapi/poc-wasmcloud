mod models;
mod nodes;
mod utils;
mod workflow_runner;
mod workflow_service;

wit_bindgen::generate!();

use std::fmt::Debug;

use crate::wasi::io::streams::StreamError;
use anyhow::{anyhow, bail, Result};
use exports::wasi::http::incoming_handler::Guest;
use models::WorkflowData;
use nodes::{AddNode, Node, PrintNode, TriggerNode};
use utils::parse_workflow_data;
use wasi::http::types::*;
use workflow_service::WorkflowService;
use std::collections::HashMap;

const MAX_READ_BYTES: u32 = 2048;

struct WorkflowController;

impl Guest for WorkflowController {
    fn handle(request: IncomingRequest, response_out: ResponseOutparam) {
        // Créer la structure a similaire à { "add": AddNode, "print": PrintNode }
        let mut all_nodes: HashMap<&str, Box<dyn Node>> = HashMap::new();
        all_nodes.insert("add", Box::new(AddNode));
        all_nodes.insert("print", Box::new(PrintNode));
        all_nodes.insert("trigger", Box::new(TriggerNode));

        let path = request.path_with_query().unwrap();
        let splited_path: Vec<&str> = path.split("/").collect();

        match (request.method(), splited_path.as_slice()) {
            (Method::Post, [_, "manual-trigger", ..]) => {
                let body = request.read_body().expect("Incorrect body format");

                let result = std::str::from_utf8(&body).expect("Can not convert to str");

                let workflow_data: Result<WorkflowData, _> = parse_workflow_data(&result);
                if workflow_data.is_ok() {
                    WorkflowService::execute_manually(workflow_data.unwrap(), all_nodes);

                    send_response(response_out, 200, "Is OK");
                } else {
                    send_response(response_out, 400, "Workflow not found");
                }
            }
            _ => (),
        }
    }
}

fn send_response<T>(response_out: ResponseOutparam, status_code: u16, result: T)
where
    T: Debug,
{
    let response = OutgoingResponse::new(Fields::new());
    response.set_status_code(status_code).unwrap();
    let response_body = response.body().unwrap();
    ResponseOutparam::set(response_out, Ok(response));
    response_body
        .write()
        .unwrap()
        .blocking_write_and_flush(format!(" {result:?}").as_bytes())
        .unwrap();
    OutgoingBody::finish(response_body, None).expect("failed to finish response body");
}

impl IncomingRequest {
    /// This is a convenience function that writes out the body of a IncomingRequest (from wasi:http)
    /// into anything that supports [`std::io::Write`]
    fn read_body(self) -> Result<Vec<u8>> {
        // Read the body
        let incoming_req_body = self
            .consume()
            .map_err(|()| anyhow!("failed to consume incoming request body"))?;

        let incoming_req_body_stream = incoming_req_body
            .stream()
            .map_err(|()| anyhow!("failed to build stream for incoming request body"))?;

        let mut buf = Vec::<u8>::with_capacity(MAX_READ_BYTES as usize);
        loop {
            match incoming_req_body_stream.blocking_read(MAX_READ_BYTES as u64) {
                Ok(bytes) => buf.extend(bytes),
                Err(StreamError::Closed) => {
                    break;
                }
                Err(e) => bail!("failed to read bytes: {e}"),
            }
        }
        buf.shrink_to_fit();
        drop(incoming_req_body_stream);
        IncomingBody::finish(incoming_req_body);
        Ok(buf)
    }
}

export!(WorkflowController);
