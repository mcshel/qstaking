use anchor_lang::prelude::*;

use crate::errors::*;


#[account]
pub struct Character { 
    
    // Bump used in generating the Character account
    pub bump: u8,
    
    // Level of the Character
    pub level: u8,
    
    // Comulative experience points
    pub experience: u64,
    
    // Base mining points
    pub mining_points: u64,
    
    // Maximum mining rewards capacity
    pub mining_capacity: u64,
    
    // Base looting points
    pub looting_points: u64,
    
    // Looting survival score
    pub looting_survival: u64,
    
    // Base sentinel points
    pub bounty_points: u64,
    
    // Number of available bounty hunt attempts
    pub bounty_bullets: u16,
    
    // Comulative amount of mined Reward Tokens
    pub mining_rewards: u128,
    
    // Comulative amount of looted Reward Tokens
    pub looting_rewards: u128,
    
    // Comulative amount of bounty Reward Tokens
    pub bounty_rewards: u128,
    
    // NFT staking status: 0 unstaked, 1 staked in mine, 2 staked in loot
    pub staked: u8,
    
    // Staking pool's commualtive accrued rewards per mining point at the moment of staking
    pub staked_peg: u128,
    
    // Timestamp when the NFT was staked
    pub staked_timestamp: i64,
    
    // Timestamp when the mine cooldown expires
    pub mine_cooldown_timestamp: i64,
    
    // Timestamp when the loot cooldown expires
    pub loot_cooldown_timestamp: i64,
    
    // Timestamp when bounty bullets were last claimed
    pub bounty_bulltes_timestamp: i64
}


impl Character {
    
    pub fn initialize(&mut self, bump: u8) -> Result<()> {
    
        self.bump = bump;
        self.level = 1;
        self.experience = 0;
        
        self.mining_points = 1;
        self.mining_capacity = 1000000;
        
        self.looting_points = 1;
        self.looting_survival = 0;
        
        self.bounty_points = 0;
        self.bounty_bullets = 0;
        
        self.mining_rewards = 0;
        self.looting_rewards = 0;
        self.bounty_rewards = 0;
        
        self.staked = 0;
        self.staked_peg = 0;
        self.staked_timestamp = 0;
        self.mine_cooldown_timestamp = 0;
        self.loot_cooldown_timestamp = 0;
        self.bounty_bulltes_timestamp = 0;
        
        Ok(())
    }
    
    
    pub fn stake_mine(&mut self, timestamp: i64, accrued_rewards: u128) -> Result<()> {
        
        self.staked = 1;
        self.staked_peg = accrued_rewards;
        self.staked_timestamp = timestamp;
        
        Ok(())
    }
    
    
    pub fn unstake_mine(&mut self, timestamp: i64, cooldown: u64, mine_pool_strength: u64, loot_pool_strength: u64, mine_accrued_rewards: u128) -> Result<(u64, u64)> {
        
        let accrued_rewards_total = self.mining_points.checked_mul(
            u64::try_from(
                mine_accrued_rewards.checked_sub(self.staked_peg).ok_or(QstakingErrors::InvalidComputation).unwrap()
            ).unwrap()
        ).ok_or(QstakingErrors::InvalidComputation).unwrap();
        
        let looted_rewards_nom = accrued_rewards_total.checked_mul(loot_pool_strength).ok_or(QstakingErrors::InvalidComputation).unwrap();
        let looted_rewards_denom = loot_pool_strength.checked_add(mine_pool_strength).ok_or(QstakingErrors::InvalidComputation).unwrap();
        let looted_rewards = looted_rewards_nom.checked_div(looted_rewards_denom).ok_or(QstakingErrors::InvalidComputation).unwrap();
        
        let accrued_rewards = std::cmp::min(
            accrued_rewards_total.checked_sub(looted_rewards).ok_or(QstakingErrors::InvalidComputation).unwrap(),
            self.mining_capacity
        );
        
        
        self.mine_cooldown_timestamp = timestamp.checked_add(
            i64::try_from(cooldown).unwrap()
        ).ok_or(QstakingErrors::InvalidComputation).unwrap();
        
        self.mining_rewards = self.mining_rewards.checked_add(
            u128::try_from(accrued_rewards).unwrap()
        ).ok_or(QstakingErrors::InvalidComputation).unwrap();
        
        //TODO: Add leveling and mining_points increase logic
        self.experience = self.experience.checked_add(
            u64::try_from(
                timestamp.checked_sub(self.staked_timestamp).ok_or(QstakingErrors::InvalidComputation).unwrap()
            ).unwrap()
        ).ok_or(QstakingErrors::InvalidComputation).unwrap();
        
        self.staked = 0;
        self.staked_timestamp = 0;
        
        return Ok((accrued_rewards, looted_rewards));
    }
    
    
    pub fn stake_loot(&mut self, timestamp: i64, accrued_rewards: u128) -> Result<()> {
        
        self.staked = 2;
        self.staked_peg = accrued_rewards;
        self.staked_timestamp = timestamp;
        
        Ok(())
    }
    
    
    pub fn unstake_loot(&mut self, timestamp: i64, cooldown: u64, loot_accrued_rewards: u128) -> Result<u64> {
        
        let accrued_rewards = self.looting_points.checked_mul(
            u64::try_from(
                loot_accrued_rewards.checked_sub(self.staked_peg).ok_or(QstakingErrors::InvalidComputation).unwrap()
            ).unwrap()
        ).ok_or(QstakingErrors::InvalidComputation).unwrap();
        
        
        self.loot_cooldown_timestamp = timestamp.checked_add(
            i64::try_from(cooldown).unwrap()
        ).ok_or(QstakingErrors::InvalidComputation).unwrap();
        
        self.looting_rewards = self.looting_rewards.checked_add(
            u128::try_from(accrued_rewards).unwrap()
        ).ok_or(QstakingErrors::InvalidComputation).unwrap();
        
        //TODO: Add leveling and mining_points increase logic
        self.experience = self.experience.checked_add(
            u64::try_from(
                timestamp.checked_sub(self.staked_timestamp).ok_or(QstakingErrors::InvalidComputation).unwrap()
            ).unwrap()
        ).ok_or(QstakingErrors::InvalidComputation).unwrap();
        
        self.staked = 0;
        self.staked_timestamp = 0;
        
        return Ok(accrued_rewards);
    }
}
