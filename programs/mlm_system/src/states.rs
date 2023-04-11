use anchor_lang::prelude::*;
use core::mem::size_of;

#[derive(Accounts)]
#[instruction(program_bump: u8)]
pub struct Initialize<'info>{
    #[account(
    init,
    seeds = [b"program".as_ref()],
    bump,
    payer = user,
    space = size_of::<ProgramAccount>()
    )]
    pub program: Account<'info, ProgramAccount>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
#[derive(Default)]
pub struct ProgramAccount {
    pub balance: u64,
    pub bump: u8,
    pub percentage: u8,
}

#[derive(Accounts)]
#[instruction(program_bump: u8)]
pub struct CreatePDAUserAccount<'info> {
    #[account(
    init,
    seeds = [b"program".as_ref()],
    bump,
    payer = user,
    space = size_of::<User>()
    )]
    pub user_info: Account<'info, User>,

    pub program_info: Account<'info, ProgramAccount>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct User {
    pub referrer: Pubkey,
    pub balance: u64,
    pub partners: Vec<Pubkey>,
    pub bump: u8,
}
