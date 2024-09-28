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

pub fn close_sol_account(_ctx: Context<CloseSolAccount>) -> Result<()> {
    
    Ok(())
}
