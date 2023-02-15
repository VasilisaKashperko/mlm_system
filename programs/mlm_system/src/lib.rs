pub mod states;
pub mod instructions;

use anchor_lang::prelude::*;
use states::*;

declare_id!("EuuVAbcqK268gEjAoSKHkrG9nCpbmooYqYXfcndbFHdn");

#[program]
pub mod mlm_system {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, bump: u8) -> Result<()> {
        let program = &mut ctx.accounts.program;
        program.bump = bump;
        Ok(())
    }
}