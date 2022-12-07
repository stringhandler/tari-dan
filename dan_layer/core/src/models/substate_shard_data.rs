//   Copyright 2022 The Tari Project
//   SPDX-License-Identifier: BSD-3-Clause

use serde::{Deserialize, Serialize};
use tari_dan_common_types::{quorum_certificate::QuorumCertificate, NodeHeight, PayloadId, ShardId, TreeNodeHash};
use tari_engine_types::substate::Substate;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SubstateShardData {
    shard_id: ShardId,
    version: u32,
    substate: Substate,
    created_height: NodeHeight,
    destroyed_height: Option<NodeHeight>,
    created_node_hash: TreeNodeHash,
    destroyed_node_hash: Option<TreeNodeHash>,
    created_payload_id: PayloadId,
    destroyed_payload_id: Option<PayloadId>,
    created_justify: QuorumCertificate,
    destroyed_justify: Option<QuorumCertificate>,
}

impl SubstateShardData {
    pub fn new(
        shard_id: ShardId,
        version: u32,
        substate: Substate,
        created_height: NodeHeight,
        destroyed_height: Option<NodeHeight>,
        created_node_hash: TreeNodeHash,
        destroyed_node_hash: Option<TreeNodeHash>,
        created_payload_id: PayloadId,
        destroyed_payload_id: Option<PayloadId>,
        created_justify: QuorumCertificate,
        destroyed_justify: Option<QuorumCertificate>,
    ) -> Self {
        Self {
            shard_id,
            version,
            substate,
            created_height,
            destroyed_height,
            created_node_hash,
            destroyed_node_hash,
            created_payload_id,
            destroyed_payload_id,
            created_justify,
            destroyed_justify,
        }
    }

    pub fn shard_id(&self) -> ShardId {
        self.shard_id
    }

    pub fn substate(&self) -> &Substate {
        &self.substate
    }

    pub fn version(&self) -> u32 {
        self.version
    }

    pub fn created_height(&self) -> NodeHeight {
        self.created_height
    }

    pub fn destroyed_height(&self) -> Option<NodeHeight> {
        self.destroyed_height
    }

    pub fn created_node_hash(&self) -> TreeNodeHash {
        self.created_node_hash
    }

    pub fn destroyed_node_hash(&self) -> Option<TreeNodeHash> {
        self.destroyed_node_hash
    }

    pub fn created_payload_id(&self) -> PayloadId {
        self.created_payload_id
    }

    pub fn destroyed_payload_id(&self) -> Option<PayloadId> {
        self.destroyed_payload_id
    }

    pub fn created_justify(&self) -> &QuorumCertificate {
        &self.created_justify
    }

    pub fn destroyed_justify(&self) -> &Option<QuorumCertificate> {
        &self.destroyed_justify
    }
}
