use anchor_lang::prelude::*;

declare_id!("EuuVAbcqK268gEjAoSKHkrG9nCpbmooYqYXfcndbFHdn");

#[program]
pub mod mlm_system {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

