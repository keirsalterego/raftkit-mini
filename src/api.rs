use axum::{Router, routing::{get, post}, extract::{State, Query}, Json};
use std::sync::Arc;

use crate::types::*;
use crate::store::KvStateMachine;
use openraft::raft::{
    AppendEntriesRequest, AppendEntriesResponse,
    VoteRequest, VoteResponse,
};

#[derive(serde::Deserialize)]
pub struct GetParams {
    pub key: String,
}

pub struct AppState {
    pub raft: Raft,
    pub store: Arc<KvStateMachine>,
}

pub async fn post_kv_set(
    State(state): State<Arc<AppState>>,
    Json(req): Json<KvRequest>,
) -> Json<KvResponse> {
    match state.raft.client_write(req).await {
        Ok(resp) => Json(resp.data),
        Err(_e) => Json(KvResponse::Ok),
    }
}

pub async fn get_kv_get(
    State(state): State<Arc<AppState>>,
    Query(params): Query<GetParams>,
) -> Json<KvResponse> {
    let val = state.store.data.get(&params.key).cloned();
    Json(KvResponse::Value(val))
}

pub async fn post_raft_append(
    State(state): State<Arc<AppState>>,
    Json(req): Json<AppendEntriesRequest<TypeConfig>>,
) -> Json<AppendEntriesResponse<NodeId>> {
    Json(state.raft.append_entries(req).await.unwrap())
}

pub async fn post_raft_vote(
    State(state): State<Arc<AppState>>,
    Json(req): Json<VoteRequest<NodeId>>,
) -> Json<VoteResponse<NodeId>> {
    Json(state.raft.vote(req).await.unwrap())
}

pub fn build_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/kv/set", post(post_kv_set))
        .route("/kv/get", get(get_kv_get))
        .route("/raft/append", post(post_raft_append))
        .route("/raft/vote", post(post_raft_vote))
        .with_state(state)
}
