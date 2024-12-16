use crate::{
    error::MovieReviewError,
    state::{
        MovieAccountState, DISCRIMINATOR, MAX_DESCRIPTION_LENGTH, MAX_RATING, MAX_TITLE_LENGTH,
        MIN_RATING,
    },
};
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{mint_to, Mint, MintTo, Token, TokenAccount};

#[derive(Accounts)]
#[instruction(title: String)]
pub struct CreateReview<'info> {
    #[account(
      init,
      payer = payer,
      space = DISCRIMINATOR + MovieAccountState::INIT_SPACE,
      seeds = [b"review", payer.key().as_ref(), title.as_bytes()],
      bump
    )]
    pub movie_review: Account<'info, MovieAccountState>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        mut,
        seeds = [b"mint"],
        bump
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = payer
    )]
    pub token_account: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> CreateReview<'info> {
    pub fn init(
        &mut self,
        title: String,
        description: String,
        rating: u8,
        bump: u8,
        mint_bump: u8,
    ) -> Result<()> {
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
        self.movie_review.reviewer = self.payer.key();
        self.movie_review.bump = bump;
        self.movie_review.mint_bump = mint_bump;

        let instruction = MintTo {
            authority: self.payer.to_account_info(),
            mint: self.mint.to_account_info(),
            to: self.token_account.to_account_info(),
        };
        let seeds = &["mint".as_bytes(), &[self.movie_review.mint_bump]];
        let signer_seeds = &[&seeds[..]];
        let ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            instruction,
            signer_seeds,
        );
        mint_to(ctx, 10 * (10 ^ 6))?;
        msg!("Minted tokens");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeMint<'info> {
    #[account(
        init,
        seeds = ["mint".as_bytes()],
        bump,
        payer = user,
        mint::decimals = 6,
        mint::authority = user
    )]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    // pub rent: Sysvar<'info, Rent>,
}
