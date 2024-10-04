use anchor_lang::prelude::*;
use anchor_spl::token::{self, CloseAccount, Mint, Token, TokenAccount, Transfer};

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
    pub user: Signer<'info>,

    pub token_program: Program<'info, Token>,
}

pub fn close_token_account(ctx: Context<CloseTokenAccount>) -> Result<()> {
    let transfer_amount = ctx.accounts.pension_token_account.amount;

    // 1. 转移token
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.pension_token_account.to_account_info(),
                to: ctx.accounts.user_token_account.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        ),
        transfer_amount,
    )?;
    // 检查转账后的余额
    ctx.accounts.pension_token_account.reload()?;
    // 获取转账后的余额
    let post_transfer_balance = ctx.accounts.pension_token_account.amount;
    msg!("转账后的余额: {}", post_transfer_balance);

    if post_transfer_balance == 0 {
        // 2. 只有在余额为0时才关闭账户
        token::close_account(CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            CloseAccount {
                account: ctx.accounts.pension_token_account.to_account_info(),
                destination: ctx.accounts.user.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        ))?;
        msg!("已关闭养老金账户,并将 {} 代币转还给用户", transfer_amount);
    } else {
        msg!(
            "账户余额不为0,无法关闭。当前余额: {}",
            post_transfer_balance
        );
    }

    Ok(())
}
