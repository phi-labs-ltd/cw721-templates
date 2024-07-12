#[cfg(not(feature = "library"))]
use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};

use cw2::{get_contract_version, set_contract_version};

use crate::error::ContractError;
use crate::execute::{execute_enable_public_mint, execute_enable_reveal, execute_init, execute_mint, execute_reveal, execute_update_config, execute_private_whitelist_approve, execute_private_whitelist_remove, execute_withdraw_funds, execute_public_whitelist_remove, execute_public_whitelist_approve, execute_enable_normal_whitelist};
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::query::{query_config, query_token_statuses, query_whitelist_member};
use crate::state::{State, WhitelistMember, STATE, PUBLIC_WHITELIST, PRIVATE_WHITELIST};

// Mainnet
pub static DENOM: &str = "aarch";
// Testnet
// pub static DENOM: &str = "aconst";

// version info for migration info
const CONTRACT_NAME: &str = "whitelist-minter";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    // Create Config State
    let state = State {
        owner: info.sender.clone(),
        cw721: msg.cw721.clone(),
        artist: msg.reserved_recipient.clone(),
        supply: msg.supply,
        phase: Default::default(),
        private_whitelist_allowance: msg.private_whitelist_allowance,
        public_whitelist_allowance: msg.public_whitelist_allowance,
        price: msg.price,
        name_prefix: msg.naming_prefix,
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    // Create Whitelist
    for member in msg.public_whitelist_members.iter() {
        let whitelist_member = WhitelistMember { whitelisted: true };
        PUBLIC_WHITELIST.update(deps.storage, member, |existing| match existing {
            None => Ok(whitelist_member.clone()),
            Some(_) => Err(ContractError::InvalidInput {}),
        })?;
    }
    for member in msg.private_whitelist_members.iter() {
        let whitelist_member = WhitelistMember { whitelisted: true };
        PRIVATE_WHITELIST.update(deps.storage, member, |existing| match existing {
            None => Ok(whitelist_member.clone()),
            Some(_) => Err(ContractError::InvalidInput {}),
        })?;
    }

    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("cw721", msg.cw721)
        .add_attribute("supply", msg.supply.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Mint(msg) => execute_mint(deps, env, info, msg),
        ExecuteMsg::Reveal(msg) => execute_reveal(deps, env, info, msg),

        // Artist only
        ExecuteMsg::Withdraw(msg) => execute_withdraw_funds(deps, env, info, msg),

        // Admin only
        ExecuteMsg::Initialize(msg) => execute_init(deps, env, info, msg),
        ExecuteMsg::EnableNormalWhitelist(msg) => execute_enable_normal_whitelist(deps, env, info, msg),
        ExecuteMsg::EnablePublicMint(msg) => execute_enable_public_mint(deps, env, info, msg),
        ExecuteMsg::EnableReveal(msg) => execute_enable_reveal(deps, env, info, msg),
        ExecuteMsg::PublicWhitelistRemove(msg) => execute_public_whitelist_remove(deps, env, info, msg),
        ExecuteMsg::PublicWhitelistApprove(msg) => execute_public_whitelist_approve(deps, env, info, msg),
        ExecuteMsg::PrivateWhitelistRemove(msg) => execute_private_whitelist_remove(deps, env, info, msg),
        ExecuteMsg::PrivateWhitelistApprove(msg) => execute_private_whitelist_approve(deps, env, info, msg),
        ExecuteMsg::UpdateConfig { config } => execute_update_config(deps, env, info, config),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    Ok(match msg {
        QueryMsg::Config {} => to_json_binary(&query_config(deps)?),
        QueryMsg::TokenStatuses { token_ids } => to_json_binary(&query_token_statuses(deps, token_ids)?),
        QueryMsg::Whitelist { address } => to_json_binary(&query_whitelist_member(deps, address)?),
    }?)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    let original_version = get_contract_version(deps.storage)?;
    let name = CONTRACT_NAME.to_string();
    let version = CONTRACT_VERSION.to_string();
    if original_version.contract != name {
        return Err(ContractError::InvalidInput {});
    }
    if original_version.version >= version {
        return Err(ContractError::InvalidInput {});
    }

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::default())
}
