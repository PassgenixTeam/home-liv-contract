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
        total_price: u128,
    },

    AcceptJob {
        job_id: u128,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // Get deposit fee percent
    #[returns(GetDepositFeePercentResponse)]
    GetDepositFeePercent {},

    // Get last job_id
    #[returns(GetLastJobIdResponse)]
    GetLastJobId {},

    // Query job by job_id
    #[returns(GetJobResponse)]
    GetJob { job_id: u128 },
}

#[cw_serde]
pub struct GetDepositFeePercentResponse {
    pub deposit_fee_percent: u128,
}

#[cw_serde]
pub struct GetLastJobIdResponse {
    pub last_job_id: u128,
}

#[cw_serde]
pub struct GetJobResponse {
    pub job: Job,
}
