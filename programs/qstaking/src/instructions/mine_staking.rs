use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, TokenAccount, Token};
use anchor_spl::associated_token::AssociatedToken;
use solana_program::program::invoke_signed;
use mpl_token_metadata as metaplex;
//use mpl_token_metadata::state::Metadata;

use crate::states::*;
use crate::errors::QstakingErrors; 



/*
 * Create Miner account for NFT


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
 */

 

/*
 *  Stake an NFT



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
 */


/*
 *  Unstake an NFT


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
    
    require!(
        ctx.accounts.nft_source_ata.amount == 1,
        QstakingErrors::NotStaked
    );
    
    let mine = &mut ctx.accounts.mine;
    let miner = &mut ctx.accounts.miner;
    mine.remove_miner(miner.points)?;
    let accrued_reward = miner.unstake(mine.accrued_rewards)?;
    
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
    
    Ok(())
 
}
 */



/*
 *  Stake-delegate an NFT
 */


#[derive(Accounts)]
pub struct StakeMine<'info> {

    // Mine account in which to stake the NFT
    #[account(
        mut,
        seeds = [b"mine".as_ref(),],
        bump,
    )]
    pub mine: Account<'info, Mine>,
    
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


pub fn stake(ctx: Context<StakeMine>) -> Result<()> {

    assert_edition_account(&ctx.accounts.nft_mint.key(), &ctx.accounts.nft_edition.to_account_info())?;
    
    let clock = Clock::get()?;
    let mine = &mut ctx.accounts.mine;
    let character = &mut ctx.accounts.character;
    let whitelist = &mut ctx.accounts.whitelist;
    
    require!(
        !mine.locked,
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
    mine.add_character(clock.unix_timestamp, character.mining_points)?;
    character.stake_mine(clock.unix_timestamp, mine.accrued_rewards)?;
    
    // Add the Mine account as delegate to user's NFT ATA
    token::approve(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Approve {
                to: ctx.accounts.nft_ata.to_account_info(),
                delegate: mine.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        ),
        1,
    )?;
    
    // Freeze the user's NFT ATA
    invoke_signed(
        &metaplex::instruction::freeze_delegated_account(
            ctx.accounts.token_metadata_program.key(),
            mine.key(),
            ctx.accounts.nft_ata.key(),
            ctx.accounts.nft_edition.key(),
            ctx.accounts.nft_mint.key(),
        ),
        &[
            ctx.accounts.token_metadata_program.to_account_info(),
            mine.to_account_info(),
            ctx.accounts.nft_ata.to_account_info(),
            ctx.accounts.nft_edition.to_account_info(),
            ctx.accounts.nft_mint.to_account_info(),
        ],
        &[&[b"mine".as_ref(), &[mine.bump]]],
    )?;
    
    Ok(())
 
} 



/*
 *  Unstake-delegate an NFT
 */


#[derive(Accounts)]
pub struct UnstakeMine<'info> {

    // Mine staking pool account
    #[account(
        mut,
        seeds = [b"mine".as_ref(),],
        bump,
    )]
    pub mine: Box<Account<'info, Mine>>,
    
    // Loot staking pool account
    #[account(
        mut,
        seeds = [b"loot".as_ref(),],
        bump,
    )]
    pub loot: Box<Account<'info, Loot>>,
    
    // Miner account of the NFT
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
        token::mint = reward_mint,
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
        mint::authority = mine.key(),
        constraint = mine.mint == reward_mint.key(),
    )]
    pub reward_mint: Box<Account<'info, Mint>>,
    
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


pub fn unstake(ctx: Context<UnstakeMine>) -> Result<()> {

    assert_edition_account(&ctx.accounts.nft_mint.key(), &ctx.accounts.nft_edition.to_account_info())?;
    
    let clock = Clock::get()?;
    let mine = &mut ctx.accounts.mine;
    let loot = &mut ctx.accounts.loot;
    let character = &mut ctx.accounts.character;
    
    require!(
        ctx.accounts.nft_ata.amount == 1,
        QstakingErrors::NotHolder
    );
    
    require!(
        character.staked == 1,
        QstakingErrors::NotStaked
    );
    
    
    let mine_pool_strength = u64::try_from(mine.staked_characters).unwrap().checked_add(20).ok_or(QstakingErrors::InvalidComputation).unwrap();
    let loot_pool_strength = u64::try_from(loot.staked_characters).unwrap().checked_add(5).ok_or(QstakingErrors::InvalidComputation).unwrap();
    
    mine.remove_character(clock.unix_timestamp, character.mining_points)?;
    let (accrued_reward, looted_reward) = character.unstake_mine(
        clock.unix_timestamp,
        mine.cooldown,
        mine_pool_strength,
        loot_pool_strength,
        mine.accrued_rewards
    )?;
    loot.add_funds(clock.unix_timestamp, looted_reward)?;
    
    // Thaw the user's NFT ATA
    invoke_signed(
        &metaplex::instruction::thaw_delegated_account(
            ctx.accounts.token_metadata_program.key(),
            mine.key(),
            ctx.accounts.nft_ata.key(),
            ctx.accounts.nft_edition.key(),
            ctx.accounts.nft_mint.key(),
        ),
        &[
            ctx.accounts.token_metadata_program.to_account_info(),
            mine.to_account_info(),
            ctx.accounts.nft_ata.to_account_info(),
            ctx.accounts.nft_edition.to_account_info(),
            ctx.accounts.nft_mint.to_account_info(),
        ],
        &[&[b"mine".as_ref(), &[mine.bump]]],
    )?;
    
    // Remove the Mine account as delegate from user's NFT ATA
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
    
    
    // Mint the looted reward tokens to loot proceeds account
    token::mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::MintTo {
                mint: ctx.accounts.reward_mint.to_account_info(),
                to: ctx.accounts.loot_proceeds.to_account_info(),
                authority: mine.to_account_info(),
            },
            &[&[b"mine".as_ref(), &[mine.bump]]],
        ),
        looted_reward,
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
