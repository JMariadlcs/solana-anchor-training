use anchor_lang::prelude::*;

declare_id!("Cr1YK6FNDAZmNbojRF24k6ytRXNcrWw4S9dbbwjwXN6Y");

#[program]
pub mod day7 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn encode_and_decode(_ctx: Context<Initialize>) -> Result<()> {
        // Create a new instance of the `Person` struct
        let init_person: Person = Person {
            name: "Alice".to_string(),
            age: 27,
        };
    
        // Encode the `init_person` struct into a byte vector
        let encoded_data: Vec<u8> = init_person.try_to_vec().unwrap();
    
        // Decode the encoded data back into a `Person` struct
        let data: Person = decode(_ctx, encoded_data)?;
    
        // Logs the decoded person's name and age
        msg!("My name is {:?}, I am {:?} years old.", data.name, data.age);
    
        Ok(())
    }
    
    pub fn decode(_accounts: Context<Initialize>, encoded_data: Vec<u8>) -> Result<Person> {
        // Decode the encoded data back into a `Person` struct
        let decoded_data: Person = Person::try_from_slice(&encoded_data).unwrap();
    
        Ok(decoded_data)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
