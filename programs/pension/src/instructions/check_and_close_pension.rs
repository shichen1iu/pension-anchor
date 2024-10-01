use crate::state::Pension;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, CloseAccount, Token, TokenAccount};

#[derive(Accounts)]
pub struct CheckAndClosePension<'info> {
    #[account(mut)]
    pub pension: Account<'info, Pension>,

    #[account(
        mut,
        close = user,
        constraint = pension_token_account.owner == user.key(),
    )]
    pub pension_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub token_program: Program<'info, Token>,

    pub clock: Sysvar<'info, Clock>,
}

pub fn check_and_close_pension(ctx: Context<CheckAndClosePension>) -> Result<()> {
    let pension = &mut ctx.accounts.pension;
    let current_time = ctx.accounts.clock.unix_timestamp;

    // 检查是否已经过了冷却期的三天
    if current_time > pension.cooldown + 3 * 24 * 60 * 60 {
        // 关闭 pension_token_account
        let cpi_accounts = CloseAccount {
            account: ctx.accounts.pension_token_account.to_account_info(),
            destination: ctx.accounts.user.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::close_account(cpi_ctx)?;

        // 关闭 pension account
        pension.close(ctx.accounts.user.to_account_info())?;

        msg!("Pension account and token account closed successfully");
    } else {
        msg!("Cooldown period has not passed yet");
    }

    Ok(())
}