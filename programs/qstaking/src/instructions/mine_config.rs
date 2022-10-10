use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

use crate::program::Qstaking;
use crate::states::Mine;
// use crate::errors::QstakingErrors;


/*
 *  Initialize a new Mine.
 *  There is a hard limit of 1 Mine per Qstaking smart contract. The init() function can only be called 
 *  by the owner of the smart contract instance.
 */
 

#[derive(Accounts)]
pub struct InitMine<'info> {

    // Address of the Qstaking program
    #[account(constraint = program.programdata_address()? == Some(program_data.key()))]
    pub program: Program<'info, Qstaking>,
    
    // Address of the Qstaking program data
    #[account(constraint = program_data.upgrade_authority_address == Some(authority.key()))]
    pub program_data: Account<'info, ProgramData>,
    
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
    #[account(mut)]
    pub authority: Signer<'info>,
    
    // System program
    pub system_program: Program<'info, System>,
    
    // Rent program
    pub rent: Sysvar<'info, Rent>,
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
//     mine.bump = *ctx.bumps.get("mine").unwrap();
//     mine.locked = true;
//     mine.manager = manager;
//     mine.mint = ctx.accounts.mint.key();
//     mine.rate = 0;
//     mine.staked_miners = 0;
//     mine.staked_points = 0;
//     mine.accrued_rewards = 0;
//     mine.accrued_timestamp = 0;
        
    Ok(())
}



/*
 *  Set mine manager
 */


#[derive(Accounts)]
pub struct SetMineManager<'info> {

    // Address of the Qstaking program
    #[account(constraint = program.programdata_address()? == Some(program_data.key()))]
    pub program: Program<'info, Qstaking>,
    
    // Address of the Qstaking program data
    #[account(constraint = program_data.upgrade_authority_address == Some(authority.key()))]
    pub program_data: Account<'info, ProgramData>,
    
    // Address of the Mine
    #[account(
        mut,
        seeds = [b"mine".as_ref(),],
        bump,
    )]
    pub mine: Account<'info, Mine>,
    
    // Authority for creating the Mine -> upgrade authority of the Qstaking program
    #[account(mut)]
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
pub struct SetMineRate<'info> {

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



pub fn set_rate(ctx: Context<SetMineRate>, rate: u64) -> Result<()> {
    
    let mine = &mut ctx.accounts.mine;
    mine.update_accrued_rewards()?;
    mine.rate = rate;
    
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
    
    let mine = &mut ctx.accounts.mine;
    mine.update_accrued_rewards()?;
    
    Ok(())
}



