use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, TokenAccount, Token};
use anchor_spl::associated_token::AssociatedToken;
use solana_program::program::invoke_signed;
use mpl_token_metadata as metaplex;
//use mpl_token_metadata::state::Metadata;

use crate::states::*;
use crate::errors::QstakingErrors; 



/*
 *  Stake-delegate an NFT
 */


#[derive(Accounts)]
pub struct StakeLoot<'info> {

    // Loot account in which to stake the NFT
    #[account(
        mut,
        seeds = [b"loot".as_ref(),],
        bump,
    )]
    pub loot: Account<'info, Loot>,
    
    // Character account of the NFT
    #[account(
        mut,
        seeds = [b"character".as_ref(), nft_mint.key().as_ref(),],
        bump,
    )]
    pub character: Account<'info, Character>,
    
    // Whitelist account to be used for whitelist proof
    #[account(mut)]
    pub whitelist: Account<'info, Whitelist>,
    
    // Associated token account of the NFT
    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = user,
    )]
    pub nft_ata: Account<'info, TokenAccount>,
    
    // Token (Master) Edition account
    ///CHECKED: custom logic checks for the validity of this account
    pub nft_edition: UncheckedAccount<'info>,
    
    // Metadata account of the NFT
    ///CHECKED: custom logic checks for the validity of this account
    pub nft_metadata: UncheckedAccount<'info>,
    
    // Mint account of the NFT
    pub nft_mint: Account<'info, Mint>,
    
    // User account that holds the NFT
    #[account(mut)]
    pub user: Signer<'info>,
    
    // Metaplex Token Metadata program
    /// CHECKED: This is not dangerous because we don't read or write from this account
    pub token_metadata_program: UncheckedAccount<'info>,
    
    // Token program
    pub token_program: Program<'info, Token>,
    
    // Associated token program
    pub associated_token_program: Program<'info, AssociatedToken>,

    // System program
    pub system_program: Program<'info, System>,
}


pub fn stake(ctx: Context<StakeLoot>) -> Result<()> {

    assert_edition_account(&ctx.accounts.nft_mint.key(), &ctx.accounts.nft_edition.to_account_info())?;
    
    let clock = Clock::get()?;
    let loot = &mut ctx.accounts.loot;
    let character = &mut ctx.accounts.character;
    let whitelist = &mut ctx.accounts.whitelist;
    
    require!(
        !loot.locked,
        QstakingErrors::StakingPoolLocked
    );
    
    require!(
        ctx.accounts.nft_ata.amount == 1,
        QstakingErrors::NotHolder
    );
    
    require!(
        character.staked == 0,
        QstakingErrors::AlreadyStaked
    );
    
    
    whitelist.verify(&ctx.program_id, &whitelist.key(), &ctx.accounts.nft_mint.key(), &ctx.accounts.nft_metadata.to_account_info())?;
    loot.add_character(clock.unix_timestamp, character.mining_points)?;
    character.stake_loot(clock.unix_timestamp, loot.accrued_rewards)?;
    
    // Add the Loot account as delegate to user's NFT ATA
    token::approve(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Approve {
                to: ctx.accounts.nft_ata.to_account_info(),
                delegate: loot.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        ),
        1,
    )?;
    
    // Freeze the user's NFT ATA
    invoke_signed(
        &metaplex::instruction::freeze_delegated_account(
            ctx.accounts.token_metadata_program.key(),
            loot.key(),
            ctx.accounts.nft_ata.key(),
            ctx.accounts.nft_edition.key(),
            ctx.accounts.nft_mint.key(),
        ),
        &[
            ctx.accounts.token_metadata_program.to_account_info(),
            loot.to_account_info(),
            ctx.accounts.nft_ata.to_account_info(),
            ctx.accounts.nft_edition.to_account_info(),
            ctx.accounts.nft_mint.to_account_info(),
        ],
        &[&[b"loot".as_ref(), &[loot.bump]]],
    )?;
    
    Ok(())
 
} 



/*
 *  Unstake-delegate an NFT
 */


#[derive(Accounts)]
pub struct UnstakeLoot<'info> {
    
    // Loot staking pool account
    #[account(
        mut,
        seeds = [b"loot".as_ref(),],
        bump,
    )]
    pub loot: Box<Account<'info, Loot>>,
    
    // Character account of the NFT
    #[account(
        mut,
        seeds = [b"character".as_ref(), nft_mint.key().as_ref(),],
        bump,
    )]
    pub character: Box<Account<'info, Character>>,
    
    // Token account with loot rewards
    #[account(
        mut,
        seeds = [b"proceeds".as_ref(), loot.key().as_ref()],
        bump,
        token::authority = loot,
    )]
    pub loot_proceeds: Box<Account<'info, TokenAccount>>,
    
    // Associated token account of the NFT
    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = user,
    )]
    pub nft_ata: Box<Account<'info, TokenAccount>>,
    
    // Token (Master) Edition account
    ///CHECKED: custom logic checks for the validity of this account
    pub nft_edition: UncheckedAccount<'info>,
    
    // Mint account of the NFT
    pub nft_mint: Box<Account<'info, Mint>>,
    
    // User's associated token account for the reward tokens
    #[account(
        mut,
        associated_token::mint = loot_proceeds.mint,
        associated_token::authority = user,
    )]
    pub reward_ata: Box<Account<'info, TokenAccount>>,
    
    // User account that holds the NFT
    #[account(mut)]
    pub user: Signer<'info>,
    
    // Metaplex Token Metadata program
    /// CHECKED: This is not dangerous because we don't read or write from this account
    pub token_metadata_program: UncheckedAccount<'info>,
    
    // Token program
    pub token_program: Program<'info, Token>,
    
    // Associated token program
    pub associated_token_program: Program<'info, AssociatedToken>,

    // System program
    pub system_program: Program<'info, System>,
}


pub fn unstake(ctx: Context<UnstakeLoot>) -> Result<()> {

    assert_edition_account(&ctx.accounts.nft_mint.key(), &ctx.accounts.nft_edition.to_account_info())?;
    
    let clock = Clock::get()?;
    let loot = &mut ctx.accounts.loot;
    let character = &mut ctx.accounts.character;
    
    require!(
        ctx.accounts.nft_ata.amount == 1,
        QstakingErrors::NotHolder
    );
    
    require!(
        character.staked == 2,
        QstakingErrors::NotStaked
    );
    
    loot.update_accrued_rewards(clock.unix_timestamp)?;
    let accrued_reward = character.unstake_loot(clock.unix_timestamp, loot.cooldown, loot.accrued_rewards)?;
    loot.remove_character(character.looting_points, accrued_reward)?;
    
    // Thaw the user's NFT ATA
    invoke_signed(
        &metaplex::instruction::thaw_delegated_account(
            ctx.accounts.token_metadata_program.key(),
            loot.key(),
            ctx.accounts.nft_ata.key(),
            ctx.accounts.nft_edition.key(),
            ctx.accounts.nft_mint.key(),
        ),
        &[
            ctx.accounts.token_metadata_program.to_account_info(),
            loot.to_account_info(),
            ctx.accounts.nft_ata.to_account_info(),
            ctx.accounts.nft_edition.to_account_info(),
            ctx.accounts.nft_mint.to_account_info(),
        ],
        &[&[b"loot".as_ref(), &[loot.bump]]],
    )?;
    
    // Remove the Loot account as delegate from user's NFT ATA
    token::revoke(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Revoke {
                source: ctx.accounts.nft_ata.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        )
    )?;
    
    // Mint the reward tokens to user's ATA
    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.loot_proceeds.to_account_info(),
                to: ctx.accounts.reward_ata.to_account_info(),
                authority: loot.to_account_info(),
            },
            &[&[b"loot".as_ref(), &[loot.bump]]],
        ),
        accrued_reward,
    )?;
    
    
    Ok(())
}



/*
 *  Utility functions
 */


pub fn assert_edition_account(mint: &Pubkey, mint_edition: &AccountInfo) -> Result<()> {
    let metadata_program = metaplex::id();
    
    require_keys_eq!(
        *mint_edition.owner,
        metadata_program,
        QstakingErrors::InvalidEditionAccount
    );
    
    let seed = &[b"metadata".as_ref(), metadata_program.as_ref(), mint.as_ref(), b"edition".as_ref()];
    let (edition_account, _bump) = Pubkey::find_program_address(seed, &metadata_program);
    require_keys_eq!(
        edition_account,
        mint_edition.key(),
        QstakingErrors::InvalidEditionAccount
    );
    
    Ok(())
}
