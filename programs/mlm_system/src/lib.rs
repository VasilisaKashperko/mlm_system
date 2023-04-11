pub mod states;
pub mod instructions;
pub mod token_states;

use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;
use states::*;
use token_states::*;

declare_id!("EuuVAbcqK268gEjAoSKHkrG9nCpbmooYqYXfcndbFHdn");

#[program]
pub mod mlm_system {
    use crate::states::Initialize;
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, percentage: u8 ) -> Result<()> {
        let program = &mut ctx.accounts.program;
        program.balance = 0;
        program.percentage = percentage;

        Ok(())
    }

    pub fn signup(ctx: Context<CreatePDAUserAccount>, user_address: Pubkey, referrer: Pubkey) -> ProgramResult {
        instructions::signup(ctx, user_address, referrer)
    }

    pub fn get_partners(ctx: Context<CreatePDAUserAccount>) -> ProgramResult {
        instructions::get_partners(ctx)
    }

    pub fn invest(ctx: Context<CreatePDAUserAccount>, investment_amount: u64, user: Pubkey) -> ProgramResult {
        instructions::invest(ctx, investment_amount, user)
    }

    pub fn withdraw(ctx: Context<CreatePDAUserAccount>, user_address: Pubkey) -> ProgramResult {
        instructions::withdraw(ctx, user_address)
    }

    pub fn initialize_mint(ctx: Context<InitializeMint>) -> Result<()>{
        instructions::initialize_mint(ctx)
    }

    pub fn mint_token(ctx: Context<MintToken>, amount: u64) -> Result<()> {
        instructions::mint_token(ctx, amount)
    }
}
