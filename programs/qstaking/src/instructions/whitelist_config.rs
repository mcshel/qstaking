use anchor_lang::prelude::*;

use crate::program::Qstaking;
use crate::states::Whitelist;
use crate::errors::QstakingErrors;


/*
 *  Add reference account to whitelist
 */


#[derive(Accounts)]
#[instruction(_reference_account: Pubkey)]
pub struct AddWhitelist<'info> {

    // Address of the Qstaking program
    #[account(constraint = program.programdata_address()? == Some(program_data.key()))]
    pub program: Program<'info, Qstaking>,
    
    // Address of the Qstaking program data
    #[account(constraint = program_data.upgrade_authority_address == Some(authority.key()))]
    pub program_data: Account<'info, ProgramData>,
    
    // Whitelist account
    #[account(
        init,
        payer = authority,
        seeds = [b"whitelist".as_ref(), _reference_account.as_ref()],
        bump,
        space = 8 + std::mem::size_of::<Whitelist>()
    )]
    pub whitelist: Account<'info, Whitelist>,
    
    // Authority for creating the Mine -> upgrade authority of the Qstaking program
    #[account(mut)]
    pub authority: Signer<'info>,
    
    // System program
    pub system_program: Program<'info, System>,
    
    // Rent program
    pub rent: Sysvar<'info, Rent>,
    
}


pub fn add(ctx: Context<AddWhitelist>, _reference_account: Pubkey, whitelist_type: u8) -> Result<()> {
    
    //TODO Add sanity checks for the reference_account
    
    require!(
        whitelist_type < 2,
        QstakingErrors::InvalidWhitelistType
    );
    
    let whitelist = &mut ctx.accounts.whitelist;
    whitelist.bump = *ctx.bumps.get("whitelist").unwrap();
    whitelist.whitelist_type = whitelist_type;
    
    Ok(())

}



/*
 *  Remove reference account from whitelist
 */


#[derive(Accounts)]
#[instruction(_reference_account: Pubkey)]
pub struct RemoveWhitelist<'info> {

    // Address of the Qstaking program
    #[account(constraint = program.programdata_address()? == Some(program_data.key()))]
    pub program: Program<'info, Qstaking>,
    
    // Address of the Qstaking program data
    #[account(constraint = program_data.upgrade_authority_address == Some(authority.key()))]
    pub program_data: Account<'info, ProgramData>,
    
    // Whitelist account
    #[account(
        mut,
        seeds = [b"whitelist".as_ref(), _reference_account.as_ref()],
        bump,
        close = authority,
    )]
    pub whitelist: Account<'info, Whitelist>,
    
    // Authority for creating the Mine -> upgrade authority of the Qstaking program
    #[account(mut)]
    pub authority: Signer<'info>,
    
    // System program
    pub system_program: Program<'info, System>,
    
}


pub fn remove(_ctx: Context<RemoveWhitelist>, _reference_account: Pubkey) -> Result<()> {

    Ok(())
    
}



