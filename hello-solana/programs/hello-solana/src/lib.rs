use anchor_lang::prelude::*;

declare_id!("7gB35J4QYBEBSW6V8konoZKQA7RF2NoLi7o3kXgQJMi7");

#[program]
pub mod hello_solana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: mr.pavankumar ");
        msg!("Hello world");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
