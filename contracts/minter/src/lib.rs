pub mod contract;
pub mod execute;
pub mod msg;
pub mod query;
pub mod state;

// mod data;
mod error;
mod integration_tests;
pub use crate::error::ContractError;

pub use {{crate_name}}_token as token;