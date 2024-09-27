use crate::state::Pension;
use anchor_lang::prelude::*;

use anchor_lang::system_program;

#[derive(Accounts)]
pub struct DepositSol<'info> {
    #[account(
        init,
        payer = user,
        seeds = [b"pension_sol".as_ref(), user.key().as_ref()],
        space = Pension::LEN,
        bump
    )]
    pub pension_account: Account<'info, Pension>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}


pub fn initialize_sol(
    ctx: Context<DepositSol>,
    expected_lamports: u16,
    expected_year: u8,
) -> Result<()> {
    let pension_account = &mut ctx.accounts.pension_account;

    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.user.to_account_info(),
                to: pension_account.to_account_info(),
            },
        ),
        expected_lamports as u64,
    )?;

    pension_account.expected_lamports = expected_lamports;
    pension_account.expected_year = expected_year;
    pension_account.cooldown = Clock::get()?.unix_timestamp + 60 * 60 * 24 * 30;

    Ok(())
}
