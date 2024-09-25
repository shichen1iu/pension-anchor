use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("BAYLCVPy2rmpxbC3okEbbR3PV5Peo4XKGncygMzmxtCC");

#[program]
pub mod pension {
    use anchor_lang::solana_program::entrypoint::ProgramResult;

    use super::*;

    pub fn deposit_sol(
        ctx: Context<DepositSol>,
        amount: u64,
        expected_lamports: u16,
        expected_year: u8,
    ) -> ProgramResult {
        instructions::deposit_sol(ctx, amount, expected_lamports, expected_year)?;
        Ok(())
    }

    pub fn deposit_usdc(
        ctx: Context<DepositUsdc>,
        amount: u64,
        expected_usdc: u16,
        expected_year: u8,
    ) -> ProgramResult {
        instructions::deposit_usdc(ctx, amount, expected_usdc, expected_year)?;
        Ok(())
    }
}
