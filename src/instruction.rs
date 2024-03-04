use borsh::BorshDeserialize;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError, pubkey::Pubkey};

pub enum MovieInstruction {
    AddMovieReview {
        title: String,
        rating: u8,
        description: String,
    },
}

pub fn add_movie_review(
    program_id: &Pubkey,
    accounts: &[AccountInfo],title: String, rating: u8, description: String) -> ProgramResult {
    msg!("正在添加电影评论！");
    msg!("title: {}", title);
    msg!("rating: {}", rating);
    msg!("description: {}", description);
    msg!("program_id: {} accounts: {:?}", program_id, accounts);

    Ok(())
}

#[derive(BorshDeserialize)]
struct MovieReview {
    title: String,
    rating: u8,
    description: String,
}

impl MovieInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&first, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;

        let payload: MovieReview = MovieReview::try_from_slice(rest).map_err(|_| {
            msg!("Failed to deserialize MovieReview");
            ProgramError::InvalidInstructionData
        })?;

        // msg!("MovieReview unpacked: title: {}, rating: {}, description: {}", payload.title, payload.rating, payload.description);

        Ok(match first {
            0 => Self::AddMovieReview {
                title: payload.title,
                rating: payload.rating,
                description: payload.description,
            },
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}