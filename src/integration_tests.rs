#[cfg(test)]
mod tests {
    use crate::helpers::CwTemplateContract;
    use crate::msg::InstantiateMsg;
    use cosmwasm_std::{Addr, Coin, Empty, Uint128};
    use cw_multi_test::{App, AppBuilder, Contract, ContractWrapper, Executor};

    pub fn contract_template() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(
            crate::contract::execute,
            crate::contract::instantiate,
            crate::contract::query,
        );
        Box::new(contract)
    }

    const ADMIN: &str = "ADMIN";
    const USER1: &str = "USER1";
    const USER2: &str = "USER2";
    const NATIVE_DENOM: &str = "denom";

    fn mock_app() -> App {
        AppBuilder::new().build(|router, _, storage| {
            router
                .bank
                .init_balance(
                    storage,
                    &Addr::unchecked(ADMIN),
                    vec![Coin {
                        denom: NATIVE_DENOM.to_string(),
                        amount: Uint128::new(1000),
                    }],
                )
                .unwrap();

            router
                .bank
                .init_balance(
                    storage,
                    &Addr::unchecked(USER1),
                    vec![Coin {
                        denom: NATIVE_DENOM.to_string(),
                        amount: Uint128::new(1000),
                    }],
                )
                .unwrap();

            router
                .bank
                .init_balance(
                    storage,
                    &Addr::unchecked(USER2),
                    vec![Coin {
                        denom: NATIVE_DENOM.to_string(),
                        amount: Uint128::new(1000),
                    }],
                )
                .unwrap();
        })
    }

    fn proper_instantiate() -> (App, CwTemplateContract) {
        let mut app = mock_app();
        let cw_template_id = app.store_code(contract_template());

        let msg = InstantiateMsg {};
        let cw_template_contract_addr = app
            .instantiate_contract(
                cw_template_id,
                Addr::unchecked(ADMIN),
                &msg,
                &[],
                "test",
                None,
            )
            .unwrap();

        let cw_template_contract = CwTemplateContract(cw_template_contract_addr);

        (app, cw_template_contract)
    }

    mod flow_tests {
        use super::*;
        use crate::msg::{ExecuteMsg, GetJobResponse, GetLastJobIdResponse, QueryMsg};

        #[test]
        fn flow1() {
            let (mut app, cw_template_contract) = proper_instantiate();

            // Create new job
            let commitment: String = "<EUENO link to commitment>".to_owned();
            let description: String = "<EUENO link to description>".to_owned();
            let total_price: u128 = 1000;

            let msg = ExecuteMsg::CreateNewJob {
                worker: Addr::unchecked(USER2),
                commitment: commitment.to_owned(),
                description: description.to_owned(),
                total_price,
            };
            let cosmos_msg = cw_template_contract.call(msg).unwrap();
            app.execute(Addr::unchecked(USER1), cosmos_msg).unwrap();
            let resp: GetLastJobIdResponse = app
                .wrap()
                .query_wasm_smart(cw_template_contract.addr(), &QueryMsg::GetLastJobId {})
                .unwrap();
            let last_job_id = resp.last_job_id;

            // Accept job

            let msg = ExecuteMsg::AcceptJob {
                job_id: last_job_id,
            };
            let cosmos_msg = cw_template_contract.call(msg).unwrap();
            app.execute(Addr::unchecked(USER2), cosmos_msg).unwrap();

            // Check job
            let resp: GetJobResponse = app
                .wrap()
                .query_wasm_smart(
                    cw_template_contract.addr(),
                    &QueryMsg::GetJob {
                        job_id: last_job_id,
                    },
                )
                .unwrap();
            let job = resp.job;
            assert_eq!(job.commitment, commitment.to_owned());
            assert_eq!(job.description, description.to_owned());
            assert_eq!(job.owner, Addr::unchecked(USER1));
            assert_eq!(job.worker, Addr::unchecked(USER2));
        }
    }
}
