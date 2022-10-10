use anchor_lang::prelude::*;

use crate::errors::*;


#[account]
pub struct Miner { 
    
    pub bump: u8,
    
    // NFT owner
    pub owner: Pubkey,
    
    // NFTs mining points when staked
    pub points: u32,
    
    // Bonus multiplier for the mining points
    pub bonus_multiplier: f32,
    
    // Bonus expiry timestamp
    pub bonus_expiry: i64,
    
    // Maximum capacity for the accrued rewards
    pub capacity: u128,
    
    // Timestamp when staking was started
    pub stake_timestamp: i64,
    
    // Mine's commualtive accrued rewards per mining point at the moment of staking
    pub accrued_rewards_peg: u128,
}

impl Miner {

    pub fn initialize(&mut self, bump: u8, owner: &Pubkey) -> Result<()> {
        self.bump = bump;
        self.owner = *owner;
        self.points = 1;
        self.capacity = 1000 * 1000;
        self.bonus_multiplier = 1.0;
        self.bonus_expiry = 0;
        self.stake_timestamp = 0;
        self.accrued_rewards_peg = 0;
        
        Ok(())
    }

    
    pub fn stake(&mut self, accrued_rewards: u128) -> Result<()> {
        
        let clock = Clock::get()?;
        
        self.stake_timestamp = clock.unix_timestamp;
        self.accrued_rewards_peg = accrued_rewards;
        
        Ok(())
    }
    
    
    pub fn unstake(&mut self, accrued_rewards: u128) -> Result<u64> {
        
        self.stake_timestamp = 0;
        
        let accrued_rewards_diff_u64: u64 = u64::try_from(
            accrued_rewards.checked_sub(self.accrued_rewards_peg).ok_or(QstakingErrors::InvalidComputation).unwrap()
        ).unwrap();
        
        let accrued_rewards_diff_f64: f64 = accrued_rewards_diff_u64 as f64;
        let base_points_f64: f64 = f64::try_from(self.points).unwrap();
        let multiplier_f64: f64 = f64::try_from(self.bonus_multiplier).unwrap();
        let points_f64: f64 = base_points_f64 * multiplier_f64;
        
        let accrued_reward: u64 = (accrued_rewards_diff_f64 * points_f64).floor() as u64;
        
        return Ok(accrued_reward);
        
    }
}
