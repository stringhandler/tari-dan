use serde::{Deserialize, Serialize};
use tari_dan_common_types::Epoch;
use tari_dan_common_types::shard::Shard;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EpochEnding {
    pub epoch: Epoch,
    pub action: EpochEndAction
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EpochEndAction {
    ContinueWithCurrentShard,
    MergeIntoShard{ shard: Shard},
    SplitInTwo{ shard_a: Shard, shard_b: Shard}
}