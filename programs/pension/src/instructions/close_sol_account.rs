use crate::state::Pension;
use anchor_lang::prelude::*;
use anchor_lang::system_program;

#[derive(Accounts)]
pub struct CloseSolAccount<'info> {
    #[account(mut)]
    pub pension_account: Account<'info, Pension>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn close_sol_account(ctx: Context<CloseSolAccount>, expected_month: u8) -> Result<()> {
    let transfer_amount = ctx.accounts.pension_account.to_account_info().lamports();
    let cpi_ctx_transfer = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        system_program::Transfer {
            from: ctx.accounts.pension_account.to_account_info(),
            to: ctx.accounts.user.to_account_info(),
        },
    );
    let remain_month = &mut ctx.accounts.pension_account.remain_month;
    if expected_month == 0 {
        *remain_month = expected_month as i16;
    }

    if *remain_month == 0 {
        // 立即转移所有资金并关闭账户
        system_program::transfer(cpi_ctx_transfer, transfer_amount)?;
        ctx.accounts
            .pension_account
            .close(ctx.accounts.user.to_account_info())?;
        msg!("已关闭养老金账户,并将 {} lamports转给用户", transfer_amount);
    } else {
        //判断是否是第一次退款,如果remain_month小于0，则表示是第一次退款
        if *remain_month < 0_i16 {
            *remain_month = expected_month as i16;
            system_program::transfer(cpi_ctx_transfer, transfer_amount / expected_month as u64)?;
            //转账结束后，更新remain_month和amount
            ctx.accounts.pension_account.remain_month -= 1;
            ctx.accounts.pension_account.amount -= transfer_amount / expected_month as u64;
            //查看转账后的余额
            ctx.accounts.pension_account.reload()?;
            // 获取转账后的余额
            let post_transfer_balance = ctx.accounts.pension_account.amount;
            msg!("转账后的养老金余额: {}", post_transfer_balance);
        } else {
            //如果不是第一次存款
            system_program::transfer(cpi_ctx_transfer, transfer_amount / *remain_month as u64)?;
            //转账结束后，更新remain_month和amount
            ctx.accounts.pension_account.remain_month -= 1;
            ctx.accounts.pension_account.amount -= transfer_amount / expected_month as u64;
            //查看转账后的余额
            ctx.accounts.pension_account.reload()?;
            // 获取转账后的余额
            let post_transfer_balance = ctx.accounts.pension_account.amount;
            msg!("转账后的养老金余额: {}", post_transfer_balance);
        }
    }

    Ok(())
}
