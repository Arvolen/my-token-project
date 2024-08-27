use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, TokenAccount, Transfer, MintTo, Burn};

declare_id!("bosuUbACQoWaF6Zj8A57tJedjFbYUU4aJkrgutcCYE5");

#[program]
pub mod my_token_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, decimals: u8) -> ProgramResult {
        let cpi_accounts = token::InitializeMint {
            mint: ctx.accounts.mint.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::initialize_mint(cpi_ctx, decimals, &ctx.accounts.authority.key(), Some(&ctx.accounts.authority.key()))?;
        Ok(())
    }

    // Additional methods for minting, transferring, burning tokens can be added here.
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, mint::decimals = 0, mint::authority = authority)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, token::Token>,
    pub system_program: Program<'info, System>,
}
