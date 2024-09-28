use crate::state::Pension;
use anchor_lang::prelude::*;

use anchor_lang::system_program;

use super::close_sol_account::close_sol_account;

#[derive(Accounts)]
pub struct InitializeSol<'info> {
    #[account(
        init,
        payer = user,
        seeds = [b"pension_sol".as_ref(), user.key().as_ref()],
        space = Pension::LEN,
        bump,

    )]
    pub pension_account: Account<'info, Pension>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}


pub fn initialize_sol(
    ctx: Context<InitializeSol>,
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

    // let close_time = Clock::get()?.unix_timestamp + 60 * 60 * 24 * 33;


    // //等下完成::超过33天,关闭账户
    // if(Clock::get()?.unix_timestamp > close_time){

    //     let close_accounts = CloseSolAccount {
    //         pension_account: ctx.accounts.pension_account.to_account_info(),
    //         user: ctx.accounts.user.to_account_info(),
    //         system_program: ctx.accounts.system_program.to_account_info(),
    //     };
    //     let close_ctx = Context::new(&ctx.program_id, &close_accounts, &[]);
    //     close_sol_account(close_ctx)?;
    // }

    Ok(())
}
