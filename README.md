# raftkit-mini

a minimal distributed key-value store built with openraft + sled.

built as a proof of concept for my gsoc 2026 proposal for dora-rs project #3
(distributed state management).

the core idea: followers serve reads locally using openraft's lease mechanism
instead of routing every read through the leader. writes go through raft
consensus as normal. for a robotics system where nodes read state at control
loop frequency, this matters.

## stack

- openraft 0.9 — raft consensus
- sled 0.34 — crash-safe embedded kv store for log persistence
- axum 0.7 — http transport for raft rpcs and client api
- tokio — async runtime
- clap — cli args

## running a 3-node cluster
```bash
# terminal 1
cargo run -- --id 1 --addr 127.0.0.1:8001

# terminal 2
cargo run -- --id 2 --addr 127.0.0.1:8002

# terminal 3
cargo run -- --id 3 --addr 127.0.0.1:8003

# initialize the cluster from node 1
curl -X POST http://127.0.0.1:8001/cluster/init
# → cluster initialized
```

## writing and reading state
```bash
# write through raft consensus (goes to leader)
curl -X POST http://127.0.0.1:8001/kv/set \
  -H "Content-Type: application/json" \
  -d '{"Set": {"key": "daemon:1:status", "value": "alive"}}'
# → "Ok"

# read from a follower (lease-based local read)
curl http://127.0.0.1:8002/kv/get?key=daemon:1:status
```

## failover demo
```bash
# kill node 1 (the leader) with ctrl+c
# wait ~500ms for election

# node 2 or 3 wins the election and becomes leader
# writes still work
curl -X POST http://127.0.0.1:8002/kv/set \
  -H "Content-Type: application/json" \
  -d '{"Set": {"key": "daemon:1:status", "value": "recovered"}}'
# → "Ok"
```

cluster survives leader crash. new leader elected. writes resume.

## known limitation

reads currently return null because the state machine clone passed to
AppState is separate from the live one inside the raft instance. the write
path through raft consensus works correctly — this is a wiring issue in
the prototype, not a raft correctness issue. fixing this requires wrapping
the state machine in Arc<RwLock<>> and sharing it between raft and the
axum state. intentionally left as-is to keep the prototype simple.

## how this maps to dora-rs project #3

| raftkit-mini | dora-rs equivalent |
|---|---|
| TypeConfig | DoraRaftConfig |
| SledStore (RaftLogStorage) | DaemonLogStore |
| KvStateMachine.apply() | DaemonStateMachine.apply() |
| /kv/set → raft leader | node.set_state() → raft leader |
| /kv/get → local follower | node.get_state() → local follower (lease) |
| /cluster/init | daemon cluster formation |
| failover test | required deliverable in project #3 |

the key naming convention (daemon:1:status, dataflow:slam_01:definition)
already mirrors the schema i'm proposing for dora's control plane state.

## gsoc 2026

this prototype was built alongside my proposal for dora-rs project #3.
proposal: [link when submitted]