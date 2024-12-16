use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct MovieAccountState {
    pub reviewer: Pubkey,
    pub rating: u8,
    #[max_len(20)]
    pub title: String,
    #[max_len(50)]
    pub description: String,
    pub bump: u8,
    pub mint_bump: u8,
}

pub const DISCRIMINATOR: usize = 8;
pub const MAX_RATING: u8 = 5;
pub const MIN_RATING: u8 = 1;
pub const MAX_TITLE_LENGTH: usize = 20;
pub const MAX_DESCRIPTION_LENGTH: usize = 50;
