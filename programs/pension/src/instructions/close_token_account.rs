use crate::state::Pension;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, CloseAccount, Token, TokenAccount, Transfer};
#[derive(Accounts)]
pub struct CloseTokenAccount<'info> {
    #[account(
        mut,
        constraint = pension_token_account.owner == user.key()
    )]
    pub pension_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub pension_user_info: Account<'info, Pension>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub token_program: Program<'info, Token>,
}

pub fn close_token_account(ctx: Context<CloseTokenAccount>, expected_month: u8) -> Result<()> {
    //获取总共存放的养老金金额
    let transfer_amount = ctx.accounts.pension_token_account.amount;
    let cpi_ctx_transfer = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.pension_token_account.to_account_info(),
            to: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        },
    );
    let cpi_ctx_close = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        CloseAccount {
            account: ctx.accounts.pension_token_account.to_account_info(),
            destination: ctx.accounts.user.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        },
    );

    // 如果为0，则立即转移所有资金并关闭账户
    if expected_month == 0 {
        // 1. 转移token
        token::transfer(cpi_ctx_transfer, transfer_amount)?;
        //2.关闭账户
        token::close_account(cpi_ctx_close)?;
        msg!("已关闭养老金账户,并将 {} 代币转还给用户", transfer_amount);
    } else {
        //如果不为0
        let remain_month = ctx.accounts.pension_user_info.remain_month;
        //判断是否是第一次退款 ,如果remain_month小于0，则表示是第一次退款
        if remain_month < 0_i16 {
            ctx.accounts.pension_user_info.remain_month = expected_month as i16;
            token::transfer(cpi_ctx_transfer, transfer_amount / expected_month as u64)?;
            //转账结束后，更新remain_month和amount
            ctx.accounts.pension_user_info.remain_month -= 1;
            ctx.accounts.pension_user_info.amount -= transfer_amount / expected_month as u64;
            //查看转账后的余额
            ctx.accounts.pension_token_account.reload()?;
            // 获取转账后的余额
            let post_transfer_balance = ctx.accounts.pension_token_account.amount;
            msg!("转账后的养老金余额: {}", post_transfer_balance);
        } else {
            //如果不是第一次退款
            let remain_month = ctx.accounts.pension_user_info.remain_month;
            token::transfer(cpi_ctx_transfer, transfer_amount / remain_month as u64)?;
            //转账结束后，更新remain_month和amount
            ctx.accounts.pension_user_info.remain_month -= 1;
            ctx.accounts.pension_user_info.amount -= transfer_amount / expected_month as u64;
            // 检查转账后的余额
            ctx.accounts.pension_token_account.reload()?;
            // 获取转账后的余额
            let post_transfer_balance = ctx.accounts.pension_token_account.amount;
            msg!("转账后的养老金余额: {}", post_transfer_balance);
        }
    }

    Ok(())
}
