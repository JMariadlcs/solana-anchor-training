use anchor_lang::prelude::*;

declare_id!("ACf7gwqTZVZxXwLWjzDDpSssAdqSzpq8RNkbygwLpTpJ");

#[program]
pub mod day8 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
