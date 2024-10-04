use crate::error::PensionError;
use crate::state::Pension;
use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Mint, Token, TokenAccount, Transfer};

#[derive(Accounts)]
pub struct DepositToken<'info> {
    #[account(
        mut,
        constraint = pension_token_account.mint == usdc_usdt_mint.key() @ PensionError::InvalidTokenAccount,
        constraint = pension_token_account.owner == user.key() @ PensionError::InvalidTokenOwner
    )]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub pension_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub pension_user_info: Account<'info, Pension>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub usdc_usdt_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
}

pub fn deposit_token(ctx: Context<DepositToken>) -> Result<()> {
    let pension_user_info = &mut ctx.accounts.pension_user_info;

    // 判断冷却时间是否超过30天
    let current_timestamp = Clock::get()?.unix_timestamp;
    //test
    if current_timestamp < pension_user_info.cooldown {
        return Err(PensionError::CooldownNotExpired.into());
    }

    // 重置冷却时间
    pension_user_info.cooldown = current_timestamp + 60 * 60 * 24 * 30; // 30 days from now
                                                                        //test
                                                                        // pension_user_info.cooldown = current_timestamp; //

    // msg!(
    //     "指令:deposit_token 当前执行转账前pension_token_account的余额为:{}",
    //     ctx.accounts.pension_token_account.amount
    // );
    // msg!(
    //     "指令:deposit_token 当前执行转账前pension_user_info的amount为:{}",
    //     pension_user_info.amount
    // );

    // 构建 CPI 转账操作
    let cpi_accounts = Transfer {
        from: ctx.accounts.user_token_account.to_account_info(),
        to: ctx.accounts.pension_token_account.to_account_info(),
        authority: ctx.accounts.user.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

    // 执行转账
    transfer(
        cpi_ctx,
        pension_user_info.amount * 10u64.pow(ctx.accounts.usdc_usdt_mint.decimals as u32),
    )?;

    ctx.accounts.pension_token_account.reload()?;

    // msg!(
    //     "指令:deposit_token 当前执行转账完成后pension_token_account的余额为:{}",
    //     ctx.accounts.pension_token_account.amount
    // );

    // 更新已经存储的金额
    pension_user_info.amount += pension_user_info.expected_amount;

    // msg!(
    //     "指令:deposit_token 当前执行转账后pension_user_info的amount为:{}",
    //     pension_user_info.amount
    // );
    Ok(())
}
