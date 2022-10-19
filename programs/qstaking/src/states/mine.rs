use anchor_lang::prelude::*;

use crate::errors::*;


#[account]
pub struct Mine {

    // Bump used in generating the Mine account
    pub bump: u8,
    
    // Lock controling if NFTs can be staked into the pool
    pub locked: bool,
    
    // Mine manager
    pub manager: Pubkey,
    
    // Account of the mint that is being mined
    pub mint: Pubkey,
    
    // Mining reward rate in units of Reward Tokens / mining point / s
    pub rate: u64,
    
    // Base price for staking in the Mine pool
    pub price: u64,
    
    // Cooldown period for re-staking in Mine pool
    pub cooldown: u64,
    
    // Number of staked NFTs
    pub staked_characters: u16,
    
    // Total number of staked mining points
    pub staked_points: u64,
    
    // Comulative accrued rewards per mining point
    pub accrued_rewards: u128,
    
    // Timestamp the of the last comulative accrued reward update
    pub accrued_timestamp: i64,
}

impl Mine {

    pub fn initialize(&mut self, bump: u8, manager: &Pubkey, mint: &Pubkey) -> Result<()> {
    
        self.bump = bump;
        self.locked = true;
        self.manager = *manager;
        self.mint = *mint;
        self.rate = 0;
        self.price = 0;
        self.cooldown = 0;
        self.staked_characters = 0;
        self.staked_points = 0;
        self.accrued_rewards = 0;
        self.accrued_timestamp = 0;
        
        Ok(())
    }
    

    pub fn update_accrued_rewards(&mut self, timestamp: i64) -> Result<()> {
        
        let timestamp_delta_signed: i64 = timestamp.checked_sub(self.accrued_timestamp).ok_or(QstakingErrors::InvalidComputation).unwrap();
        let timestamp_delta: u128 = u128::try_from(timestamp_delta_signed).unwrap();
        let mut newly_accrued_rewards: u128 =  u128::try_from(self.rate).unwrap();
        newly_accrued_rewards = newly_accrued_rewards.checked_mul(timestamp_delta).ok_or(QstakingErrors::InvalidComputation).unwrap();
        
        self.accrued_rewards = self.accrued_rewards.checked_add(newly_accrued_rewards).ok_or(QstakingErrors::InvalidComputation).unwrap();
        self.accrued_timestamp = timestamp;
        
        Ok(())
    }
    
    
    pub fn add_character(&mut self, timestamp: i64, mining_points: u64) -> Result<()> {
    
        self.update_accrued_rewards(timestamp)?;
        
        self.staked_characters = self.staked_characters.checked_add(1).ok_or(QstakingErrors::InvalidComputation).unwrap();
        self.staked_points = self.staked_points.checked_add(mining_points).ok_or(QstakingErrors::InvalidComputation).unwrap();
        
        Ok(())
    }
    
    
    pub fn remove_character(&mut self, timestamp: i64, mining_points: u64) -> Result<()> {
        
        self.update_accrued_rewards(timestamp)?;
        
        self.staked_characters = self.staked_characters.checked_sub(1).ok_or(QstakingErrors::InvalidComputation).unwrap();
        self.staked_points = self.staked_points.checked_sub(mining_points).ok_or(QstakingErrors::InvalidComputation).unwrap();
        
        Ok(())
    }

}
