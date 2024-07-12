pub mod msgs;
pub mod error;

use cosmwasm_std::{Empty, Reply, SubMsgResult};
use cw2::{get_contract_version, set_contract_version};

pub use {{cw721_base_lib}} as cw721_base;
pub use {{cw721_lib}} as cw721;
pub use cw721_metadata::*;

pub use crate::cw721_base::{InstantiateMsg, QueryMsg};
use crate::cw721::ContractInfoResponse;
pub use error::ContractError;
pub use msgs::*;

const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub mod entry {
    use super::*;

    #[cfg(not(feature = "library"))]
    use cosmwasm_std::entry_point;
    use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
    use crate::msgs::{Cw721MetadataContract, ExecuteMsg, MigrateMsg};

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn instantiate(
        deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        msg: InstantiateMsg,
    ) -> StdResult<Response> {
        set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

        let info = ContractInfoResponse {
            name: msg.name,
            symbol: msg.symbol,
        };

        let contract = Cw721MetadataContract::default();
        contract.contract_info.save(deps.storage, &info)?;
        let minter = deps.api.addr_validate(&msg.minter)?;
        {{minter_save_snippet}}

        Ok(Response::default())
    }

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, ContractError> {
        Cw721MetadataContract::default().execute(deps, env, info, msg).map_err(|err| err.into())
    }

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn reply(_deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
        match msg.result {
            SubMsgResult::Ok(_) => Ok(Response::default()),
            SubMsgResult::Err(_) => Err(ContractError::Unauthorized {}),
        }
    }

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn query(deps: Deps, env: Env, msg: QueryMsg<Empty>) -> StdResult<Binary> {
        Cw721MetadataContract::default().query(deps, env, msg)
    }

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
        let original_version = get_contract_version(deps.storage)?;
        let name = CONTRACT_NAME.to_string();
        let version = CONTRACT_VERSION.to_string();
        if original_version.contract != name {
            return Err(ContractError::Unauthorized {});
        }
        if original_version.version >= version {
            return Err(ContractError::Unauthorized {});
        }
        set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
        Ok(Response::default())
    }
}