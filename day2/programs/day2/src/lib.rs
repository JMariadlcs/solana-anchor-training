use anchor_lang::prelude::*;

declare_id!("CgVcZanBVfh1x8LgeUdeSUF7DWBqGZth6iQZ4HCF4qD");

#[program]
pub mod day2 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, a: u64, b: u64, message: String) -> Result<()> {
        msg!("Greetings from: {} and {}", a, b);
        msg!("Message: {}", message);
        Ok(())
    }

    pub fn array(ctx: Context<Initialize>, arr: Vec<u64>) -> Result<()> {
        msg!("Your array {:?}", arr);
        Ok(())
    }

    pub fn checkOverflowUnderflow(ctx: Context<Initialize>, a: u64, b: u64) -> Result<()> {
        let x = a - b;
        msg!("Your result {:?}", x);
        Ok(())
    }

    pub fn checkOverflowUnderflowProtected(ctx: Context<Initialize>, a: u64, b: u64) -> Result<()> {
        let x = a.checked_sub(b).unwrap();
        msg!("Your result {:?}", x);
        Ok(())
    }

    pub fn cubeRoot(ctx: Context<Initialize>, a: f32) -> Result<()> {
        msg!("You said {:?}", a.cbrt());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
