use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema, Default)]
pub enum Phase {
    #[default]
    Disabled,
    PrivateWhitelist,
    NormalWhitelist,
    Public,
    Reveal,
}

impl Phase {
    pub fn is_enabled(&self) -> bool { !self.is_disabled() }
    pub fn is_disabled(&self) -> bool { *self == Phase::Disabled }
    pub fn is_private_whitelist(&self) -> bool { *self == Phase::PrivateWhitelist }
    pub fn is_public_whitelist(&self) -> bool { *self == Phase::NormalWhitelist }
    pub fn is_public_mint(&self) -> bool { *self == Phase::Public }
    pub fn is_reveal(&self) -> bool { *self == Phase::Reveal }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema, Default)]
pub struct WhitelistMember {
    pub whitelisted: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub owner: Addr,
    pub cw721: Addr,
    pub artist: Addr,
    pub supply: u64,
    pub phase: Phase,

    pub private_whitelist_allowance: u64,
    pub public_whitelist_allowance: u64,
    pub price: Uint128,
    pub name_prefix: String,
}

impl Into<QueriedState> for State {
    fn into(self) -> QueriedState {
        QueriedState {
            owner: self.owner,
            cw721: self.cw721,
            artist: self.artist,
            supply: self.supply,

            whitelist_allowance: self.public_whitelist_allowance,
            private_whitelist_allowance: self.private_whitelist_allowance,

            total_reserved: 0,
            total_reserved_founders: 0,

            price: self.price,
            name_prefix: self.name_prefix,

            public_whitelist: self.phase.is_public_whitelist(),
            public_mint: self.phase.is_public_mint(),
            reveal: self.phase.is_reveal(),
            initialized: self.phase.is_enabled(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct QueriedState {
    pub owner: Addr,
    pub cw721: Addr,
    pub artist: Addr,
    pub supply: u64,


    pub whitelist_allowance: u64,
    pub private_whitelist_allowance: u64,

    // TODO: review later
    pub total_reserved: u64,
    pub total_reserved_founders: u64,
    
    pub price: Uint128,
    pub name_prefix: String,

    pub public_whitelist: bool,
    pub public_mint: bool,
    pub reveal: bool,
    pub initialized: bool,
}

pub const STATE: Item<State> = Item::new("state");
pub const PUBLIC_WHITELIST: Map<&Addr, WhitelistMember> = Map::new("public_whitelist");
pub const PUBLIC_WHITELIST_COUNTER: Map<&Addr, u64> = Map::new("public_whitelist_counter");
pub const PRIVATE_WHITELIST: Map<&Addr, WhitelistMember> = Map::new("private_whitelist");
pub const PRIVATE_WHITELIST_COUNTER: Map<&Addr, u64> = Map::new("private_whitelist_counter");