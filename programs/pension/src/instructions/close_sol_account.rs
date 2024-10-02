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
    let sol_amount = ctx.accounts.pension_account.to_account_info().lamports();
    msg!("已关闭养老金账户,并将 {} lamports转给用户", sol_amount);
    Ok(())
}
