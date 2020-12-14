use futures::channel::mpsc;
use prost::DecodeError;
use thiserror::Error as ThisError;

use crate::protos::ChaincodeMessage;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("chaincode `{0}` is not registered")]
    NotRegistered(String),
    #[error("unsupported: {0}")]
    Unsupported(&'static str),
    #[error("chaincode msg decode failed: {0}")]
    DecodeError(#[from] DecodeError),
    #[error("chaincode chat stream is interrupted: {0}")]
    InterruptedStream(#[from] mpsc::SendError),
    #[error("gRPC error: {0}")]
    GrpcError(#[from] tonic::Status),
    #[error("invalid operation: {0}")]
    InvalidOperation(&'static str),
    #[error("invalid chaincode msg: {0:?}")]
    InvalidChaincodeMsg(ChaincodeMessage),
    #[error("unknown chaincode msg type: {0:?}")]
    UnknownChaincodeMsg(ChaincodeMessage),
    #[error("other error: {0}")]
    Other(String),
}
