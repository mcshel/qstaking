use anchor_lang::prelude::*;

use crate::errors::*;


#[account]
pub struct Mine {

    pub bump: u8,
    
    // lock controling if NFTs can be staked into the pool
    pub locked: bool,
    
    // mine manager
    pub manager: Pubkey,
    
    // account of the mint that is being mined
    pub mint: Pubkey,
    
    // mining reward rate in terms of tokens / mining point / s
    pub rate: u64,
    
    // number of staked NFTs
    pub staked_miners: u16,
    
    // total number of staked mining points
    pub staked_points: u64,
    
    // comulative accrued rewards per mining point
    pub accrued_rewards: u128,
    
    // timestamp when the comulative accrued reward were logged
    pub accrued_timestamp: i64,
}

impl Mine {

    pub fn initialize(&mut self, bump: u8, manager: &Pubkey, mint: &Pubkey) -> Result<()> {
    
        self.bump = bump;
        self.locked = true;
        self.manager = *manager;
        self.mint = *mint;
        self.rate = 0;
        self.staked_miners = 0;
        self.staked_points = 0;
        self.accrued_rewards = 0;
        self.accrued_timestamp = 0;
        
        Ok(())
    }
    

    pub fn update_accrued_rewards(&mut self) -> Result<()> {
    
        let clock = Clock::get()?;
        
        let timestamp: i64 = clock.unix_timestamp;
        let timestamp_delta_signed: i64 = timestamp.checked_sub(self.accrued_timestamp).ok_or(QstakingErrors::InvalidComputation).unwrap();
        let timestamp_delta: u128 = u128::try_from(timestamp_delta_signed).unwrap();
        let mut newly_accrued_rewards: u128 =  u128::try_from(self.rate).unwrap();
        newly_accrued_rewards = newly_accrued_rewards.checked_mul(timestamp_delta).ok_or(QstakingErrors::InvalidComputation).unwrap();
        
        self.accrued_rewards = self.accrued_rewards.checked_add(newly_accrued_rewards).ok_or(QstakingErrors::InvalidComputation).unwrap();
        self.accrued_timestamp = timestamp;
        
        Ok(())
    }
    
    
    pub fn add_miner(&mut self, miner_points: u32) -> Result<()> {
    
        self.update_accrued_rewards()?;
    
        self.staked_miners = self.staked_miners.checked_add(1).ok_or(QstakingErrors::InvalidComputation).unwrap();
        self.staked_points = self.staked_points.checked_add(u64::try_from(miner_points).unwrap()).ok_or(QstakingErrors::InvalidComputation).unwrap();
        
        Ok(())
    }
    
    
    pub fn remove_miner(&mut self, miner_points: u32) -> Result<()> {
        
        self.update_accrued_rewards()?;
        
        self.staked_miners = self.staked_miners.checked_sub(1).ok_or(QstakingErrors::InvalidComputation).unwrap();
        self.staked_points = self.staked_points.checked_sub(u64::try_from(miner_points).unwrap()).ok_or(QstakingErrors::InvalidComputation).unwrap();
        
        Ok(())
    }

}
