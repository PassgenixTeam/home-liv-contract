use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct Metadata {
    pub author: String,
    pub description: String,
    pub copyright: String,
}
pub const METADATA: Item<Metadata> = Item::new("metadata");

#[cw_serde]
pub struct Job {
    pub owner: Addr,
    pub worker: Addr,
    pub description: String,
    pub commitment: String,
    pub owner_signature: String,
    pub worker_signature: String,
}
pub const LAST_JOB_ID: Item<u128> = Item::new("last_job_id");
pub const JOBS: Map<u128, Job> = Map::new("job");
