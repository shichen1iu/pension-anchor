use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod error;

use instructions::*;

declare_id!("BAYLCVPy2rmpxbC3okEbbR3PV5Peo4XKGncygMzmxtCC");

#[program]
pub mod pension {
    use anchor_lang::solana_program::entrypoint::ProgramResult;

    use super::*;

    pub fn initialize_sol(
        ctx: Context<InitializeSol>,
        expected_lamports: u16,
        expected_year: u8,
    ) -> ProgramResult {
        instructions::initialize_sol(ctx, expected_lamports, expected_year)?;
        Ok(())
    }

    pub fn initialize_token(
        ctx: Context<InitializeToken>,
        expected_lamports: u16,
        expected_year: u8,
    ) -> ProgramResult {
        instructions::initialize_token(ctx, expected_lamports, expected_year)?;
        Ok(())
    }

    
}
