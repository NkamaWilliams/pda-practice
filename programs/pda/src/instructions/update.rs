use crate::{
    error::MovieReviewError,
    state::{
        MovieAccountState, DISCRIMINATOR, MAX_DESCRIPTION_LENGTH, MAX_RATING, MAX_TITLE_LENGTH,
        MIN_RATING,
    },
};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(title: String)]
pub struct UpdateReview<'info> {
    #[account(
    mut,
    seeds = [b"review", payer.key().as_ref(), title.as_bytes()],
    bump = movie_review.bump,
    realloc = DISCRIMINATOR + MovieAccountState::INIT_SPACE,
    realloc::payer = payer,
    realloc::zero = false
  )]
    pub movie_review: Account<'info, MovieAccountState>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> UpdateReview<'info> {
    pub fn update(&mut self, title: String, description: String, rating: u8) -> Result<()> {
        require!(
            rating >= MIN_RATING && rating <= MAX_RATING,
            MovieReviewError::InvalidRating
        );
        require!(
            title.len() <= MAX_TITLE_LENGTH,
            MovieReviewError::InvalidTitle
        );
        require!(
            description.len() <= MAX_DESCRIPTION_LENGTH,
            MovieReviewError::InvalidDescription
        );

        self.movie_review.title = title;
        self.movie_review.description = description;
        self.movie_review.rating = rating;
        Ok(())
    }
}
