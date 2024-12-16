use anchor_lang::prelude::*;

use crate::state::MovieAccountState;

#[derive(Accounts)]
#[instruction(title: String)]
pub struct DeleteReview<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
    mut,
    seeds = [b"review", payer.key().as_ref(), title.as_bytes()],
    bump = movie_review.bump,
    close = payer
  )]
    pub movie_review: Account<'info, MovieAccountState>,
    pub system_program: Program<'info, System>,
}
