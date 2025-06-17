use anchor_lang::prelude::*;

// Import `calculate` module or crate
pub mod calculate;

declare_id!("53hgft52DHUKMPHGu1kusuwxFGk2T8qngwSw2SyGRNrX");

#[program]
pub mod func_visibility {
    use super::*;

    pub fn add_two_numbers(_ctx: Context<Initialize>, x: u64, y: u64) -> Result<()> {
        // Call `add` function in calculate.rs
        let result = calculate::add(x, y);

        msg!("{} + {} = {}", x, y, result);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

