use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod testing {
    use super::*;

    pub fn initialize_a(ctx: Context<Initialize>) -> Result<()> {
        emit!(HelloWorld {
            message: "foobar".to_string(),
            number: 123,
            such: "baz".to_string(),
            compute_units: "bar".to_string(),
            very_wow: true,
            a: "a".to_string(),
            b: "b".to_string()
        });
        Ok(())
    }

    pub fn initialize_a_pubkeys(ctx: Context<Initialize>) -> Result<()> {
        emit!(HelloWorldWithPubkeys {
            message: "foobar".to_string(),
            number: 123,
            such: "baz".to_string(),
            compute_units: "bar".to_string(),
            very_wow: true,
            a: "a".to_string(),
            b: "b".to_string(),
            pubkey_a: *ctx.program_id,
            pubkey_b: *ctx.program_id
        });
        Ok(())
    }

    pub fn initialize_b_pubkeys(ctx: Context<Initialize>) -> Result<()> {
        msg!("{{\"message\": \"{}\", \"number\": {}, \"such\": \"{}\", \"compute_units\": \"{}\", \"very_wow\": \"{}\", \"a\": \"{}\", \"b\": \"{}\", \"pubkey_a\": \"{}\", \"pubkey_b\": \"{}\"}}", "foobar", 123, "baz", "bar", true, "a", "b", *ctx.program_id, *ctx.program_id);
        Ok(())
    }

    pub fn initialize_b(ctx: Context<Initialize>) -> Result<()> {
        msg!("{{\"message\": \"{}\", \"number\": {}, \"such\": \"{}\", \"compute_units\": \"{}\", \"very_wow\": \"{}\", \"a\": \"{}\", \"b\": \"{}\"}}", "foobar", 123, "baz", "bar", true, "a", "b");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[event]
pub struct HelloWorldWithPubkeys {
    pub message: String,
    pub number: u64,
    pub such: String,
    pub compute_units: String,
    pub very_wow: bool,
    pub a: String,
    pub b: String,
    pub pubkey_a: Pubkey,
    pub pubkey_b: Pubkey,
}

#[event]
pub struct HelloWorld {
    pub message: String,
    pub number: u64,
    pub such: String,
    pub compute_units: String,
    pub very_wow: bool,
    pub a: String,
    pub b: String,
}
