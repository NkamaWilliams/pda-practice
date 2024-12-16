use anchor_lang::prelude::*;

#[error_code]
pub enum MovieReviewError {
    #[msg("Rating must be between 1 and 5")]
    InvalidRating,
    #[msg("Title too long")]
    InvalidTitle,
    #[msg("Description too long")]
    InvalidDescription,
}

// impl From<MovieReviewError> for ProgramError {
//     fn from(value: MovieReviewError) -> Self {
//         ProgramError::Custom(value as u32)
//     }
// }
