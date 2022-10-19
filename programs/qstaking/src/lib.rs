use anchor_lang::prelude::*;
use instructions::*;

pub mod instructions;
pub mod states;
pub mod errors;


declare_id!("4sjvE7PiZ5rzv6y7HxE6kTQqRrMAoERSYKv4hhwhNccb");


#[program]
pub mod qstaking {

    use super::*;
    
    
    // ----- Staking program config functions -----
    
    pub fn init_admin(ctx: Context<InitAdmin>, admin: Pubkey) -> Result<()> {
        instructions::program_config::init_admin(ctx, admin)?;
        Ok(())
    }
    
    
    pub fn set_admin(ctx: Context<SetAdmin>, admin: Pubkey) -> Result<()> {
        instructions::program_config::set_admin(ctx, admin)?;
        Ok(())
    }
    
    
    pub fn add_whitelist(ctx: Context<AddWhitelist>, reference_account: Pubkey, whitelist_type: u8) -> Result<()> {
        instructions::program_config::add_whitelist(ctx, reference_account, whitelist_type)?;
        Ok(())
    }
    
    
    pub fn remove_whitelist(ctx: Context<RemoveWhitelist>, reference_account: Pubkey) -> Result<()> {
        instructions::program_config::remove_whitelist(ctx, reference_account)?;
        Ok(())
    }
    
    
    pub fn init_pools(ctx: Context<InitPools>, mine_manager: Pubkey, loot_manager: Pubkey) -> Result<()> {
        instructions::program_config::init_pools(ctx, mine_manager, loot_manager)?;
        Ok(())
    }
    
    
    pub fn init_character(ctx: Context<InitCharacter>) -> Result<()> {
        instructions::program_config::init_character(ctx)?;
        Ok(())
    }
    
    
    // ----- Mine config functions ----

    /*
    pub fn init_mine(ctx: Context<InitMine>, manager: Pubkey) -> Result<()> {
        instructions::mine_config::init(ctx, manager)?;
        Ok(())
    }
    */    
    
    pub fn set_mine_manager(ctx: Context<SetMineManager>, manager: Pubkey) -> Result<()> {
        instructions::mine_config::set_manager(ctx, manager)?;
        Ok(())
    }
    
    
    pub fn set_mine_lock(ctx: Context<SetMineLock>, locked: bool) -> Result<()> {
        instructions::mine_config::set_locked(ctx, locked)?;
        Ok(())
    }
    
    
    pub fn set_mine_parameters(ctx: Context<SetMineParameters>, rate: u64, price: u64, cooldown: u64) -> Result<()> {
        instructions::mine_config::set_parameters(ctx, rate, price, cooldown)?;
        Ok(())
    }
    
    
    pub fn update_mine(ctx: Context<UpdateMine>) -> Result<()> {
        instructions::mine_config::update(ctx)?;
        Ok(())
    }
    
    
    // ----- Loot config functions -----
    
    pub fn set_loot_manager(ctx: Context<SetLootManager>, manager: Pubkey) -> Result<()> {
        instructions::loot_config::set_manager(ctx, manager)?;
        Ok(())
    }
    
    
    pub fn set_loot_lock(ctx: Context<SetLootLock>, locked: bool) -> Result<()> {
        instructions::loot_config::set_locked(ctx, locked)?;
        Ok(())
    }
    
    
    pub fn set_loot_parameters(ctx: Context<SetLootParameters>, duration: u64, price: u64, cooldown: u64) -> Result<()> {
        instructions::loot_config::set_parameters(ctx, duration, price, cooldown)?;
        Ok(())
    }
    
    
    pub fn update_loot(ctx: Context<UpdateLoot>) -> Result<()> {
        instructions::loot_config::update(ctx)?;
        Ok(())
    }
    
    
    // ----- Mine staking functions -----
    
    pub fn stake_mine(ctx: Context<StakeMine>) -> Result<()> {
        instructions::mine_staking::stake(ctx)?;
        Ok(())
    }
    
    
    pub fn unstake_mine(ctx: Context<UnstakeMine>) -> Result<()> {
        instructions::mine_staking::unstake(ctx)?;
        Ok(())
    }
    
    
    // ----- Loot staking functions -----
    
    pub fn stake_loot(ctx: Context<StakeLoot>) -> Result<()> {
        instructions::loot_staking::stake(ctx)?;
        Ok(())
    }
    
    
    pub fn unstake_loot(ctx: Context<UnstakeLoot>) -> Result<()> {
        instructions::loot_staking::unstake(ctx)?;
        Ok(())
    }
}
