use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount, Token};
use anchor_spl::associated_token::AssociatedToken;

use crate::program::Qstaking;
use crate::errors::QstakingErrors;
use crate::states::*;



/*
 * Initialize program's AddminSettings account and set a program admin
 */


#[derive(Accounts)]
pub struct InitAdmin<'info> {

    // AdminSettings account
    #[account(
        init,
        seeds = [b"admin".as_ref()], 
        bump, 
        payer = authority,
        space = 8 + std::mem::size_of::<AdminSettings>(),
    )]
    pub admin_settings: Account<'info, AdminSettings>,
    
    // Qstaking program
    #[account(
        constraint = program.programdata_address()? == Some(program_data.key())
    )]
    pub program: Program<'info, Qstaking>,
    
    // Qstaking program data
    #[account(
        constraint = program_data.upgrade_authority_address == Some(authority.key())
    )]
    pub program_data: Account<'info, ProgramData>,
    
    // Authority for creating the AdminSettings account -> upgrade authority of the Qstaking program
    #[account(mut)]
    pub authority: Signer<'info>,
    
    // System program
    pub system_program: Program<'info, System>,
}


pub fn init_admin(ctx: Context<InitAdmin>, admin_key: Pubkey) -> Result<()> {

    let admin_settings = &mut ctx.accounts.admin_settings;
    admin_settings.bump = *ctx.bumps.get("admin_settings").unwrap();
    admin_settings.admin_key = admin_key;
    
    Ok(())
}



/*
 *  Set admin by updating the AdminSettings account
 */


#[derive(Accounts)]
pub struct SetAdmin<'info> {

    // AdminSettings account
    #[account(
        mut,
        seeds = [b"admin".as_ref()],
        bump,
    )]
    pub admin_settings: Account<'info, AdminSettings>,
    
    // Qstaking program
    #[account(
        constraint = program.programdata_address()? == Some(program_data.key())
    )]
    pub program: Program<'info, Qstaking>,
    
    // Qstaking program data
    #[account(
        constraint = program_data.upgrade_authority_address == Some(authority.key())
    )]
    pub program_data: Account<'info, ProgramData>,
    
    // Authority for updating the AdminSettings account -> upgrade authority of the Qstaking program
    #[account(mut)]
    pub authority: Signer<'info>,
} 


pub fn set_admin(ctx: Context<SetAdmin>, admin_key: Pubkey) -> Result<()> {

    let admin_settings = &mut ctx.accounts.admin_settings;
    admin_settings.admin_key = admin_key;
    
    Ok(())
}



/*
 *  Add reference account to whitelist
 */


#[derive(Accounts)]
#[instruction(_reference_account: Pubkey)]
pub struct AddWhitelist<'info> {
    
    // AdminSettings account
    #[account(
        seeds = [b"admin".as_ref()],
        bump,
    )]
    pub admin_settings: Account<'info, AdminSettings>,
    
    // Whitelist account
    #[account(
        init,
        payer = authority,
        seeds = [b"whitelist".as_ref(), _reference_account.as_ref()],
        bump,
        space = 8 + std::mem::size_of::<Whitelist>(),
    )]
    pub whitelist: Account<'info, Whitelist>,
    
    // Staking program admin defined in AdminSettings
    #[account(
        mut,
        constraint = admin_settings.admin_key == authority.key(),
    )]
    pub authority: Signer<'info>,
    
    // System program
    pub system_program: Program<'info, System>,
}


pub fn add_whitelist(ctx: Context<AddWhitelist>, _reference_account: Pubkey, whitelist_type: u8) -> Result<()> {
    
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
    
    // AdminSettings account
    #[account(
        seeds = [b"admin".as_ref()],
        bump,
    )]
    pub admin_settings: Account<'info, AdminSettings>,
    
    // Whitelist account
    #[account(
        mut,
        seeds = [b"whitelist".as_ref(), _reference_account.as_ref()],
        bump,
        close = authority,
    )]
    pub whitelist: Account<'info, Whitelist>,
    
    // Staking program admin defined in AdminSettings
    #[account(
        mut,
        constraint = admin_settings.admin_key == authority.key(),
    )]
    pub authority: Signer<'info>,
    
    // System program
    pub system_program: Program<'info, System>,
}


pub fn remove_whitelist(_ctx: Context<RemoveWhitelist>, _reference_account: Pubkey) -> Result<()> {

    Ok(())
}



/*
 *  Initialize the Mine and Loot staking pools
 *  There is a hard limit of 1 Mine and 1 Loot pool per Qstaking smart contract. The init() function 
 *  can only be called by the the smart contract administrator.
 */
 

#[derive(Accounts)]
pub struct InitPools<'info> {
    
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
    
    // Address of the Loot
    #[account(
        init,
        payer = authority,
        seeds = [b"loot".as_ref(),],
        bump,
        space = 8 + std::mem::size_of::<Loot>()
    )]
    pub loot: Account<'info, Loot>,
    
    // Token account with loot rewards
    #[account(
        init,
        payer = authority,
        seeds = [b"proceeds".as_ref(), loot.key().as_ref()],
        bump,
        token::mint = mint,
        token::authority = loot,
    )]
    pub loot_proceeds: Account<'info, TokenAccount>,
    
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
    
    // Token program
    pub token_program: Program<'info, Token>,
    
    // System program
    pub system_program: Program<'info, System>,
    
    // Rent program
    pub rent: Sysvar<'info, Rent>,
}


pub fn init_pools(ctx: Context<InitPools>, mine_manager: Pubkey, loot_manager: Pubkey) -> Result<()> {

    /*
    require!(
        Pubkey::is_on_curve(&mine_manager),
        QstakingErrors::ManagerOffCurve
    );
    
    require!(
        Pubkey::is_on_curve(&loot_manager),
        QstakingErrors::ManagerOffCurve
    );
    */
    
    let mine = &mut ctx.accounts.mine;
    mine.initialize(*ctx.bumps.get("mine").unwrap(), &mine_manager, &ctx.accounts.mint.key())?;
    
    let loot = &mut ctx.accounts.loot;
    loot.initialize(*ctx.bumps.get("loot").unwrap(), &loot_manager)?;
        
    Ok(())
}



/*
 *  Init NFT's staking accounts (Character, Miner and Looter accounts)
 */
 

#[derive(Accounts)]
pub struct InitCharacter<'info> {
    
    // Character account of the NFT
    #[account(
        init,
        payer = user,
        seeds = [b"character".as_ref(), nft_mint.key().as_ref(),],
        bump,
        space = 8 + std::mem::size_of::<Character>(),
    )]
    pub character: Account<'info, Character>,
    
    // Whitelist account to be used for whitelist proof
    whitelist: Account<'info, Whitelist>,
    
    // Associated token account of the NFT
    #[account(
        associated_token::mint = nft_mint,
        associated_token::authority = user,
    )]
    pub nft_ata: Account<'info, TokenAccount>,
    
    // Metadata account of the NFT
    ///CHECKED: custom logic checks for the validity of this account
    pub nft_metadata: UncheckedAccount<'info>,
    
    // Mint account of the NFT
    pub nft_mint: Account<'info, Mint>,
    
    // User account that holds the NFT
    #[account(mut)]
    pub user: Signer<'info>,
    
    // Associated token program
    pub associated_token_program: Program<'info, AssociatedToken>,

    // System program
    pub system_program: Program<'info, System>,
}


pub fn init_character(ctx: Context<InitCharacter>) -> Result<()> {

    require!(
        ctx.accounts.nft_ata.amount == 1,
        QstakingErrors::NotHolder
    );
    
    let whitelist = &mut ctx.accounts.whitelist;
    whitelist.verify(&ctx.program_id, &whitelist.key(), &ctx.accounts.nft_mint.key(), &ctx.accounts.nft_metadata.to_account_info())?;
    
    let character = &mut ctx.accounts.character;
    character.initialize(*ctx.bumps.get("character").unwrap())?;
    
    Ok(())
}

