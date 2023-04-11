use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;
use anchor_spl::token::{MintTo};
use anchor_spl::token::mint_to;

use crate::CreatePDAUserAccount;
use crate::token_states::MintToken;
use crate::token_states::InitializeMint;

pub const MIN_INVESTMENT: u64 = 1;

pub const INVESTMENT_LEVELS: [f64; 10] = [0.005, 0.01, 0.02, 0.05, 0.1, 0.2, 0.5, 1.0, 2.0, 5.0];

pub const COMMISSION_LEVELS: [f32; 10] = [1.0, 0.7, 0.5, 0.2, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1];

pub fn signup(ctx: Context<CreatePDAUserAccount>, user_address: Pubkey, referrer: Pubkey) -> ProgramResult {
    let user = &mut ctx.accounts.user_info;

    // Check if the public key passed to the function matches the public key of the user account
    if user.key() != user_address {
        return Err(ProgramError::InvalidArgument);
    }
    user.referrer = referrer;

    Ok(())
}

pub fn get_partners(ctx: Context<CreatePDAUserAccount>) -> ProgramResult {
    let user = &mut ctx.accounts.user_info;
    let partners_amount = user.partners.len();
    let partners_levels = Vec::<usize>::with_capacity(partners_amount);
    let mut partners = Vec::new();

    for i in 0..partners_amount{
        let _address = &mut ctx.accounts.user_info.partners;
        partners.push(i);
    }

    print!("Amount of partners: {}", partners_amount);
    print!("Levels of partners: {:?}", partners_levels);

    Ok(())
}

pub fn get_level(account_balance: f64) -> u128 {
    let mut level: u128 = 0;

    for i in 0..9 {
        if (account_balance) < INVESTMENT_LEVELS[i] {
            level = (i + 1) as u128;
        }
    }

    return level;
}

pub fn invest(ctx: Context<CreatePDAUserAccount>, investment_amount: u64, user: Pubkey) -> ProgramResult {
    if investment_amount > MIN_INVESTMENT {
        let mut user_account = ctx.accounts.user.to_account_info();
        let program = &mut ctx.accounts.program_info;
        let percentage:u8 = program.percentage;

        **user_account.try_borrow_mut_lamports()? -= investment_amount;
        **user_account.try_borrow_mut_lamports()? += investment_amount;

        // Check if the public key passed to the function matches the public key of the user account
        if user_account.key() != user {
            return Err(ProgramError::InvalidArgument);
        }

        let user_info = &mut ctx.accounts.user_info;
        let total_investment_amount = investment_amount - (investment_amount * percentage as u64/ 100);

        user_info.balance += total_investment_amount;

        let lamports = &mut user_account.lamports;
        **lamports.borrow_mut() += total_investment_amount;

        Ok(())
    }
    else {
        panic!("You're trying to invest less than it's needed! The minimum investment is {}.", MIN_INVESTMENT);
    }
}

pub fn withdraw(ctx: Context<CreatePDAUserAccount>, user_address: Pubkey) -> ProgramResult {
    let user = &mut ctx.accounts.user_info;

    // Check if the public key passed to the function matches the public key of the user account
    if user.key() != user_address {
        return Err(ProgramError::InvalidArgument);
    }

    let mut user_balance: f32 = user.balance as f32;

    if user_balance <= 0.0 {
        panic!("You cannot withdraw money from your account, you don't have no money at all!");
    }

    let payer = &mut ctx.accounts.user.key();
    let mut _address: &mut Pubkey = payer;
    let mut _referral_count_depth: i8 = 1;
    let mut index;
    let mut _partner_commission: f32 = 0.0;

    for _i in 0..10 {
        while payer.to_string() != " " {
            _referral_count_depth += 1;
            _address = &mut user.referrer;
            index = get_level(user_balance as f64) as usize / 1000;
            _partner_commission = user_balance * COMMISSION_LEVELS[index];

            **ctx.accounts.user
                .to_account_info()
                .try_borrow_mut_lamports()? -= _partner_commission as u64;

            **ctx.accounts.system_program
                .to_account_info()
                .try_borrow_mut_lamports()? += _partner_commission as u64;

            user_balance = user_balance - _partner_commission;
        }
    }

    user.balance = 0;

    Ok(())
}

pub fn initialize_mint(_ctx: Context<InitializeMint>) -> Result<()> {
    Ok(())
}

pub fn mint_token(ctx: Context<MintToken>, amount: u64) -> Result<()> {
    let cpi_accounts = MintTo {
        mint: ctx.accounts.mint.to_account_info(),
        to: ctx.accounts.token_account.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

    // Execute anchor's helper function to mint tokens
    mint_to(cpi_ctx, amount)?;

    Ok(())
}