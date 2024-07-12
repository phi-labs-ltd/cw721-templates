use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Uint128};

use crate::state::State;

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// pub struct InstantiateMsg {
//     pub cw721: Addr,
//     pub supply: u64,
//     pub whitelist_allowance: u64,
//     pub whitelist_members: Vec<Addr>,
//     pub total_reserved: u64,
//     pub total_reserved_founders: u64,
//     pub reserved_recipient: Addr,
//     pub price: Uint128,
//     pub naming_prefix: String,
// }

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub cw721: Addr,
    pub supply: u64,
    pub public_whitelist_allowance: u64,
    pub public_whitelist_members: Vec<Addr>,
    pub private_whitelist_allowance: u64,
    pub private_whitelist_members: Vec<Addr>,
    pub reserved_recipient: Addr,
    pub price: Uint128,
    pub naming_prefix: String,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Mint(MintMsg),
    Reveal(RevealMsg), // Only works for updatable
    // WhitelistAdd(WhitelistAddMsg), // Added members must be approved by admin
    // Artist only
    Withdraw(WithdrawMsg),
    // Admin only
    Initialize(InitMsg),
    EnableNormalWhitelist(EnableWhitelistMintMsg),
    EnablePublicMint(EnablePublicMintMsg),
    EnableReveal(EnableRevealMsg), // Only works for updatable

    PrivateWhitelistApprove(WhitelistApproveMsg), // Bulk approve WLM members
    PrivateWhitelistRemove(WhitelistRemoveMsg),   // Bulk remove WLM members
    PublicWhitelistApprove(WhitelistApproveMsg), // Bulk approve WLM members
    PublicWhitelistRemove(WhitelistRemoveMsg),   // Bulk remove WLM members

    UpdateConfig { config: State },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Config {},
    TokenStatuses { token_ids: Vec<String> }, // Only works for updatable
    Whitelist { address: Addr },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct MigrateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MintMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct EnableRevealMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct EnableWhitelistMintMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct EnablePublicMintMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RevealMsg {
    pub token_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct WithdrawMsg {
    pub amount: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {}

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// pub struct WhitelistAddMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct WhitelistApproveMsg {
    pub whitelist_members: Vec<Addr>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct WhitelistRemoveMsg {
    pub whitelist_members: Vec<Addr>,
}
