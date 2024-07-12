use cosmwasm_std::StdError;
use thiserror::Error;

use crate::cw721_base::ContractError as Cw721Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error(transparent)]
    Std(#[from] StdError),

    #[error(transparent)]
    Cw721(#[from] Cw721Error),

    #[error("Address cannot do this")]
    Unauthorized {}
}
