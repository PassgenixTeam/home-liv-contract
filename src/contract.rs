#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Addr, Binary, Deps, DepsMut, Env, Event, MessageInfo, Response, StdResult,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Job, JOBS, LAST_JOB_ID};

// version info for migration info
const CONTRACT_NAME: &str = "HomeLib";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    LAST_JOB_ID.save(deps.storage, &0)?;

    let event = Event::new("Instantiated")
        .add_attribute("contract_name", CONTRACT_NAME.to_owned())
        .add_attribute("contract_version", CONTRACT_VERSION.to_owned());

    Ok(Response::default().add_event(event))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreateNewJob {
            worker,
            commitment,
            description,
            owner_signature,
        } => execute::create_new_job(deps, info, worker, commitment, description, owner_signature),

        ExecuteMsg::AcceptJob {
            job_id,
            worker_signature,
        } => execute::accept_job(deps, info, job_id, worker_signature),
    }
}

pub mod execute {
    use super::*;

    pub fn create_new_job(
        deps: DepsMut,
        info: MessageInfo,
        worker: Addr,
        commitment: String,
        description: String,
        owner_signature: String,
    ) -> Result<Response, ContractError> {
        // TODO: Validate signature

        let new_job = Job {
            owner: info.sender,
            worker,
            description,
            commitment,
            owner_signature,
            worker_signature: "".to_owned(),
        };

        let last_job_id =
            LAST_JOB_ID.update(deps.storage, |mut state| -> Result<_, ContractError> {
                state += 1;
                Ok(state)
            })?;

        JOBS.save(deps.storage, last_job_id, &new_job)?;

        let event =
            Event::new("CreatedNewJob").add_attribute("last_job_id", last_job_id.to_string());

        Ok(Response::new().add_event(event))
    }

    pub fn accept_job(
        deps: DepsMut,
        info: MessageInfo,
        job_id: u128,
        worker_signature: String,
    ) -> Result<Response, ContractError> {
        // TODO: validate job_id
        let _last_job_id = LAST_JOB_ID.load(deps.storage)?;

        JOBS.update(
            deps.storage,
            job_id,
            |mut _job| -> Result<_, ContractError> {
                let mut job = _job.unwrap();
                job.worker_signature = worker_signature.to_owned();
                job.worker = info.sender.to_owned();
                Ok(job)
            },
        )?;

        let event = Event::new("AcceptedJob")
            .add_attribute("job_id", job_id.to_string())
            .add_attribute("worker", info.sender.to_string())
            .add_attribute("worker_signature", worker_signature);

        Ok(Response::new().add_event(event))
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetLastJobId {} => to_binary(&query::last_job_id(deps)?),

        QueryMsg::GetJob { job_id } => to_binary(&query::get_job(deps, job_id)?),
    }
}

pub mod query {
    use crate::msg::{GetJobResponse, GetLastJobIdResponse};

    use super::*;

    pub fn last_job_id(deps: Deps) -> StdResult<GetLastJobIdResponse> {
        let last_job_id = LAST_JOB_ID.load(deps.storage)?;
        Ok(GetLastJobIdResponse { last_job_id })
    }

    pub fn get_job(deps: Deps, job_id: u128) -> StdResult<GetJobResponse> {
        let job = JOBS.load(deps.storage, job_id)?;
        Ok(GetJobResponse { job })
    }
}

#[cfg(test)]
mod tests {
    use crate::msg::{GetJobResponse, GetLastJobIdResponse};

    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

    #[test]
    fn check_instantiate() {
        let mut deps = mock_dependencies();
        let creator_info = mock_info("creator", &coins(1000, "token"));

        // Check response of initialization
        let msg = InstantiateMsg {};
        let res = instantiate(deps.as_mut(), mock_env(), creator_info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // Check initial states
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetLastJobId {}).unwrap();
        let value: GetLastJobIdResponse = from_binary(&res).unwrap();
        assert_eq!(0, value.last_job_id);
    }

    #[test]
    fn check_create_new_job() {
        let mut deps = mock_dependencies();
        let creator_info = mock_info("creator", &coins(1000, "token"));
        let sender1_info = mock_info("sender1", &coins(1000, "token"));
        let sender2_info = mock_info("sender2", &coins(1000, "token"));

        let instantiate_msg = InstantiateMsg {};
        instantiate(deps.as_mut(), mock_env(), creator_info, instantiate_msg).unwrap();

        // Create new job
        let commitment: String = "<EUENO link to commitment>".to_owned();
        let description: String = "<EUENO link to description>".to_owned();
        let owner_signature: String = "Sender1 signature".to_owned();

        let msg = ExecuteMsg::CreateNewJob {
            worker: sender2_info.sender.to_owned(),
            commitment: commitment.to_owned(),
            description: description.to_owned(),
            owner_signature: owner_signature.to_owned(),
        };
        execute(deps.as_mut(), mock_env(), sender1_info.to_owned(), msg).unwrap();

        // Check new job_id
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetLastJobId {}).unwrap();
        let value: GetLastJobIdResponse = from_binary(&res).unwrap();
        assert_eq!(1, value.last_job_id);

        // Check new job
        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::GetJob {
                job_id: value.last_job_id,
            },
        )
        .unwrap();
        let value: GetJobResponse = from_binary(&res).unwrap();
        let job = value.job;
        assert_eq!(job.commitment, commitment.to_owned());
        assert_eq!(job.description, description.to_owned());
        assert_eq!(job.owner_signature, owner_signature.to_owned());
        assert_eq!(job.owner, sender1_info.sender.to_owned());
        assert_eq!(job.worker, sender2_info.sender.to_owned());
        assert_eq!(job.worker_signature, "".to_owned());
    }

    #[test]
    fn check_accept_job() {
        let mut deps = mock_dependencies();
        let creator_info = mock_info("creator", &coins(1000, "token"));
        let sender1_info = mock_info("sender1", &coins(1000, "token"));
        let sender2_info = mock_info("sender2", &coins(1000, "token"));

        let instantiate_msg = InstantiateMsg {};
        instantiate(deps.as_mut(), mock_env(), creator_info, instantiate_msg).unwrap();

        // Create new job
        let commitment: String = "<EUENO link to commitment>".to_owned();
        let description: String = "<EUENO link to description>".to_owned();
        let owner_signature: String = "Sender1 signature".to_owned();

        let msg = ExecuteMsg::CreateNewJob {
            worker: sender2_info.sender.to_owned(),
            commitment: commitment.to_owned(),
            description: description.to_owned(),
            owner_signature: owner_signature.to_owned(),
        };
        execute(deps.as_mut(), mock_env(), sender1_info.to_owned(), msg).unwrap();

        // Get job_id
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetLastJobId {}).unwrap();
        let value: GetLastJobIdResponse = from_binary(&res).unwrap();
        let job_id = value.last_job_id;

        // Accept job
        let worker_signature: String = "Sender2 signature".to_owned();

        let msg = ExecuteMsg::AcceptJob {
            job_id,
            worker_signature: worker_signature.to_owned(),
        };
        execute(deps.as_mut(), mock_env(), sender2_info.to_owned(), msg).unwrap();

        // Check new job
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetJob { job_id }).unwrap();
        let value: GetJobResponse = from_binary(&res).unwrap();
        let job = value.job;
        assert_eq!(job.commitment, commitment.to_owned());
        assert_eq!(job.description, description.to_owned());
        assert_eq!(job.owner_signature, owner_signature.to_owned());
        assert_eq!(job.owner, sender1_info.sender.to_owned());
        assert_eq!(job.worker, sender2_info.sender.to_owned());
        assert_eq!(job.worker_signature, worker_signature.to_owned());
    }
}
