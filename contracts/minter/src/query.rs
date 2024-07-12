use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Deps, QueryRequest, StdResult, to_json_binary, WasmQuery};
use crate::ContractError;

use crate::token::{Extension, QueryMsg as Cw721QueryMsg};
use crate::token::cw721::NftInfoResponse;

use crate::state::{State, WhitelistMember, STATE, PUBLIC_WHITELIST, QueriedState};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TokenStatus {
    pub token_id: String,
    pub token_uri: Option<String>,
    pub extension: Extension,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TokenStatuses {
    pub revealed: Vec<TokenStatus>,
    pub unrevealed: Vec<TokenStatus>,
}

pub fn query_config(deps: Deps) -> Result<QueriedState, ContractError> {
    let config: State = STATE.load(deps.storage)?;
    Ok(config.into())
}

pub fn query_whitelist_member(deps: Deps, address: Addr) -> Result<WhitelistMember, ContractError> {
    let whitelist_member = PUBLIC_WHITELIST.may_load(deps.storage, &address)?;
    let query_resp: WhitelistMember = whitelist_member.unwrap_or_default();
    Ok(query_resp)
}

/// Accepts a list of token_ids as argument and returns
/// nft info, for those nfts, separated by statuses.
/// E.g. a list of `unrevealed` tokens, and a list of
/// `revealed` tokens)
pub fn query_token_statuses(deps: Deps, token_ids: Vec<String>) -> Result<TokenStatuses, ContractError> {
    {{token_status_query}}

    // Expand to this if updatable
    let state = STATE.load(deps.storage)?;
    let mut statuses = TokenStatuses {
        revealed: vec![],
        unrevealed: vec![],
    };
    for token_id in token_ids.iter() {
        let query_msg: crate::token::QueryMsg<Extension> = Cw721QueryMsg::NftInfo {
            token_id: token_id.clone(),
        };
        let query_req = QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: state.cw721.clone().into(),
            msg: to_json_binary(&query_msg).unwrap(),
        });
        let query_resp: NftInfoResponse<Extension> = deps.querier.query(&query_req)?;

        let status = TokenStatus {
            token_id: token_id.to_string(),
            token_uri: query_resp.token_uri,
            extension: query_resp.extension,
        };

        if status.extension.is_some() {
            statuses.revealed.push(status);
        } else {
            statuses.unrevealed.push(status);
        }
    }
    Ok(statuses)
}
