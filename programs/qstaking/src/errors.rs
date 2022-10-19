use anchor_lang::prelude::*;

#[error_code]
pub enum QstakingErrors {

    // 6000
    #[msg("Manager account is not on-curve")]
    ManagerOffCurve,
    
    // 6001
    #[msg("Invalid computation")]
    InvalidComputation,
    
    // 6002
    #[msg("The supplied metadata account is not valid")]
    InvalidMetadataAccount,
    
    // 6003
    #[msg("The supplied edition account is not valid")]
    InvalidEditionAccount,
    
    // 6004
    #[msg("The supplied whitelist proof account is not valid")]
    InvalidWhitelistProof,
    
    // 6005
    #[msg("The supplied whitelist type is invalid")]
    InvalidWhitelistType,
    
    // 6006
    #[msg("The supplied loot duration is not greater than zero")]
    InvalidLootDuration,
    
    // 6007
    #[msg("The requested staking pool is currently locked")]
    StakingPoolLocked,
    
    // 6008
    #[msg("The user is not the current holder of the NFT")]
    NotHolder,
    
    // 6009
    #[msg("The NFT is already staked")]
    AlreadyStaked,
    
    // 6010
    #[msg("The NFT is not staked")]
    NotStaked,
} 
