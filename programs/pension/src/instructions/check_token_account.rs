use crate::error::PensionError;
use crate::state::Pension;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, CloseAccount, Mint, Token, TokenAccount, Transfer};

#[derive(Accounts)]
pub struct CheckTokenAccount<'info> {
    #[account(
        mut,
    close=user
    )]
    pub pension_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,

    pub pension_user_info: Account<'info, Pension>,

    pub usdc_usdt_mint: Account<'info, Mint>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub token_program: Program<'info, Token>,
}

pub fn check_token_account(ctx: Context<CheckTokenAccount>) -> Result<()> {
    let pension_user_info = &mut ctx.accounts.pension_user_info;
    let current_time = Clock::get()?.unix_timestamp;

    // 即将关闭账户倒计时
    if (pension_user_info.cooldown < current_time)
        && (current_time < pension_user_info.cooldown + 3 * 24 * 60 * 60)
    {
        msg!(
            "距离关闭账户还有{}天",
            (pension_user_info.cooldown + 3 * 24 * 60 * 60 - current_time) / (24 * 60 * 60)
        );
    }
    if current_time > pension_user_info.cooldown + 3 * 24 * 60 * 60 {
        // 1. 首先，转移所有 token 到用户的 token 账户
        let transfer_amount = ctx.accounts.pension_token_account.amount;

        let cpi_accounts_transfer = Transfer {
            from: ctx.accounts.pension_token_account.to_account_info(),
            to: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx_transfer = CpiContext::new(cpi_program.clone(), cpi_accounts_transfer);

        token::transfer(
            cpi_ctx_transfer,
            transfer_amount * 10u64.pow(ctx.accounts.usdc_usdt_mint.decimals as u32),
        )?;

        // 2. 然后，关闭 pension token 账户
        token::close_account(CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            CloseAccount {
                account: ctx.accounts.pension_token_account.to_account_info(),
                destination: ctx.accounts.user.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        ))?;
        msg!(
            "已关自动闭养老金账户,并将 {} 代币转还给用户",
            transfer_amount
        );
    } else {
        return Err(PensionError::AccountClosureTimeNotYetReached.into());
    }
    Ok(())
}
