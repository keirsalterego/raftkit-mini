use serde::{Deserialize, Serialize};
use openraft::BasicNode;
use std::io::Cursor;

pub type NodeId = u64;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum KvRequest {
    Set { key: String, value: String },
    Delete { key: String },
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum KvResponse {
    Ok,
    Value(Option<String>),
}

openraft::declare_raft_types!(
    pub TypeConfig:
        D = KvRequest,
        R = KvResponse,
        NodeId = NodeId,
        Node = BasicNode,
    SnapshotData = Cursor<Vec<u8>>,
);


pub type Raft = openraft::Raft<TypeConfig>;