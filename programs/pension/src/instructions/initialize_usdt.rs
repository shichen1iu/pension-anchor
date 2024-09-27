use crate::state::Pension;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer_checked, Mint, Token, TokenAccount, TransferChecked},
};

#[derive(Accounts)]
pub struct InitializeUsdt<'info> {
    #[account(
        init,
        payer = user,
        associated_token::mint = usdt,
        associated_token::authority = user,
    )]
    pub pension_usdt_token_account: Account<'info, TokenAccount>,
    #[account(
        init,
        seeds = [b"pension_userinfo".as_ref(), user.key().as_ref()],
        bump,
        space = Pension::LEN,
        payer = user
    )]
    pub pension_user_info: Account<'info, Pension>,
    #[account(mut)]
    pub user_usdt_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub usdt: Account<'info, Mint>, //usdt地址
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn initialize_usdt(
    ctx: Context<InitializeUsdt>,
    expected_lamports: u16,
    expected_year: u8,
) -> Result<()> {
    //录入信息
    let pension_user_info = &mut ctx.accounts.pension_user_info;
    pension_user_info.expected_lamports = expected_lamports;
    pension_user_info.expected_year = expected_year;
    pension_user_info.cooldown = Clock::get()?.unix_timestamp + 60 * 60 * 24 * 30;
    pension_user_info.amount = expected_lamports as u64;

    // 转账
    let cpi_accounts = TransferChecked {
        from: ctx.accounts.user_usdt_token_account.to_account_info(),
        mint: ctx.accounts.usdt.to_account_info(),
        to: ctx.accounts.pension_usdt_token_account.to_account_info(),
        authority: ctx.accounts.user.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    transfer_checked(cpi_ctx, expected_lamports as u64, 6)?;
    Ok(())
}
