mod error;
mod handler;
mod server;

use crate::protos::ChaincodeInput;
use crate::protos::ChaincodeMessage;
use log::{debug, info};
use prost::Message;
pub use server::ChaincodeSupportService;

#[derive(Debug)]
pub enum Task {
    Chaincode(ChaincodeMessage),
    Executor(ExecutorCommand),
}

#[derive(Debug)]
pub struct ExecutorCommand {
    tx_hash: Vec<u8>,
    payload: Vec<u8>,
}

impl ExecutorCommand {
    pub fn new(tx_hash: Vec<u8>, payload: Vec<u8>) -> Self {
        Self { tx_hash, payload }
    }
}

pub trait MessageDump {
    fn dump(&self) -> Vec<u8>;
}

impl<T: Message> MessageDump for T {
    fn dump(&self) -> Vec<u8> {
        let mut payload = vec![];
        self.encode(&mut payload).unwrap();
        payload
    }
}
