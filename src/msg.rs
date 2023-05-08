use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

use crate::state::Job;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    CreateNewJob {
        worker: Addr,
        commitment: String,
        description: String,
        owner_signature: String,
    },

    AcceptJob {
        job_id: u128,
        worker_signature: String,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // Get last job_id
    #[returns(GetLastJobIdResponse)]
    GetLastJobId {},

    // Query job by job_id
    #[returns(GetJobResponse)]
    GetJob { job_id: u128 },
}

#[cw_serde]
pub struct GetLastJobIdResponse {
    pub last_job_id: u128,
}

#[cw_serde]
pub struct GetJobResponse {
    pub job: Job,
}
