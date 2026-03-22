mod types;
mod store;
mod network;
mod api;

use clap::Parser;
use openraft::{BasicNode, Config, Raft};
use openraft::network::RaftNetworkFactory;
use std::sync::Arc;

use crate::types::*;
use crate::store::{SledStore, KvStateMachine};
use crate::network::HttpNetwork;
use crate::api::{AppState, build_router};

#[derive(Parser)]
struct Cli {
    #[arg(long)]
    id: u64,
    #[arg(long)]
    addr: String,
}

struct HttpNetworkFactory;

impl RaftNetworkFactory<TypeConfig> for HttpNetworkFactory {
    type Network = HttpNetwork;

    async fn new_client(&mut self, target: NodeId, node: &BasicNode) -> Self::Network {
        HttpNetwork {
            target_id: target,
            target_addr: node.addr.clone(),
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let config = Arc::new(Config::default().validate()?);
    let store = SledStore::new(&format!("data/node-{}", cli.id))?;
    let sm = KvStateMachine::new();
    let sm_clone = sm.clone();
    let network = HttpNetworkFactory;

    let raft = Raft::new(cli.id, config, network, store, sm).await?;

    let state = Arc::new(AppState { raft, store: Arc::new(sm_clone) });
    let router = build_router(state);

    let listener = tokio::net::TcpListener::bind(&cli.addr).await?;
    axum::serve(listener, router).await?;

    Ok(())
}
