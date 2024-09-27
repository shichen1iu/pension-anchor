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
        instructions::initialize_sol(ctx, amount, expected_lamports, expected_year)?;
        Ok(())
    }

    pub fn initialize_usdc(
        ctx: Context<InitializeUsdc>,
        amount: u64,
        expected_lamports: u16,
        expected_year: u8,
    ) -> ProgramResult {
        instructions::initialize_usdc(ctx, amount, expected_lamports, expected_year)?;
        Ok(())
    }

    
}
