use crate::state::Pension;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, CloseAccount, Token, TokenAccount};

#[derive(Accounts)]
pub struct CloseTokenAccount<'info> {
    #[account(mut)]
    pub pension_user_info: Account<'info, Pension>,

    #[account(
        mut,
        close = user,
        constraint = pension_token_account.owner == user.key() ,
    )]
    pub pension_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub token_program: Program<'info, Token>,
}

pub fn close_pension_token_account(ctx: Context<CloseTokenAccount>) -> Result<()> {
    // 关闭 pension_token_account
    let cpi_accounts = CloseAccount {
        account: ctx.accounts.pension_token_account.to_account_info(),
        destination: ctx.accounts.user.to_account_info(),
        authority: ctx.accounts.user.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::close_account(cpi_ctx)?;

    msg!("Token account closed successfully");
    Ok(())
}
