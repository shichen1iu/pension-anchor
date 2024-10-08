use crate::state::Pension;
use anchor_lang::prelude::*;

use anchor_lang::system_program;

#[derive(Accounts)]
pub struct InitializeSol<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + std::mem::size_of::<Pension>(),
        seeds = [b"pension_sol", user.key().as_ref()],
        bump
    )]
    pub pension_account: Account<'info, Pension>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn initialize_sol(
    ctx: Context<InitializeSol>,
    expected_lamports: u64,
    expected_year: u8,
) -> Result<()> {
    msg!("Initializing pension account");

    let pension_account = &mut ctx.accounts.pension_account;
    let user = &ctx.accounts.user;
    let system_program = &ctx.accounts.system_program;

    system_program::transfer(
        CpiContext::new(
            system_program.to_account_info(),
            system_program::Transfer {
                from: user.to_account_info(),
                to: pension_account.to_account_info(),
            },
        ),
        expected_lamports * 10u64.pow(6),
    )?;

    pension_account.expected_amount = expected_lamports;
    pension_account.expected_year = expected_year;
    pension_account.amount = expected_lamports as u64;
    pension_account.cooldown = Clock::get()?.unix_timestamp + 60 * 60 * 24 * 30;
    //test
    // pension_account.cooldown = Clock::get()?.unix_timestamp ;

    Ok(())
}
