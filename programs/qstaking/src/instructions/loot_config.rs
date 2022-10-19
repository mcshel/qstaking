use anchor_lang::prelude::*;

use crate::states::*;
use crate::errors::QstakingErrors;



/*
 *  Set Loot pool manager
 */


#[derive(Accounts)]
pub struct SetLootManager<'info> {
    
    // AdminSettings account
    #[account(
        seeds = [b"admin".as_ref()],
        bump,
    )]
    pub admin_settings: Account<'info, AdminSettings>,
    
    // Address of the Loot pool
    #[account(
        mut,
        seeds = [b"loot".as_ref(),],
        bump,
    )]
    pub loot: Account<'info, Loot>,
    
    // Authority for updating the Loot account manager -> staking program admin
    #[account(
        mut,
        constraint = admin_settings.admin_key == authority.key(),
    )]
    pub authority: Signer<'info>,
}


pub fn set_manager(ctx: Context<SetLootManager>, manager: Pubkey) -> Result<()> {

    /*
    require!(
        Pubkey::is_on_curve(&manager),
        QstakingErrors::ManagerOffCurve
    );
    */
    
    let loot = &mut ctx.accounts.loot;
    loot.manager = manager;
        
    Ok(())
}



/*
 *  Set the Loot pool lock.
 */
 

#[derive(Accounts)]
pub struct SetLootLock<'info> {

    // Address of the Loot staking pool
    #[account(
        mut,
        seeds = [b"loot".as_ref(),],
        bump,
    )]
    pub loot: Account<'info, Loot>,
    
    // Manager of the Loot staking pool
    #[account(
        mut,
        constraint = loot.manager == manager.key()
    )]
    pub manager: Signer<'info>,
}


pub fn set_locked(ctx: Context<SetLootLock>, locked: bool) -> Result<()> {
    
    let loot = &mut ctx.accounts.loot;
    loot.locked = locked;
    
    Ok(())
}



/*
 *  Set the Loot pool parameters
 */

 
#[derive(Accounts)]
pub struct SetLootParameters<'info> {

    // Address of the Loot pool
    #[account(
        mut,
        seeds = [b"loot".as_ref(),],
        bump,
    )]
    pub loot: Account<'info, Loot>,
    
    // Manager of the Loot pool
    #[account(
        mut,
        constraint = loot.manager == manager.key()
    )]
    pub manager: Signer<'info>,
}



pub fn set_parameters(ctx: Context<SetLootParameters>, duration: u64, price: u64, cooldown: u64) -> Result<()> {
    
    require!(
        duration > 0,
        QstakingErrors::InvalidLootDuration
    );
    
    let clock = Clock::get()?;
    let loot = &mut ctx.accounts.loot;
    
    loot.duration = duration;
    loot.price = price;
    loot.cooldown = cooldown;
    
    loot.update_accrued_rewards(clock.unix_timestamp)?;
    loot.recompute_rate()?;
    
    Ok(())
}


/*
 *  Update Loot staking pool's accrued rewards
 */
 
 
#[derive(Accounts)]
pub struct UpdateLoot<'info> {

    // Address of the Loot staking pool
    #[account(
        mut,
        seeds = [b"loot".as_ref(),],
        bump,
    )]
    pub loot: Account<'info, Loot>,
}



pub fn update(ctx: Context<UpdateLoot>) -> Result<()> {

    let clock = Clock::get()?;
    let loot = &mut ctx.accounts.loot;
    
    loot.update_accrued_rewards(clock.unix_timestamp)?;
    
    Ok(())
}



