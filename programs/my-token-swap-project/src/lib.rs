#![feature(build_hasher_simple_hash_one)]
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, MintTo, Burn, Token, TokenAccount, Transfer};
mod error; // This imports your error.rs file

use error::TokenSwapError;

declare_id!("BijeGR4HQSugsyn3pAKEf7J3SXqT6wxzWukH8nR7cJvX");

#[program]
pub mod my_token_swap_project {
    use super::*;

    pub fn deposit_x_receive_y(ctx: Context<DepositXReceiveY>, amount: u64) -> ProgramResult {
        ctx.accounts.process(amount)
    }

    pub fn burn_y_receive_x(ctx: Context<BurnYReceiveX>, amount: u64) -> ProgramResult {
        ctx.accounts.process(amount)
    }
}

#[derive(Accounts)]
pub struct DepositXReceiveY<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub x_mint: Account<'info, Mint>,
    #[account(mut)]
    pub y_mint: Account<'info, Mint>,
    #[account(mut)]
    pub user_x_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_y_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub vault_x_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct BurnYReceiveX<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub x_mint: Account<'info, Mint>,
    #[account(mut)]
    pub y_mint: Account<'info, Mint>,
    #[account(mut)]
    pub user_x_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_y_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub vault_x_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}



impl<'info> DepositXReceiveY<'info> {
    fn process(&self, amount: u64) -> Result<(), ProgramError> {
        if amount == 0 {
            return Err(TokenSwapError::InsufficientFunds.into());
        }
        // Transfer Token X from user to vault
        let cpi_accounts = Transfer {
            from: self.user_x_account.to_account_info(),
            to: self.vault_x_account.to_account_info(),
            authority: self.user.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);
        token::transfer(cpi_ctx, amount).map_err(|_| TokenSwapError::TransferFailed)?;


        // Mint Token Y to user
        let seeds = &[b"mint_y", &[self.y_mint.nonce]];
        let signer = &[&seeds[..]];
        let cpi_accounts = MintTo {
            mint: self.y_mint.to_account_info(),
            to: self.user_y_account.to_account_info(),
            authority: self.y_mint.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(self.token_program.to_account_info(), cpi_accounts, signer);
        token::mint_to(cpi_ctx, amount)?;

        Ok(())
    }
}

impl<'info> BurnYReceiveX<'info> {
    fn process(&self, amount: u64) -> Result<(), ProgramError> {
        // Ensure the amount is not zero
        if amount == 0 {
            return Err(TokenSwapError::InsufficientFunds.into());
        }

        // Burn Token Y from user
        let cpi_accounts_burn = Burn {
            mint: self.y_mint.to_account_info(),
            to: self.user_y_account.to_account_info(),
            authority: self.user.to_account_info(),
        };
        let cpi_ctx_burn = CpiContext::new(self.token_program.to_account_info(), cpi_accounts_burn);
        token::burn(cpi_ctx_burn, amount).map_err(|_| TokenSwapError::BurnFailed)?;

        // Transfer Token X from vault to user
        let cpi_accounts_transfer = Transfer {
            from: self.vault_x_account.to_account_info(),
            to: self.user_x_account.to_account_info(),
            authority: self.vault_x_account.to_account_info(),
        };
        let seeds = &[b"vault_x", &[self.vault_x_account.nonce]];
        let signer = &[&seeds[..]];
        let cpi_ctx_transfer = CpiContext::new_with_signer(self.token_program.to_account_info(), cpi_accounts_transfer, signer);
        token::transfer(cpi_ctx_transfer, amount).map_err(|_| TokenSwapError::TransferFailed)?;

        Ok(())
    }
}
