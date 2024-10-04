use anchor_lang::prelude::*;

pub mod error;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("BAYLCVPy2rmpxbC3okEbbR3PV5Peo4XKGncygMzmxtCC");

#[program]
pub mod pension {
    use anchor_lang::solana_program::entrypoint::ProgramResult;

    use super::*;
    // 初始化sol养老金账户
    pub fn initialize_sol(
        ctx: Context<InitializeSol>,
        expected_lamports: u64,
        expected_year: u8,
    ) -> ProgramResult {
        instructions::initialize_sol(ctx, expected_lamports, expected_year)?;
        Ok(())
    }

    // 初始化usdt/usdc养老金账户
    pub fn initialize_token(
        ctx: Context<InitializeToken>,
        expected_lamports: u64,
        expected_year: u8,
    ) -> ProgramResult {
        instructions::initialize_token(ctx, expected_lamports, expected_year)?;
        Ok(())
    }

    //存入sol到养老金账户
    pub fn deposit_sol(ctx: Context<DepositSol>) -> ProgramResult {
        instructions::deposit_sol(ctx)?;
        Ok(())
    }

    //存入usdc/usdt到养老金账户
    pub fn deposit_token(ctx: Context<DepositToken>) -> ProgramResult {
        instructions::deposit_token(ctx)?;
        Ok(())
    }

    //检查sol养老金账户
    pub fn check_sol_account(ctx: Context<CheckSolAccount>) -> ProgramResult {
        instructions::check_sol_account(ctx)?;
        Ok(())
    }

    //检查usdc/usdt养老金账户
    pub fn check_token_account(ctx: Context<CheckTokenAccount>) -> ProgramResult {
        instructions::check_token_account(ctx)?;
        Ok(())
    }

    //关闭sol养老金账户
    pub fn close_sol_account(ctx: Context<CloseSolAccount>, expected_month: u8) -> ProgramResult {
        instructions::close_sol_account(ctx, expected_month)?;
        Ok(())
    }

    //关闭usdc/usdt养老金账户
    pub fn close_token_account(ctx: Context<CloseTokenAccount>, expected_month: u8) -> ProgramResult {
        instructions::close_token_account(ctx, expected_month)?;
        Ok(())
    }
}
