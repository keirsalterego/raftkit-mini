# raftkit-mini

a minimal distributed key-value store built with openraft + sled.

built as a proof of concept for my gsoc 2026 proposal for dora-rs project #3 
(distributed state management).

## what this is

three nodes. raft consensus. sled for persistence. lease-based reads so 
followers serve get requests locally without hitting the leader every time.

## stack

- openraft 0.9
- sled 0.34
- axum 0.7
- tokio

## status

- [x] types — RaftTypeConfig, KvRequest, KvResponse
- [x] store — SledStore (log storage) + KvStateMachine
- [ ] network — HTTP Raft RPC transport
- [ ] api — client endpoints + raft rpc handlers
- [ ] main — boots the node, wires everything
- [ ] failover test — kill leader, state survives

## mapping to dora-rs

| raftkit-mini | dora-rs project #3 |
|---|---|
| TypeConfig | DoraRaftConfig |
| SledStore | DaemonLogStore |
| KvStateMachine.apply() | DaemonStateMachine.apply() |
| /kv/get (lease read) | node.get_state() |
| /kv/set | node.set_state() |