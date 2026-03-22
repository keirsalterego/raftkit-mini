use crate::types::*;
use openraft::network::RPCOption;
use openraft::error::{RPCError, RaftError, NetworkError, InstallSnapshotError};
use openraft::BasicNode;
use openraft::raft::{
    AppendEntriesRequest, AppendEntriesResponse,
    VoteRequest, VoteResponse,
    InstallSnapshotRequest, InstallSnapshotResponse,
};
use openraft::RaftNetwork;


pub struct HttpNetwork {
    pub target_id: NodeId,
    pub target_addr: String,
}

impl RaftNetwork<TypeConfig> for HttpNetwork {
    async fn append_entries(
        &mut self,
        rpc: AppendEntriesRequest<TypeConfig>,
        _option: RPCOption,
    ) -> Result<
        AppendEntriesResponse<NodeId>,
        RPCError<NodeId, BasicNode, RaftError<NodeId>>,
    > {
        let url = format!("http://{}/raft/append", self.target_addr);
        let response = reqwest::Client::new()
            .post(&url)
            .json(&rpc)
            .send()
            .await
            .map_err(|e| RPCError::Network(NetworkError::new(&e)))?;

        response
            .json::<AppendEntriesResponse<NodeId>>()
            .await
            .map_err(|e| RPCError::Network(NetworkError::new(&e)))
    }

    async fn vote(
        &mut self,
        _rpc: VoteRequest<NodeId>,
        _option: RPCOption,
    ) -> Result<
        VoteResponse<NodeId>,
        RPCError<NodeId, BasicNode, RaftError<NodeId>>,
    > {
        unimplemented!()
    }

    async fn install_snapshot(
        &mut self,
        _rpc: InstallSnapshotRequest<TypeConfig>,
        _option: RPCOption,
    ) -> Result<
        InstallSnapshotResponse<NodeId>,
        RPCError<NodeId, BasicNode, RaftError<NodeId, InstallSnapshotError>>,
    > {
        unimplemented!()
    }
}

