use borsh::BorshDeserialize;

#[derive(BorshDeserialize)]
pub struct MovieReview {
    title: String,
    rating: u8,
    description: String,
}