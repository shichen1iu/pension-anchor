use crate::instructions::close_sol_account::*;
use crate::state::Pension;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CheckSolAccount<'info> {
    #[account(
        mut,
        close = user,
    )]
    pub pension_account: Account<'info, Pension>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn check_sol_account(ctx: Context<CheckSolAccount>) -> Result<()> {
    let pension_account = &ctx.accounts.pension_account;
    let current_time = Clock::get()?.unix_timestamp;

    // 检查是否超过冷却期三天
    if current_time > pension_account.cooldown + 3 * 24 * 60 * 60 {
        // 如果超过冷却期，调用 close_sol_account 指令
        let close_accounts = &mut CloseSolAccount {
            pension_account: ctx.accounts.pension_account.clone(),
            user: ctx.accounts.user.clone(),
            system_program: ctx.accounts.system_program.clone(),
        };
        let close_ctx = Context::new(
            ctx.program_id,
            close_accounts,
            ctx.remaining_accounts,
            ctx.bumps,
        );
        close_sol_account(close_ctx)?;
    } else {
        msg!("Cooldown period has not passed yet");
    }

    Ok(())
}
