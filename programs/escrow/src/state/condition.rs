use anchor_lang::prelude::*;

#[account]
pub struct Condition {
    pub content: String, // "Paczka zostala wyslana"
    pub payer: Pubkey,
    pub recipient: Pubkey,
}

