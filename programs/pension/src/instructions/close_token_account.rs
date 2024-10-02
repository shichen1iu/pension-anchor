use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

#[derive(Accounts)]
pub struct CloseTokenAccount<'info> {
    #[account(
        mut,
    close=user
    )]
    pub pension_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub token_program: Program<'info, Token>,
}

pub fn close_token_account(ctx: Context<CloseTokenAccount>) -> Result<()> {
    // 1. 首先，转移所有 token 到用户的 token 账户
    let transfer_amount = ctx.accounts.pension_token_account.amount;

    let cpi_accounts_transfer = Transfer {
        from: ctx.accounts.pension_token_account.to_account_info(),
        to: ctx.accounts.user_token_account.to_account_info(),
        authority: ctx.accounts.user.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx_transfer = CpiContext::new(cpi_program.clone(), cpi_accounts_transfer);

    token::transfer(cpi_ctx_transfer, transfer_amount)?;

    // 2. 然后，关闭 pension token 账户,这一步可以通过close=user 自动完成

    msg!("Token transferred and account closed successfully");
    Ok(())
}
