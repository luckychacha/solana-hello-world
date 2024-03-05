use borsh::BorshDeserialize;

#[derive(BorshDeserialize)]
pub struct MovieReview {
    pub title: String,
    pub rating: u8,
    pub description: String,
}
