use crate::state::Pension;
use crate::error::PensionError;
use anchor_lang::prelude::*;
use anchor_spl::token::{transfer_checked, Mint, Token, TokenAccount, TransferChecked};

#[derive(Accounts)]
pub struct DepositUsdc<'info> {
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub pension_token_account: Account<'info, Pension>,
    #[account(mut)]
    pub pension_user_info: Account<'info, Pension>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub usdc: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
}

pub fn deposit_usdc(ctx: Context<DepositUsdc>) -> Result<()> {
    let pension_user_info = &mut ctx.accounts.pension_user_info;
    
    // 判断冷却时间是否超过30天
    let current_timestamp = Clock::get()?.unix_timestamp;
    if current_timestamp < pension_user_info.cooldown {
        return Err(PensionError::CooldownNotExpired.into()); 
    }

    // 重置冷却时间
    pension_user_info.cooldown = current_timestamp + 60 * 60 * 24 * 30; // 30 days from now

    // 更新已经存储的金额
    pension_user_info.amount += pension_user_info.expected_lamports as u64;

    // 转账
    let cpi_accounts = TransferChecked {
        from: ctx.accounts.user_token_account.to_account_info(),
        mint: ctx.accounts.usdc.to_account_info(),
        to: ctx.accounts.pension_token_account.to_account_info(),
        authority: ctx.accounts.user.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    transfer_checked(cpi_ctx, pension_user_info.expected_lamports as u64, 6)?;
    Ok(())
}
