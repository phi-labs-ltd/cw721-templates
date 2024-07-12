use cosmwasm_std::Empty;
use cw721_metadata::Metadata;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct MigrateMsg {}

pub type Extension = Option<Metadata>;

pub type Cw721MetadataContract<'a> = crate::cw721_base::Cw721Contract<'a, Extension, Empty, Empty, Empty>;
pub type ExecuteMsg = crate::cw721_base::ExecuteMsg<Extension, Empty>;