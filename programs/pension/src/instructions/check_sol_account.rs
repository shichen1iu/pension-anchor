use crate::state::Pension;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CheckSolAccount<'info> {
    #[account(
        mut,
        close = user,
        constraint = Clock::get()?.unix_timestamp > pension_account.cooldown + 60 * 60 * 24 * 3 @ ErrorCode::CooldownNotExpired
    )]
    pub pension_account: Account<'info, Pension>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn check_sol_account(ctx: Context<CheckSolAccount>) -> Result<()> {
    msg!("Pension account closed and funds returned to user");
    Ok(())
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized user")]
    UnauthorizedUser,
    #[msg("Cooldown period not yet expired")]
    CooldownNotExpired,
}
