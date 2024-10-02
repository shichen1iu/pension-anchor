use crate::error::PensionError;
use crate::state::Pension;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CheckAndClosePension<'info> {
    #[account(
        mut,
        close = user,
    )]
    pub pension_account: Account<'info, Pension>,

    #[account(mut)]
    pub user: Signer<'info>,

    /// CHECK: This is safe because we're just transferring SOL to this account
    #[account(mut)]
    pub user_sol_wallet: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

pub fn check_sol_account(ctx: Context<CheckAndClosePension>) -> Result<()> {
    let pension_account = &mut ctx.accounts.pension_account;
    let current_time = Clock::get()?.unix_timestamp;

    // 即将关闭账户倒计时
    if (pension_account.cooldown < current_time)
        && (current_time < pension_account.cooldown + 3 * 24 * 60 * 60)
    {
        msg!(
            "距离关闭账户还有{}天",
            (pension_account.cooldown + 3 * 24 * 60 * 60 - current_time) / (24 * 60 * 60)
        );
    }

    // 检查是否已经过了冷却期的三天
    if current_time > pension_account.cooldown + 3 * 24 * 60 * 60 {
        // 转移 SOL 到用户钱包
        let sol_amount = pension_account.to_account_info().lamports();
        **pension_account
            .to_account_info()
            .try_borrow_mut_lamports()? = 0;
        **ctx.accounts.user_sol_wallet.try_borrow_mut_lamports()? += sol_amount;
    } else {
        return Err(PensionError::AccountClosureTimeNotYetReached.into());
    }

    Ok(())
}
