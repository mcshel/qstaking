use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, TokenAccount, Token};
use anchor_spl::associated_token::AssociatedToken;
use metaplex_token_metadata::state::Metadata;

use crate::states::*;
use crate::errors::QstakingErrors; 



/*
 * Create Miner account for NFT
 */


#[derive(Accounts)]
pub struct CreateMiner<'info> {
    
    // Miner account of the NFT
    #[account(
        init,
        payer = user,
        seeds = [b"miner".as_ref(), nft_mint.key().as_ref(),],
        bump,
        space = 8 + std::mem::size_of::<Miner>(),
    )]
    pub miner: Account<'info, Miner>,
    
    // Whitelist account to be used for whitelist proof
    #[account(mut)]
    whitelist: Account<'info, Whitelist>,
    
    // Associated token account of the NFT
    #[account(
        mut,
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
    
    // Rent program
    pub rent: Sysvar<'info, Rent>,
    
}


pub fn create(ctx: Context<CreateMiner>) -> Result<()> {

    let whitelist = &mut ctx.accounts.whitelist;
    whitelist.verify(&ctx.program_id, &whitelist.key(), &ctx.accounts.nft_mint.key(), &ctx.accounts.nft_metadata.to_account_info())?;
    
    require!(
        ctx.accounts.nft_ata.amount == 1,
        QstakingErrors::NotHolder
    );
    
    let miner = &mut ctx.accounts.miner;
    miner.initialize(*ctx.bumps.get("miner").unwrap(), &ctx.accounts.user.key())?;
    
    Ok(())
}


/*
 *  Stake an NFT
 */


#[derive(Accounts)]
pub struct StakeMiner<'info> {

    // Mine account in which to stake the NFT
    #[account(
        mut,
        seeds = [b"mine".as_ref(),],
        bump,
    )]
    mine: Account<'info, Mine>,
    
    // Miner account of the NFT
    #[account(
        mut,
        seeds = [b"miner".as_ref(), nft_mint.key().as_ref(),],
        bump,
    )]
    pub miner: Account<'info, Miner>,
    
    // Whitelist account to be used for whitelist proof
    #[account(mut)]
    whitelist: Account<'info, Whitelist>,
    
    // Associated token account of the NFT
    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = user,
    )]
    pub nft_source_ata: Account<'info, TokenAccount>,
    
    // Token account in the staking pool
    #[account(
        init,
        payer = user,
        seeds = [nft_mint.key().as_ref(),],
        bump,
        token::mint = nft_mint,
        token::authority = mine,
    )]
    pub nft_destination_ata: Account<'info, TokenAccount>,
    
    // Metadata account of the NFT
    ///CHECKED: custom logic checks for the validity of this account
    pub nft_metadata: UncheckedAccount<'info>,
    
    // Mint account of the NFT
    pub nft_mint: Account<'info, Mint>,
    
    // User account that holds the NFT
    #[account(mut)]
    pub user: Signer<'info>,
    
    // Token program
    pub token_program: Program<'info, Token>,
    
    // Associated token program
    pub associated_token_program: Program<'info, AssociatedToken>,

    // System program
    pub system_program: Program<'info, System>,
    
    // Rent program
    pub rent: Sysvar<'info, Rent>,
    
}


pub fn stake(ctx: Context<StakeMiner>) -> Result<()> {

    let mine = &mut ctx.accounts.mine;
    let miner = &mut ctx.accounts.miner;
    let whitelist = &mut ctx.accounts.whitelist;
    
    require!(
        !mine.locked,
        QstakingErrors::StakingPoolLocked
    );
    
    require!(
        ctx.accounts.nft_source_ata.amount == 1,
        QstakingErrors::NotHolder
    );
    
    whitelist.verify(&ctx.program_id, &whitelist.key(), &ctx.accounts.nft_mint.key(), &ctx.accounts.nft_metadata.to_account_info())?;
    mine.add_miner(miner.points)?;
    miner.stake(mine.accrued_rewards)?;
    
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.nft_source_ata.to_account_info(),
                to: ctx.accounts.nft_destination_ata.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        ),
        1,
    )?;
    
    Ok(())
 
}



/*
 *  Unstake an NFT
 */


#[derive(Accounts)]
pub struct UnstakeMiner<'info> {

    // Mine account from which to unstake the NFT
    #[account(
        mut,
        seeds = [b"mine".as_ref(),],
        bump,
    )]
    mine: Box<Account<'info, Mine>>,
    
    // Miner account of the NFT
    #[account(
        mut,
        seeds = [b"miner".as_ref(), nft_mint.key().as_ref(),],
        bump,
        constraint = miner.owner == user.key(),
    )]
    pub miner: Box<Account<'info, Miner>>,
    
    // Associated token account of the NFT
    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = user,
    )]
    pub nft_destination_ata: Box<Account<'info, TokenAccount>>,
    
    // Token account in the staking pool
    #[account(
        mut,
        seeds = [nft_mint.key().as_ref(),],
        bump,
        token::mint = nft_mint,
        token::authority = mine,
        //close = user,
    )]
    pub nft_source_ata: Box<Account<'info, TokenAccount>>,
    
    // Mint account of the NFT
    pub nft_mint: Box<Account<'info, Mint>>,
    
    // Associated token account for the reward tokens
    #[account(
        mut,
        associated_token::mint = reward_mint,
        associated_token::authority = user,
    )]
    pub reward_ata: Box<Account<'info, TokenAccount>>,
    
    // Address of the reward mint
    #[account(
        mut,
        mint::authority = mine.key()
    )]
    pub reward_mint: Box<Account<'info, Mint>>,
    
    // User account that holds the NFT
    #[account(mut)]
    pub user: Signer<'info>,
    
    // Token program
    pub token_program: Program<'info, Token>,
    
    // Associated token program
    pub associated_token_program: Program<'info, AssociatedToken>,

    // System program
    pub system_program: Program<'info, System>,
    
    // Rent program
    pub rent: Sysvar<'info, Rent>,
}


pub fn unstake(ctx: Context<UnstakeMiner>) -> Result<()> {

    msg!("Starting unstake function");
    
    require!(
        ctx.accounts.nft_source_ata.amount == 1,
        QstakingErrors::NotStaked
    );
    
    let mine = &mut ctx.accounts.mine;
    let miner = &mut ctx.accounts.miner;
    
    mine.remove_miner(miner.points)?;
    
    msg!("Mine updated");
    
    let accrued_reward = miner.unstake(mine.accrued_rewards)?;
    
    msg!("Miner updated");
    
    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.nft_source_ata.to_account_info(),
                to: ctx.accounts.nft_destination_ata.to_account_info(),
                authority: mine.to_account_info(),
            },
            &[&[b"mine".as_ref(), &[mine.bump]]],
        ),
        1,
    )?;
    
    msg!("NFT transfer done");
    
    token::mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::MintTo {
                mint: ctx.accounts.reward_mint.to_account_info(),
                to: ctx.accounts.reward_ata.to_account_info(),
                authority: mine.to_account_info(),
            },
            &[&[b"mine".as_ref(), &[mine.bump]]],
        ),
        accrued_reward,
    )?;
    
    msg!("Reward transfer done");
                
    
    Ok(())
 
}
