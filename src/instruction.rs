use crate::types::{review::MovieReview, state::MovieAccountState};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo}, borsh1::try_from_slice_unchecked, entrypoint::ProgramResult, msg, program::invoke_signed, program_error::ProgramError, pubkey::Pubkey, rent::Rent, system_instruction, system_program, sysvar::Sysvar
};

pub fn add_movie_review(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    title: String,
    rating: u8,
    description: String,
) -> ProgramResult {

    msg!("正在添加电影评论！");
    msg!("title: {}", title);
    msg!("rating: {}", rating);
    msg!("description: {}", description);
    
    let account_info_iter = &mut accounts.iter();

    let initializer = next_account_info(account_info_iter)?;
    let pda_account = next_account_info(account_info_iter)?;
    let system_program_info = next_account_info(account_info_iter)?;
    
    // Input: Seed includes Intializer's Pubkey and title of Movie
    // Output: a valid [program derived address][pda] and its corresponding bump seed.
    let (pda, bump_seed) = Pubkey::find_program_address(&[initializer.key.as_ref(), title.as_bytes().as_ref()], program_id);

    // Calculate needed Account space
    let account_len = 1 + 1 + (4 + title.len()) + (4 + description.len());

    // Calculate Rent
    let rent = Rent::get()?;
    let rent_lamports = rent.minimum_balance(account_len);

    // TODO: Deal Result without unwrap.
    let account_len: u64 = account_len.try_into().unwrap();
    // Create PDA
    invoke_signed(&system_instruction::create_account(initializer.key, pda_account.key, rent_lamports, account_len, program_id),
    &[initializer.clone(), pda_account.clone(), system_program_info.clone()], &[&[initializer.key.as_ref(), title.as_bytes().as_ref(), &[bump_seed]]]);

    msg!("unpack State Account");
    let mut account_data = try_from_slice_unchecked::<MovieAccountState>(&pda_account.data.borrow())?;
    msg!("Borrow Account Data");

    account_data.title = title;
    account_data.rating = rating;
    account_data.description = description;
    account_data.is_initialized = true;

    msg!("Serialize State Account");
    account_data.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;
    msg!("State Account Serialized");

    Ok(())
}

pub enum MovieInstruction {
    AddMovieReview {
        title: String,
        rating: u8,
        description: String,
    },
}

impl MovieInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&first, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;

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
