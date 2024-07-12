#![cfg(test)]
use serde::{de::DeserializeOwned, Serialize};

use cosmwasm_std::{
    from_binary, to_json_binary, Addr, BalanceResponse as BalanceResponseBank, BankQuery, Coin, Empty,
    Querier, QueryRequest, StdError, Uint128, WasmQuery,
};
use cw_multi_test::{App, AppResponse, Contract, ContractWrapper, Executor};

use crate::token::InstantiateMsg as Cw721InstantiateMsg;

use crate::contract::DENOM;
use crate::msg::{ExecuteMsg, InitMsg, InstantiateMsg};
use crate::state::State;

pub static NAME_PREFIX: &str = "Token #";

pub fn mock_app() -> App {
    App::default()
}

pub fn contract_whitelist_minter() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        crate::contract::execute,
        crate::contract::instantiate,
        crate::contract::query,
    );
    Box::new(contract)
}

pub fn create_whitelist_minter(
    router: &mut App,
    owner: Addr,
    supply: u64,
    whitelist_allowance: u64,
    whitelist_members: Vec<Addr>,
    reserved_recipient: Addr,
    price: Uint128,
) -> Addr {
    let wlm_id = router.store_code(contract_whitelist_minter());

    let cw721 = Addr::unchecked("temp_value"); // Must be updated to actual cw721 contract
                                               // after both contracts have been deployed
                                               // otherwise ExecuteMsg::Initialize will fail

    let msg = InstantiateMsg {
        cw721,
        supply,
        public_whitelist_allowance: whitelist_allowance,
        public_whitelist_members: whitelist_members.clone(),
        private_whitelist_allowance: whitelist_allowance,
        reserved_recipient,
        price,
        naming_prefix: NAME_PREFIX.to_string(),
        private_whitelist_members: whitelist_members,
    };

    router
        .instantiate_contract(wlm_id, owner.clone(), &msg, &[], "whitelist-minter", None)
        .unwrap()
}

pub fn init_whitelist_minter(
    router: &mut App,
    owner: Addr,
    wlm_contract: Addr,
    config: State,
) -> AppResponse {
    // Update State with correct cw721 contract address
    let config_msg = ExecuteMsg::UpdateConfig { config };
    let _config_res =
        router.execute_contract(owner.clone(), wlm_contract.clone(), &config_msg, &[]);
    // dbg!(_config_res);

    // Init contract and open for whitelist minting
    let msg = ExecuteMsg::Initialize(InitMsg {});
    let res = router.execute_contract(owner, wlm_contract, &msg, &[]);

    res.unwrap()
}

pub fn create_cw721(router: &mut App, minter: &Addr) -> Addr {
    let cw721_id = router.store_code(contract_cw721());
    let msg = Cw721InstantiateMsg {
        name: "TESTNFT".to_string(),
        symbol: "TEST".to_string(),
        minter: String::from(minter),
    };

    router
        .instantiate_contract(cw721_id, minter.clone(), &msg, &[], "cw721", None)
        .unwrap()
}

pub fn contract_cw721() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        crate::token::entry::execute,
        crate::token::entry::instantiate,
        crate::token::entry::query,
    );
    Box::new(contract)
}

#[allow(dead_code)]
pub fn mint_native(app: &mut App, beneficiary: String, amount: Uint128) {
    app.sudo(cw_multi_test::SudoMsg::Bank(
        cw_multi_test::BankSudo::Mint {
            to_address: beneficiary,
            amount: vec![Coin {
                denom: DENOM.to_string(),
                amount,
            }],
        },
    ))
    .unwrap();
}

pub fn query<M, T>(router: &mut App, target_contract: Addr, msg: M) -> Result<T, StdError>
where
    M: Serialize + DeserializeOwned,
    T: Serialize + DeserializeOwned,
{
    router.wrap().query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: target_contract.to_string(),
        msg: to_json_binary(&msg).unwrap(),
    }))
}

#[allow(dead_code)]
pub fn bank_query(app: &App, address: &Addr) -> Coin {
    let req: QueryRequest<BankQuery> = QueryRequest::Bank(BankQuery::Balance {
        address: address.to_string(),
        denom: DENOM.to_string(),
    });
    let res = app.raw_query(&to_json_binary(&req).unwrap()).unwrap().unwrap();
    let balance: BalanceResponseBank = from_binary(&res).unwrap();
    balance.amount
}
