use anchor_lang::prelude::*;

declare_id!("2tnyb8MzoaEWbk1assJkF2h8rNrqcwemXorfm4DF2sm6");

pub mod error;
mod instructions;
mod state;

use instructions::*;
#[program]
pub mod pda {
    use super::*;

    pub fn init_token_mint(ctx: Context<InitializeMint>) -> Result<()> {
        msg!("Token mint initialized");
        Ok(())
    }

    pub fn initialize(
        ctx: Context<CreateReview>,
        title: String,
        description: String,
        rating: u8,
    ) -> Result<()> {
        ctx.accounts.init(
            title,
            description,
            rating,
            ctx.bumps.movie_review,
            ctx.bumps.mint,
        )?;
        Ok(())
    }

    pub fn update(
        ctx: Context<UpdateReview>,
        title: String,
        description: String,
        rating: u8,
    ) -> Result<()> {
        ctx.accounts.update(title, description, rating)?;
        Ok(())
    }

    pub fn delete(ctx: Context<DeleteReview>, title: String) -> Result<()> {
        msg!("Movie review for {} deleted", title);
        Ok(())
    }
}
