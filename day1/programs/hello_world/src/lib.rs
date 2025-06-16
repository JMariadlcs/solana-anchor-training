use anchor_lang::prelude::*;

declare_id!("56UPC2kgE17wQXYsztXDFK9L5P12yksTFZn4pPHiJB36");

#[program]
pub mod hello_world {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Hello, world!"); 
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
