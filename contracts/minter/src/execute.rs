use cosmwasm_std::{BankMsg, Coin, CosmosMsg, DepsMut, Env, MessageInfo, QueryRequest, Response, to_json_binary, WasmMsg, WasmQuery};
use crate::token::{
    ExecuteMsg as Cw721ExecuteMsg, Extension, Metadata, QueryMsg as Cw721QueryMsg,
};
use crate::token::cw721::{ NftInfoResponse, NumTokensResponse, OwnerOfResponse };

use crate::contract::DENOM;
use crate::msg::{EnablePublicMintMsg, EnableRevealMsg, EnableWhitelistMintMsg, InitMsg, MintMsg, RevealMsg, WhitelistApproveMsg, WhitelistRemoveMsg, WithdrawMsg};
use crate::state::{State, WhitelistMember, STATE, PUBLIC_WHITELIST, PUBLIC_WHITELIST_COUNTER, PRIVATE_WHITELIST, PRIVATE_WHITELIST_COUNTER, Phase};

use crate::error::ContractError;

pub fn execute_mint(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: MintMsg,
) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;

    // Contract must be initialized
    if state.phase.is_disabled() {
        return Err(ContractError::Unauthorized {});
    }

    // Minting must not be expired
    if state.phase.is_reveal() {
        return Err(ContractError::MintExpired {});
    }

    // Enforce white list permissions
    if state.phase.is_public_whitelist() {
        // Returns an error directly if not in whitelist
        let whitelist_member = PUBLIC_WHITELIST.load(deps.storage, &info.sender)?;
        // Returns an error if whitelisting disabled for this sender
        if !whitelist_member.whitelisted {
            return Err(ContractError::NotWhitelisted {});
        }

        PUBLIC_WHITELIST_COUNTER.update(deps.storage, &info.sender, |minted| match minted {
            Some(minted) if minted >= state.public_whitelist_allowance => {
                Err(ContractError::WhitelistAllowance { minted })
            }
            _ => Ok(minted.unwrap_or(0) + 1),
        })?;
    }

    if state.phase.is_private_whitelist() {
        // Returns an error directly if not in whitelist
        let whitelist_member = PRIVATE_WHITELIST.load(deps.storage, &info.sender)?;
        // Returns an error if whitelisting disabled for this sender
        if !whitelist_member.whitelisted {
            return Err(ContractError::NotWhitelisted {});
        }

        PRIVATE_WHITELIST_COUNTER.update(deps.storage, &info.sender, |minted| match minted {
            Some(minted) if minted >= state.private_whitelist_allowance => {
                Err(ContractError::WhitelistAllowance { minted })
            }
            _ => Ok(minted.unwrap_or(0) + 1),
        })?;
    }

    // Get numeric token_id
    let query_msg: crate::token::QueryMsg<Extension> = Cw721QueryMsg::NumTokens {};
    let query_req = QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: state.cw721.clone().into(),
        msg: to_json_binary(&query_msg).unwrap(),
    });
    let query_resp: NumTokensResponse = deps.querier.query(&query_req)?;
    let token_id = query_resp.count + 1;

    // Fail if minting would exceed capacity
    if token_id > state.supply {
        return Err(ContractError::SoldOut {});
    }

    // User must send funds equal to (or, higher than) minting price
    let required_payment = Coin {
        denom: DENOM.to_string(),
        amount: state.price,
    };
    check_sent_required_payment(&info.funds, Some(required_payment))?;

    // Mint empty NFT with no metadata
    let mint_msg: crate::token::ExecuteMsg = Cw721ExecuteMsg::Mint{{token_mint_params}};
    let mint_resp: CosmosMsg = WasmMsg::Execute {
        contract_addr: state.cw721.into(),
        msg: to_json_binary(&mint_msg)?,
        funds: vec![],
    }
    .into();

    Ok(Response::new()
        .add_attribute("action", "mint")
        .add_attribute("token_id", token_id.to_string())
        .add_attribute("owner", info.sender.to_string())
        .add_message(mint_resp))
}

/// Public minting must be enabled manually.
/// This transaction can only be broadcast by
/// the contract admin account.
pub fn execute_enable_normal_whitelist(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: EnableWhitelistMintMsg,
) -> Result<Response, ContractError> {
    let mut state = STATE.load(deps.storage)?;

    // Contract must be initialized
    if state.phase.is_disabled() {
        return Err(ContractError::Unauthorized {});
    }

    // Public minting must not be enabled already
    if state.phase.is_public_whitelist() {
        return Err(ContractError::PublicWhitelistMintEnabled {});
    }

    // Only contract owner can enable public minting
    if info.sender != state.owner {
        return Err(ContractError::Unauthorized {});
    }

    state.phase = Phase::NormalWhitelist;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new().add_attribute("action", "execute_enable_public_mint"))
}

/// Public minting must be enabled manually.
/// This transaction can only be broadcast by
/// the contract admin account.
pub fn execute_enable_public_mint(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: EnablePublicMintMsg,
) -> Result<Response, ContractError> {
    let mut state = STATE.load(deps.storage)?;

    // Contract must be initialized
    if state.phase.is_disabled() {
        return Err(ContractError::Unauthorized {});
    }

    // Public minting must not be enabled already
    if state.phase.is_public_mint() {
        return Err(ContractError::PublicMintEnabled {});
    }

    // Only contract owner can enable public minting
    if info.sender != state.owner {
        return Err(ContractError::Unauthorized {});
    }

    state.phase = Phase::Public;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new().add_attribute("action", "execute_enable_public_mint"))
}

/// Revealing must be enabled manually. This transaction
/// can only be broadcast by the contract admin account.
pub fn execute_enable_reveal(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: EnableRevealMsg,
) -> Result<Response, ContractError> {

    // {{execute_enable_reveal}}

    // Expand to this is non-updatable
    // Err(ContractError::EntrypointDisabled {})

    // Expand to this if updatable
    let mut state = STATE.load(deps.storage)?;

    // Contract must be initialized
    if state.phase.is_disabled() {
        return Err(ContractError::Unauthorized {});
    }

    // Must be in pre-reveal phase
    if state.phase.is_reveal() {
        return Err(ContractError::RevealEnabled {});
    }

    // Only contract owner can enable reveal
    if info.sender != state.owner {
        return Err(ContractError::Unauthorized {});
    }

    state.phase = Phase::Reveal;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new().add_attribute("action", "enable_reveal"))
}

pub fn execute_reveal(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: RevealMsg,
) -> Result<Response, ContractError> {
    {{execute_reveal}}
}

// Can only be called by artist
pub fn execute_withdraw_funds(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: WithdrawMsg,
) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    let amount = msg.amount;

    if state.artist != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    let bank_transfer_msg = BankMsg::Send {
        to_address: info.sender.into(),
        amount: ([Coin {
            denom: DENOM.to_string(),
            amount,
        }])
        .to_vec(),
    };

    let bank_transfer: CosmosMsg = cosmwasm_std::CosmosMsg::Bank(bank_transfer_msg);

    Ok(Response::new()
        .add_attribute("action", "withdraw")
        .add_attribute("amount", amount.to_string())
        .add_message(bank_transfer))
}

// Can only be called by admin
pub fn execute_update_config(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    config_update: State,
) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;

    if state.owner != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    STATE.save(deps.storage, &config_update)?;

    Ok(Response::new().add_attribute("action", "update_config"))
}

// Can only be called by admin. Note that minting reserved NFTs
// to the founder does not enforce the minting price (state.price)
pub fn execute_init(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InitMsg,
) -> Result<Response, ContractError> {
    let mut state = STATE.load(deps.storage)?;

    if state.owner != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    if state.phase.is_enabled() {
        return Err(ContractError::Initialized {});
    }

    state.phase = Phase::PrivateWhitelist;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("action", "initialize"))
}

// Only admin can remove whitelist members
pub fn execute_public_whitelist_remove(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: WhitelistRemoveMsg,
) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;

    // Only admin can remove members
    if state.owner != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    for member in msg.whitelist_members.iter() {
        PUBLIC_WHITELIST.remove(deps.storage, member);
    }

    Ok(Response::new().add_attribute("action", "public_whitelist_remove"))
}

// Only admin can approve whitelist members. Can
// also be used to bulk add members, as non-existing
// entries will be automatically created and approved
pub fn execute_public_whitelist_approve(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: WhitelistApproveMsg,
) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;

    // Only admin can approve members
    if state.owner != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    for member in msg.whitelist_members.iter() {
        let member_approval = WhitelistMember { whitelisted: true };
        PUBLIC_WHITELIST.save(deps.storage, member, &member_approval)?;
    }

    Ok(Response::new().add_attribute("action", "public_whitelist_approve"))
}

// Only admin can remove whitelist members
pub fn execute_private_whitelist_remove(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: WhitelistRemoveMsg,
) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;

    // Only admin can remove members
    if state.owner != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    for member in msg.whitelist_members.iter() {
        PRIVATE_WHITELIST.remove(deps.storage, member);
    }

    Ok(Response::new().add_attribute("action", "private_whitelist_remove"))
}

// Only admin can approve whitelist members. Can
// also be used to bulk add members, as non-existing
// entries will be automatically created and approved
pub fn execute_private_whitelist_approve(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: WhitelistApproveMsg,
) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;

    // Only admin can approve members
    if state.owner != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    for member in msg.whitelist_members.iter() {
        let member_approval = WhitelistMember { whitelisted: true };
        PRIVATE_WHITELIST.save(deps.storage, member, &member_approval)?;
    }

    Ok(Response::new().add_attribute("action", "private_whitelist_approve"))
}

// Helper fn to enforce minting price
pub fn check_sent_required_payment(
    sent: &[Coin],
    required: Option<Coin>,
) -> Result<(), ContractError> {
    if let Some(required_coin) = required {
        let required_amount = required_coin.amount.u128();
        if required_amount > 0 {
            let sent_sufficient_funds = sent.iter().any(|coin| {
                // check if a given sent coin matches denom
                // and has sufficient amount
                coin.denom == required_coin.denom && coin.amount.u128() >= required_amount
            });

            if sent_sufficient_funds {
                return Ok(());
            } else {
                return Err(ContractError::Unauthorized {});
            }
        }
    }
    Ok(())
}
