mod types;
mod store;
mod network;
mod api;

use crate::types::*;
use crate::network::HttpNetwork;
use openraft::network::RaftNetworkFactory;
use openraft::BasicNode;

#[derive(clap::Parser)]
struct Cli {
    #[arg(long)]
    id: u64,
    #[arg(long)]
    addr: String,
}

pub struct HttpNetworkFactory;

impl RaftNetworkFactory<TypeConfig> for HttpNetworkFactory {
    type Network = HttpNetwork;

    async fn new_client(&mut self, target: NodeId, node: &BasicNode) -> Self::Network {
        HttpNetwork {
            target_id: target,
            target_addr: node.addr.clone(),
        }
    }
}

fn main() {
    println!("Hello, world!");
}
