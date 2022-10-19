// use bitflags::bitflags;
use mpl_token_metadata as metaplex;
//use metaplex_token_metadata::state::Metadata;
use anchor_lang::prelude::*;

use crate::errors::*;

// bitflags! {
//     pub struct WhitelistType: u8 {
//         const MINT = 1 << 0;
//         const CREATOR = 1 << 1;
//     }
// }


#[account]
pub struct Whitelist {

    // Bump used in generating the Whitelist account
    pub bump: u8,
    
    // whitelist type
    pub whitelist_type: u8,
    
}


impl Whitelist {

    fn assert_metadata(&self, mint: &Pubkey, mint_metadata: &AccountInfo) -> Result<()> {
        let metadata_program = metaplex::id();
        
        require_keys_eq!(
            *mint_metadata.owner,
            metadata_program,
            QstakingErrors::InvalidMetadataAccount
        );
        
        let seed = &[b"metadata".as_ref(), metadata_program.as_ref(), mint.as_ref(),];
        let (metadata_account, _bump) = Pubkey::find_program_address(seed, &metadata_program);
        require_keys_eq!(
            metadata_account,
            mint_metadata.key(),
            QstakingErrors::InvalidMetadataAccount
        );
        
        Ok(())
    }
    
    fn assert_whitelist(&self, seed_account: &Pubkey, program_account: &Pubkey, whitelist: &Pubkey) -> Result<()> {
        let seed = &[b"whitelist".as_ref(), seed_account.as_ref(),];
        let (whitelist_account, _bump) = Pubkey::find_program_address(seed, program_account);
        
        //msg!("Comparing whitelist accounts: {} vs {}", whitelist_account, *whitelist);
        require_keys_eq!(
            whitelist_account,
            *whitelist,
            QstakingErrors::InvalidWhitelistProof
        );
        
        Ok(())
    }

    pub fn verify(&self, program_account: &Pubkey, whitelist: &Pubkey, mint: &Pubkey, mint_metadata: &AccountInfo) -> Result<()> {
    
        self.assert_metadata(mint, mint_metadata)?;
        
        if self.whitelist_type == 0 {
            //msg!("Checking mint proof");
            self.assert_whitelist(&mint, &program_account, &whitelist)?;
            return Ok(())
            
        } else if self.whitelist_type == 1 {
            //msg!("Checking creator proof");
            //let metadata = metaplex::state::Metadata::from_account_info(mint_metadata)?;
            let metadata: metaplex::state::Metadata = metaplex::state::TokenMetadataAccount::from_account_info(mint_metadata)?;
            for creator in &metadata.data.creators.unwrap() {
            
                if !creator.verified {
                    continue;
                }
                
                match self.assert_whitelist(&creator.address, &program_account, &whitelist) {
                    Ok(()) => return Ok(()),
                    Err(_e) => continue,
                }
            }
        }
        
        Err(error!(QstakingErrors::InvalidWhitelistProof))
    }

}

