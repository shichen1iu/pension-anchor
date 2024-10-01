use crate::state::Pension;
use anchor_lang::prelude::*;
use clockwork_sdk::state::{Thread, ThreadAccount};
use super::check_sol_account;
use anchor_lang::system_program;

#[derive(Accounts)]
#[instruction(thread_id: Vec<u8>)]
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

    /// 添加clockwork_program
    #[account(address = clockwork_sdk::ID)]
    pub clockwork_program: Program<'info, clockwork_sdk::ThreadProgram>,

    /// 创建的Clockwork线程的地址
    #[account(mut, address = Thread::pubkey(thread_authority.key(), thread_id))]
    pub thread: SystemAccount<'info>,

    /// 管理相对应Clockwork线程的PDA账户
    #[account(seeds = [THREAD_AUTHORITY_SEED], bump)]
    pub thread_authority: SystemAccount<'info>,
}

pub fn initialize_sol(
    ctx: Context<InitializeSol>,
    expected_lamports: u16,
    expected_year: u8,
    thread_id: Vec<u8>,
) -> Result<()> {
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
        expected_lamports as u64,
    )?;

    let current_time = Clock::get()?.unix_timestamp;

    pension_account.expected_amount = expected_lamports;
    pension_account.expected_year = expected_year;
    pension_account.cooldown = current_time + 60 * 60 * 24 * 30;

    // 创建 Clockwork 线程
    let thread = &ctx.accounts.thread;
    let thread_authority = &ctx.accounts.thread_authority;

    let bump = *ctx.bumps.get("thread_authority").unwrap();
    let seeds = &[THREAD_AUTHORITY_SEED, &[bump]];
    let signer_seeds = &[&seeds[..]];

    let target_ix = Instruction {
        program_id: ctx.program_id,
        accounts: vec![
            AccountMeta::new(pension_account.key(), false),
            AccountMeta::new(user.key(), true),
            AccountMeta::new_readonly(system_program.key(), false),
        ],
        data: check_sol_account::ID.to_vec(),
    };

    clockwork_sdk::cpi::thread_create(
        CpiContext::new_with_signer(
            ctx.accounts.clockwork_program.to_account_info(),
            clockwork_sdk::cpi::ThreadCreate {
                payer: user.to_account_info(),
                system_program: system_program.to_account_info(),
                thread: thread.to_account_info(),
                authority: thread_authority.to_account_info(),
            },
            signer_seeds,
        ),
        thread_id,
        format!("Pension Check for {}", user.key()),
        target_ix.into(),
        thread::Trigger::Cron {
            schedule: "0 0 * * *".into(), // 每天午夜执行
            skippable: true,
        },
    )?;

    Ok(())
}
