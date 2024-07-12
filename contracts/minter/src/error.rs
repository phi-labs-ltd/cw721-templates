use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Invalid input")]
    InvalidInput {},

    #[error("Contract already initialized")]
    Initialized {},

    #[error("Minting period expired")]
    MintExpired {},

    #[error("Public whitelist minting already enabled")]
    PublicWhitelistMintEnabled {},

    #[error("Public minting already enabled")]
    PublicMintEnabled {},

    #[error("Revealing already enabled")]
    RevealEnabled {},

    #[error("Revealing not enabled")]
    RevealDisabled {},

    #[error("Error parsing source metadata or conflicting supply parameters")]
    SourceMetadata {},

    #[error("Whitelist allowance exceeded")]
    WhitelistAllowance { minted: u64 },

    #[error("Must be whitelisted to mint before public minting")]
    NotWhitelisted {},

    #[error("All tokens distributed")]
    SoldOut {},

    #[error("Metadata of token already revealed")]
    MetadataRevealed { token_id: String },

    // Keeps compat with both types of minters without changing the API
    #[error("This entrypoint is disabled and cannot be used")]
    EntrypointDisabled {}
}
