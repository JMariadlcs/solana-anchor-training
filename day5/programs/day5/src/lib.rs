use anchor_lang::prelude::*;

declare_id!("Csp8wQTsGgYU7rMbThkcAHwALwzjv6bPaiq3CDGYAG17");

#[program]
pub mod day5 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
