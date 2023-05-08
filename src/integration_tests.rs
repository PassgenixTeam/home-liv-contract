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

    // const DEFAULT_COMMITMENT: &str = "<EUENO link to commitment>";
    // const DEFAULT_DESCRIPTION: &str = "<EUENO link to description>";
    // const DEFAULT_OWNER_SIGNATURE: &str = "Sender 1 signature";

    fn mock_app() -> App {
        AppBuilder::new().build(|router, _, storage| {
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
        })
    }

    fn proper_instantiate() -> (App, CwTemplateContract) {
        let mut app = mock_app();
        let cw_template_id = app.store_code(contract_template());

        let msg = InstantiateMsg {
            author: "PassgenixTeam".to_owned(),
            description: "HomeLiv is a smart contract for create job.".to_owned(),
            copyright: "2023".to_owned(),
        };
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

    mod count {
        use super::*;
        use crate::msg::ExecuteMsg;

        #[test]
        fn flow1() {
            let (mut app, cw_template_contract) = proper_instantiate();

            // Create new job
            let msg = ExecuteMsg::CreateNewJob {
                worker: Addr::unchecked(USER2),
                commitment: "<EUENO link to commitment>".to_owned(),
                description: "<EUENO link to description>".to_owned(),
                owner_signature: "Sender 1 signature".to_owned(),
            };
            let cosmos_msg = cw_template_contract.call(msg).unwrap();
            app.execute(Addr::unchecked(USER1), cosmos_msg).unwrap();

            // Accept job
            // let msg = ExecuteMsg::AcceptJob { job_id: , worker_signature: () }
            // let cosmos_msg = cw_template_contract.call(msg).unwrap();
            // app.execute(Addr::unchecked(USER1), cosmos_msg).unwrap();
        }
    }
}
