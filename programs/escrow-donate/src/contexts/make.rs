use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken, 
    token::{
        Mint, 
        Token, 
        TokenAccount,
    }
};
use crate::state::Escrow;

#[derive(Accounts)]
pub struct Make<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    pub mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = maker,
        associated_token::mint = mint,
        associated_token::authority = maker,
    )]
    pub maker_ata: Account<'info, TokenAccount>,
    #[account(
        init,
        payer = maker,
        seeds = [
            b"escrow", 
            maker.key().as_ref(),
            mint.key().as_ref(),
        ],
        bump,
        space = Escrow::INIT_SPACE,
    )]
    pub escrow: Account<'info, Escrow>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> Make<'info> {
    pub fn make(&mut self, amount: u64, bumps: &MakeBumps) -> Result<()> {
        self.escrow.mint = self.mint.key();
        self.escrow.target = amount;
        self.escrow.bump = bumps.escrow;

        msg!("Escrow account created");
        msg!("Escrow Mint: {:?}", self.escrow.mint);
        msg!("Escrow Target: {:?}", self.escrow.target);

        Ok(())
    }
}