use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken, 
    token::{
        Mint, 
        Token, 
        TokenAccount, 
        Transfer,
        transfer,
    }
};

use crate::state::Escrow;

#[derive(Accounts)]
pub struct Donate<'info> {
    #[account(mut)]
    pub donor: Signer<'info>,
    // CHECK: This is just used to fetch the maker's address
    pub maker: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [
            b"escrow", 
            maker.key().as_ref(),
            mint.key().as_ref(),
        ],
        bump = escrow.bump,
    )]
    pub escrow: Account<'info, Escrow>,
    #[account(
        init_if_needed,
        payer = donor, // Consider initializing the account in the maker context so that the maker can pay for the init fees
        associated_token::mint = mint,
        associated_token::authority = escrow,
    )]
    pub escrow_ata: Account<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = donor,
    )]
    pub donor_ata: Account<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = maker,
    )]
    pub maker_ata: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> Donate<'info> {
    pub fn donate(&mut self, amount: u64) -> Result<()> {
        let total_donated = self.escrow_ata.amount; // We fetch the amount of tokens that have been donated so far
        let remaining = self.escrow.target - total_donated; // We calculate the remaining amount of tokens that need to be donated

        let amount_to_transfer = match amount > remaining { // We check if the amount to donate is greater than the remaining amount
            true => remaining, // If the amount to donate is greater than the remaining amount, we donate the remaining amount
            false => amount, // Otherwise, we donate the amount that was specified
        };

        /*
        If the user donates 10% of the total amount, mint 10 reward tokens back to the donor
        For that we will need a new mint with the escrow as authority (probably init it in the make??)
        Don't forget that we are using integers (not floating point numbers)
         */

        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Transfer {
            from: self.donor_ata.to_account_info(),
            to: self.escrow_ata.to_account_info(),
            authority: self.donor.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts); // We create a CPI context to transfer the tokens from the donor's ATA to the escrow's ATA
        transfer(cpi_ctx, amount_to_transfer)?; // We transfer the tokens from the donor's ATA to the escrow's ATA

        msg!("Donation of {} tokens successful", amount_to_transfer);
        msg!("Total donated: {}", self.escrow_ata.amount);
        
        Ok(())
    }

    pub fn check_donations(&self) -> Result<()> {
        let mint = self.mint.key().clone();

        match self.escrow_ata.amount >= self.escrow.target { // We check if the escrow account has reached its target
            true => { // If the escrow account has reached its target, we transfer the tokens from the escrow's ATA to the maker's ATA
                let seeds = &[
                    b"escrow", 
                    self.maker.key.as_ref(),
                    mint.as_ref(),
                    &[self.escrow.bump],
                ];
                let signer_seeds = &[&seeds[..]];

                let cpi_program = self.token_program.to_account_info();
                let cpi_accounts = Transfer {
                    from: self.escrow_ata.to_account_info(),
                    to: self.maker_ata.to_account_info(),
                    authority: self.escrow.to_account_info(),
                };
                let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
                transfer(cpi_ctx, self.escrow_ata.amount)?;
            }
            false => msg!("The escrow account has not reached its target yet"),
        }
        
        Ok(())
    }
}