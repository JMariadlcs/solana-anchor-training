use anchor_lang::prelude::*;

declare_id!("4KmQVcfRSFBANW9j7jTWubKpZvmP45qPdtfdVNFiFct3");

#[program]
pub mod day11 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
