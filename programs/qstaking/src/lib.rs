use anchor_lang::prelude::*;
use instructions::*;
// use states::*;
// use errors::*;

pub mod instructions;
pub mod states;
pub mod errors;


declare_id!("4sjvE7PiZ5rzv6y7HxE6kTQqRrMAoERSYKv4hhwhNccb");


#[program]
pub mod qstaking {

    use super::*;
    
    
    // ----- Whitelist config functions -----
    
    pub fn add_whitelist(ctx: Context<AddWhitelist>, reference_account: Pubkey, whitelist_type: u8) -> Result<()> {
        instructions::whitelist_config::add(ctx, reference_account, whitelist_type)?;
        Ok(())
    }
    
    pub fn remove_whitelist(ctx: Context<RemoveWhitelist>, reference_account: Pubkey) -> Result<()> {
        instructions::whitelist_config::remove(ctx, reference_account)?;
        Ok(())
    }
    
    // ----- Mine config functions ----

    pub fn init_mine(ctx: Context<InitMine>, manager: Pubkey) -> Result<()> {
        instructions::mine_config::init(ctx, manager)?;
        Ok(())
    }
    
    pub fn set_mine_manager(ctx: Context<SetMineManager>, manager: Pubkey) -> Result<()> {
        instructions::mine_config::set_manager(ctx, manager)?;
        Ok(())
    }
    
    pub fn set_mine_lock(ctx: Context<SetMineLock>, locked: bool) -> Result<()> {
        instructions::mine_config::set_locked(ctx, locked)?;
        Ok(())
    }
    
    pub fn set_mine_rate(ctx: Context<SetMineRate>, rate: u64) -> Result<()> {
        instructions::mine_config::set_rate(ctx, rate)?;
        Ok(())
    }
    
    pub fn update_mine(ctx: Context<UpdateMine>) -> Result<()> {
        instructions::mine_config::update(ctx)?;
        Ok(())
    }
    
    
    // ----- Mine staking functions -----
    
    pub fn create_miner(ctx: Context<CreateMiner>) -> Result<()> {
        instructions::mine_staking::create(ctx)?;
        Ok(())
    }
    
    pub fn stake_miner(ctx: Context<StakeMiner>) -> Result<()> {
        instructions::mine_staking::stake(ctx)?;
        Ok(())
    }
    
    pub fn unstake_miner(ctx: Context<UnstakeMiner>) -> Result<()> {
        instructions::mine_staking::unstake(ctx)?;
        Ok(())
    }
    
    /*
    */
}
