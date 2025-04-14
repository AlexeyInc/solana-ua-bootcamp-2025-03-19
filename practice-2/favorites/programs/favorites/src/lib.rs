use anchor_lang::prelude::*;

declare_id!("5tr3bee3syW4xAxJL9m47y4bhLScdv7v5geuzo8o6MHS");

#[program]
pub mod favorites {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
