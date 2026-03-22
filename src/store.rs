use crate::types::*;
use openraft::storage::{RaftLogStorage, RaftStateMachine};
use openraft::{
    Entry, EntryPayload, LogId, OptionalSend, RaftSnapshotBuilder, Snapshot, SnapshotMeta,
    StorageError, StorageIOError, StoredMembership, Vote,
};

pub struct SledStore {
    db: sled::Db,
    logs: sled::Tree,
    meta: sled::Tree,
}

impl Clone for SledStore {
    fn clone(&self) -> Self {
        Self {
            db: self.db.clone(),
            logs: self.logs.clone(),
            meta: self.meta.clone(),
        }
    }
}

pub struct KvStateMachine {
    data: std::collections::HashMap<String, String>,
    last_applied: Option<openraft::LogId<NodeId>>,
}

pub struct KvSnapshotBuilder;

impl SledStore {
    pub fn new(path: &str) -> anyhow::Result<Self> {
        let db = sled::open(path)?;
        let logs = db.open_tree("logs")?;
        let meta = db.open_tree("meta")?;
        Ok(Self { db, logs, meta })
    }
}

impl RaftLogStorage<TypeConfig> for SledStore {
    type LogReader = Self;

    async fn save_vote(&mut self, vote: &Vote<NodeId>) -> Result<(), StorageError<NodeId>> {
        let encoded = serde_json::to_vec(vote)
            .map_err(|e| StorageError::from(StorageIOError::write_vote(&e)))?;

        self.meta
            .insert(b"vote", encoded)
            .map_err(|e| StorageError::from(StorageIOError::write_vote(&e)))?;

        self.meta
            .flush()
            .map_err(|e| StorageError::from(StorageIOError::write_vote(&e)))?;

        Ok(())
    }

    async fn read_vote(&mut self) -> Result<Option<Vote<NodeId>>, StorageError<NodeId>> {
        let Some(raw) = self
            .meta
            .get(b"vote")
            .map_err(|e| StorageError::from(StorageIOError::read_vote(&e)))?
        else {
            return Ok(None);
        };

        let vote = serde_json::from_slice(raw.as_ref())
            .map_err(|e| StorageError::from(StorageIOError::read_vote(&e)))?;

        Ok(Some(vote))
    }

    async fn get_log_state(
        &mut self,
    ) -> Result<openraft::LogState<TypeConfig>, StorageError<NodeId>> {
        unimplemented!()
    }

    async fn get_log_reader(&mut self) -> Self::LogReader {
        self.clone()
    }

    async fn append<I>(
        &mut self,
        _entries: I,
        _callback: openraft::storage::LogFlushed<TypeConfig>,
    ) -> Result<(), StorageError<NodeId>>
    where
        I: IntoIterator<Item = openraft::Entry<TypeConfig>> + openraft::OptionalSend,
        I::IntoIter: openraft::OptionalSend,
    {
        unimplemented!()
    }

    async fn truncate(
        &mut self,
        _log_id: openraft::LogId<NodeId>,
    ) -> Result<(), StorageError<NodeId>> {
        unimplemented!()
    }

    async fn purge(
        &mut self,
        _log_id: openraft::LogId<NodeId>,
    ) -> Result<(), StorageError<NodeId>> {
        unimplemented!()
    }
}

impl openraft::storage::RaftLogReader<TypeConfig> for SledStore {
    async fn try_get_log_entries<R: std::ops::RangeBounds<u64> + openraft::OptionalSend>(
        &mut self,
        _range: R,
    ) -> Result<Vec<openraft::Entry<TypeConfig>>, StorageError<NodeId>> {
        unimplemented!()
    }
}

impl KvStateMachine {
    pub fn new() -> Self {
        Self {
            data: std::collections::HashMap::new(),
            last_applied: None,
        }
    }
}

impl RaftSnapshotBuilder<TypeConfig> for KvSnapshotBuilder {
    async fn build_snapshot(&mut self) -> Result<Snapshot<TypeConfig>, StorageError<NodeId>> {
        unimplemented!()
    }
}

impl RaftStateMachine<TypeConfig> for KvStateMachine {
    type SnapshotBuilder = KvSnapshotBuilder;

    async fn applied_state(
        &mut self,
    ) -> Result<
        (
            Option<LogId<NodeId>>,
            StoredMembership<NodeId, openraft::BasicNode>,
        ),
        StorageError<NodeId>,
    > {
        Ok((
            self.last_applied,
            StoredMembership::<NodeId, openraft::BasicNode>::default(),
        ))
    }

    async fn apply<I>(&mut self, entries: I) -> Result<Vec<KvResponse>, StorageError<NodeId>>
    where
        I: IntoIterator<Item = Entry<TypeConfig>> + OptionalSend,
        I::IntoIter: OptionalSend,
    {
        let mut responses = Vec::new();

        for entry in entries {
            match entry.payload {
                EntryPayload::Normal(req) => {
                    let response = match req {
                        KvRequest::Set { key, value } => {
                            self.data.insert(key, value);
                            KvResponse::Ok
                        }
                        KvRequest::Delete { key } => {
                            self.data.remove(&key);
                            KvResponse::Ok
                        }
                    };
                    responses.push(response);
                    self.last_applied = Some(entry.log_id);
                }
                _ => {}
            }
        }

        Ok(responses)
    }

    async fn get_snapshot_builder(&mut self) -> Self::SnapshotBuilder {
        KvSnapshotBuilder
    }

    async fn begin_receiving_snapshot(
        &mut self,
    ) -> Result<Box<<TypeConfig as openraft::RaftTypeConfig>::SnapshotData>, StorageError<NodeId>>
    {
        unimplemented!()
    }

    async fn install_snapshot(
        &mut self,
        _meta: &SnapshotMeta<NodeId, openraft::BasicNode>,
        _snapshot: Box<<TypeConfig as openraft::RaftTypeConfig>::SnapshotData>,
    ) -> Result<(), StorageError<NodeId>> {
        unimplemented!()
    }

    async fn get_current_snapshot(
        &mut self,
    ) -> Result<Option<Snapshot<TypeConfig>>, StorageError<NodeId>> {
        unimplemented!()
    }
}
