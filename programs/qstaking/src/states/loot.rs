use anchor_lang::prelude::*;

use crate::errors::*;


#[account]
pub struct Loot {

    // Bump used in generating the Loot account
    pub bump: u8,
    
    // Lock controling if NFTs can be staked into the pool
    pub locked: bool,
    
    // Loot manager
    pub manager: Pubkey,
    
    // Total reward fund
    pub fund: u128,
    
    // Duration ower which the reward fund will be distributed in units of s
    pub duration: u64,
    
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

impl Loot {

    pub fn initialize(&mut self, bump: u8, manager: &Pubkey) -> Result<()> {
    
        self.bump = bump;
        self.locked = true;
        self.manager = *manager;
        self.fund = 0;
        self.duration = 0;
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
        let available_rewards = self.fund.checked_div(
            u128::try_from(self.staked_points).unwrap()
        ).ok_or(QstakingErrors::InvalidComputation).unwrap();
        newly_accrued_rewards = std::cmp::min(newly_accrued_rewards, available_rewards);
        
        //self.fund = self.fund.checked_sub(newly_accrued_rewards).ok_or(QstakingErrors::InvalidComputation).unwrap();
        self.accrued_rewards = self.accrued_rewards.checked_add(newly_accrued_rewards).ok_or(QstakingErrors::InvalidComputation).unwrap();
        self.accrued_timestamp = timestamp;
        
        Ok(())
    }
    
    pub fn add_funds(&mut self, timestamp: i64, amount: u64) -> Result<()> {
    
        self.update_accrued_rewards(timestamp)?;
        
        self.fund = self.fund.checked_add(
            u128::try_from(amount).unwrap()
        ).ok_or(QstakingErrors::InvalidComputation).unwrap();
        
        self.recompute_rate()?;
        
        Ok(())
    }
    
    pub fn add_character(&mut self, timestamp: i64, points: u64) -> Result<()> {
    
        self.update_accrued_rewards(timestamp)?;
        
        self.staked_characters = self.staked_characters.checked_add(1).ok_or(QstakingErrors::InvalidComputation).unwrap();
        self.staked_points = self.staked_points.checked_add(points).ok_or(QstakingErrors::InvalidComputation).unwrap();
        
        self.rate = u64::try_from(
            self.fund.checked_div(
                u128::try_from(self.duration).unwrap()
            ).ok_or(QstakingErrors::InvalidComputation).unwrap().checked_div(
                u128::try_from(self.staked_points).unwrap()
            ).ok_or(QstakingErrors::InvalidComputation).unwrap()
        ).unwrap();
        
        Ok(())
    }
    
    
    pub fn remove_character(&mut self, points: u64, reward: u64) -> Result<()> {
        
        self.staked_characters = self.staked_characters.checked_sub(1).ok_or(QstakingErrors::InvalidComputation).unwrap();
        self.staked_points = self.staked_points.checked_sub(points).ok_or(QstakingErrors::InvalidComputation).unwrap();
        
        self.fund = self.fund.checked_sub(
            u128::try_from(reward).unwrap()
        ).ok_or(QstakingErrors::InvalidComputation).unwrap();
        
        self.recompute_rate()?;
        
        Ok(())
    }
    
    
    pub fn recompute_rate(&mut self) -> Result<()> {
        if self.staked_points > 0 {
            self.rate = u64::try_from(
                self.fund.checked_div(
                    u128::try_from(self.duration).unwrap()
                ).ok_or(QstakingErrors::InvalidComputation).unwrap().checked_div(
                    u128::try_from(self.staked_points).unwrap()
                ).ok_or(QstakingErrors::InvalidComputation).unwrap()
            ).unwrap();
        } else {
            self.rate = 0;
        }
        
        Ok(())
    }

}
