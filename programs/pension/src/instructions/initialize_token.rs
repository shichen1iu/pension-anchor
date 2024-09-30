use std::str::FromStr;

use crate::error::PensionError;
use crate::state::Pension;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Mint, Token, TokenAccount, Transfer},
};

#[derive(Accounts)]
pub struct InitializeToken<'info> {
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = usdc_usdt_mint, // 代币 mint 地址（USDC 或 USDT）
        associated_token::authority = user,
    )]
    pub pension_token_account: Account<'info, TokenAccount>, // 创建并关联到 mint 的 TokenAccount

    #[account(
        init,
        seeds = [b"pension_userinfo".as_ref(), user.key().as_ref()],
        bump,
        space = Pension::LEN,
        payer = user
    )]
    pub pension_user_info: Account<'info, Pension>, // Pension 用户信息

    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>, // 用户的 TokenAccount

    #[account(mut)]
    pub user: Signer<'info>, // 用户签名者

    pub usdc_usdt_mint: Account<'info, Mint>, // USDC 或 USDT 的 Mint 地址

    pub token_program: Program<'info, Token>, // Token 程序引用

    pub system_program: Program<'info, System>, // 系统程序引用

    pub associated_token_program: Program<'info, AssociatedToken>, // AssociatedToken 程序
}

pub fn initialize_token(
    ctx: Context<InitializeToken>,
    expected_amount: u16, // 存款金额
    expected_year: u8,    // 期望年份
) -> Result<()> {
    // 判断用户传入的是否是usdc/usdt
    let usdc = Pubkey::from_str("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v").unwrap();
    let usdt = Pubkey::from_str("Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB").unwrap();

    if ctx.accounts.usdc_usdt_mint.key() != usdc && ctx.accounts.usdc_usdt_mint.key() != usdt {
        return Err(PensionError::InvalidTokenMint.into());
    }

    // 获取当前的用户 Pension 信息并更新
    let pension_user_info = &mut ctx.accounts.pension_user_info;
    pension_user_info.expected_amount = expected_amount;
    pension_user_info.expected_year = expected_year;
    pension_user_info.cooldown = Clock::get()?.unix_timestamp + 60 * 60 * 24 * 30; // 30天的冷却期
    pension_user_info.amount = expected_amount as u64;

    // 构建 CPI 转账操作
    let cpi_accounts = Transfer {
        from: ctx.accounts.user_token_account.to_account_info(),
        to: ctx.accounts.pension_token_account.to_account_info(),
        authority: ctx.accounts.user.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

    // 执行转账，单位为 lamports（记得根据代币的小数位调整）
    transfer(cpi_ctx, expected_amount as u64)?;

    Ok(())
}
