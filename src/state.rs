use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct Job {
    pub owner: Addr,
    pub worker: Addr,
    pub description: String,
    pub commitment: String,
    pub total_price: u128,
}
pub const LAST_JOB_ID: Item<u128> = Item::new("last_job_id");

pub const JOBS: Map<u128, Job> = Map::new("job");

pub const DEPOSIT_FEE_PERCENT: Item<u128> = Item::new("deposit_fee_percent");
