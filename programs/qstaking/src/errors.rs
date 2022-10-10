use anchor_lang::prelude::*;

#[error_code]
pub enum QstakingErrors {
    #[msg("Manager account is not on-curve")]
    ManagerOffCurve,
    #[msg("Invalid computation")]
    InvalidComputation,
    #[msg("The supplied metadata account is not valid")]
    InvalidMetadataAccount,
    #[msg("The supplied whitelist proof account is not valid")]
    InvalidWhitelistProof,
    #[msg("The supplied whitelist type is invalid")]
    InvalidWhitelistType,
    #[msg("The requested staking pool is currently locked")]
    StakingPoolLocked,
    #[msg("The user is not the current holder of the NFT")]
    NotHolder,
    #[msg("The NFT is not staked")]
    NotStaked,
} 
