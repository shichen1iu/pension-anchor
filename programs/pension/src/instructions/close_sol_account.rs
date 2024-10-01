use crate::error::PensionError;
use crate::state::Pension;
use anchor_lang::prelude::*;
#[derive(Accounts)]
pub struct CloseSolAccount<'info> {
    #[account(
        mut,
        close = user,
    )]
    pub pension_account: Account<'info, Pension>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn close_sol_account(ctx: Context<CloseSolAccount>) -> Result<()> {
    // 检查账户是否可以关闭
    if ctx.accounts.pension_account.to_account_info().lamports() == 0 {
        msg!("Pension account closed successfully");
        Ok(())
    } else {
        Err(PensionError::AccountNotCloseable.into())
    }
}
