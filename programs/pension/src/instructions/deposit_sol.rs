use crate::error::PensionError;
use crate::state::Pension;
use anchor_lang::prelude::*;

use anchor_lang::system_program;

#[derive(Accounts)]
pub struct DepositSol<'info> {
    #[account(mut)]
    pub pension_account: Account<'info, Pension>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn deposit_sol(ctx: Context<DepositSol>) -> Result<()> {
    let pension_account = &mut ctx.accounts.pension_account;

    // 判断冷却时间是否超过30天
    let current_timestamp = Clock::get()?.unix_timestamp;
    if current_timestamp < pension_account.cooldown {
        return Err(PensionError::CooldownNotExpired.into());
    }

    // 重置冷却时间
    pension_account.cooldown = current_timestamp + 60 * 60 * 24 * 30; // 30 days from now

    //转账
    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.user.to_account_info(),
                to: pension_account.to_account_info(),
            },
        ),
        pension_account.expected_amount as u64,
    )?;

    Ok(())
}
