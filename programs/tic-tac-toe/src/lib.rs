use anchor_lang::prelude::*;

declare_id!("8kJ7zoPMNdF2EN7hS7XKmim7rpZoPwUdXNarcNmJqTZx");

#[program]
pub mod tic_tac_toe {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
