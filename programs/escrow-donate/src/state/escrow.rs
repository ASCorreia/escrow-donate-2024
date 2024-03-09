use anchor_lang::prelude::*;

#[account]
pub struct Escrow {
    pub mint: Pubkey,
    pub target: u64,
    pub bump: u8,
}

impl Space for Escrow {
    const INIT_SPACE: usize = 8 + 32 + 8 + 1; // Discriminator + mint + target + bump
}