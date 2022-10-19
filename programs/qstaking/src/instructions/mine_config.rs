use anchor_lang::prelude::*;

use crate::states::*;
// use crate::errors::QstakingErrors;


/*
 *  Initialize a new Mine.
 *  There is a hard limit of 1 Mine per Qstaking smart contract. The init() function can only be called 
 *  by the owner of the smart contract instance.
 

#[derive(Accounts)]
pub struct InitMine<'info> {
    
    // AdminSettings account
    #[account(
        seeds = [b"admin".as_ref()],
        bump,
    )]
    pub admin_settings: Account<'info, AdminSettings>,
    
    // Address of the Mine
    #[account(
        init,
        payer = authority,
        seeds = [b"mine".as_ref(),],
        bump,
        space = 8 + std::mem::size_of::<Mine>()
    )]
    pub mine: Account<'info, Mine>,
    
    // Address of the reward mint
    #[account(
        mint::authority = mine.key()
    )]
    pub mint: Account<'info, Mint>,
    
    // Authority for creating the Mine -> upgrade authority of the Qstaking program
    #[account(
        mut,
        constraint = admin_settings.admin_key == authority.key(),
    )]
    pub authority: Signer<'info>,
    
    // System program
    pub system_program: Program<'info, System>,
}


pub fn init(ctx: Context<InitMine>, manager: Pubkey) -> Result<()> {

    /*
    require!(
        Pubkey::is_on_curve(&manager),
        QstakingErrors::ManagerOffCurve
    );
    */
    
    let mine = &mut ctx.accounts.mine;
    mine.initialize(*ctx.bumps.get("mine").unwrap(), &manager, &ctx.accounts.mint.key())?;
        
    Ok(())
}
 */



/*
 *  Set mine manager
 */


#[derive(Accounts)]
pub struct SetMineManager<'info> {
    
    // AdminSettings account
    #[account(
        seeds = [b"admin".as_ref()],
        bump,
    )]
    pub admin_settings: Account<'info, AdminSettings>,
    
    // Address of the Mine
    #[account(
        mut,
        seeds = [b"mine".as_ref(),],
        bump,
    )]
    pub mine: Account<'info, Mine>,
    
    // Authority for creating the Mine -> upgrade authority of the Qstaking program
    #[account(
        mut,
        constraint = admin_settings.admin_key == authority.key(),
    )]
    pub authority: Signer<'info>,
}


pub fn set_manager(ctx: Context<SetMineManager>, manager: Pubkey) -> Result<()> {

    /*
    require!(
        Pubkey::is_on_curve(&manager),
        QstakingErrors::ManagerOffCurve
    );
    */
    
    let mine = &mut ctx.accounts.mine;
    mine.manager = manager;
        
    Ok(())
}



/*
 *  Set the mine lock.
 */
 

#[derive(Accounts)]
pub struct SetMineLock<'info> {

    // Address of the Mine
    #[account(
        mut,
        seeds = [b"mine".as_ref(),],
        bump,
    )]
    pub mine: Account<'info, Mine>,
    
    // Manager of the Mine
    #[account(
        mut,
        constraint = mine.manager == manager.key()
    )]
    pub manager: Signer<'info>,
}


pub fn set_locked(ctx: Context<SetMineLock>, locked: bool) -> Result<()> {
    
    let mine = &mut ctx.accounts.mine;
    mine.locked = locked;
    
    Ok(())
}



/*
 *  Set the mine.
 */

 
#[derive(Accounts)]
pub struct SetMineParameters<'info> {

    // Address of the Mine
    #[account(
        mut,
        seeds = [b"mine".as_ref(),],
        bump,
    )]
    pub mine: Account<'info, Mine>,
    
    // Manager of the Mine
    #[account(
        mut,
        constraint = mine.manager == manager.key()
    )]
    pub manager: Signer<'info>,
}



pub fn set_parameters(ctx: Context<SetMineParameters>, rate: u64, price: u64, cooldown: u64) -> Result<()> {
    
    let clock = Clock::get()?;
    let mine = &mut ctx.accounts.mine;
    
    mine.update_accrued_rewards(clock.unix_timestamp)?;
    mine.rate = rate;
    mine.price = price;
    mine.cooldown = cooldown;
    
    Ok(())
}


/*
 *  Update mine's accrued rewards
 */
 
 
#[derive(Accounts)]
pub struct UpdateMine<'info> {

    // Address of the Mine
    #[account(
        mut,
        seeds = [b"mine".as_ref(),],
        bump,
    )]
    pub mine: Account<'info, Mine>,
}



pub fn update(ctx: Context<UpdateMine>) -> Result<()> {

    let clock = Clock::get()?;
    let mine = &mut ctx.accounts.mine;
    
    mine.update_accrued_rewards(clock.unix_timestamp)?;
    
    Ok(())
}



