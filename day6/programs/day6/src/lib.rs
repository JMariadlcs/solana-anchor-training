use anchor_lang::prelude::*;

declare_id!("ARSxS72ehDFZmJYSvsUJhmPGXhDG8Q7fvSJ4M9jBKp9X");

#[program]
pub mod day6 {
    use super::*;
        // Import HashMap library
    use std::collections::HashMap;

    pub fn initialize(ctx: Context<Initialize>, key: String, value: String) -> Result<()> {
        // Initialize the mapping
        let mut my_map = HashMap::new();

        // Add a key-value pair to the mapping
        my_map.insert(key.to_string(), value.to_string());

        // Log the value corresponding to a key from the mapping
        msg!("My name is {}", my_map[&key]);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
