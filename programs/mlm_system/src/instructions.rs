use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;
use crate::CreatePDAUserAccount;

pub const MIN_INVESTMENT: u64 = 1;

pub const INVESTMENT_LEVELS: [f64; 10] = [0.005, 0.01, 0.02, 0.05, 0.1, 0.2, 0.5, 1.0, 2.0, 5.0];

pub const COMMISSION_LEVELS: [f32; 10] = [1.0, 0.7, 0.5, 0.2, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1];

pub fn signup(ctx: Context<CreatePDAUserAccount>, referrer: Pubkey) -> ProgramResult {
    let user = &mut ctx.accounts.user_info;
    user.referrer = referrer;
    Ok(())
}

pub fn get_partners(ctx: Context<CreatePDAUserAccount>) -> ProgramResult {
    let user = &mut ctx.accounts.user_info;

    let partners_amount = user.partners.len();

    let partners_levels = Vec::<usize>::with_capacity(partners_amount);

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

pub fn invest(ctx: Context<CreatePDAUserAccount>, investment_amount: u64) -> ProgramResult {

    if investment_amount > MIN_INVESTMENT {
        **ctx.accounts.user.to_account_info().try_borrow_mut_lamports()? -= investment_amount;
        **ctx.accounts.system_program.to_account_info().try_borrow_mut_lamports()? += investment_amount;

        let user = &mut ctx.accounts.user_info;
        let total_investment_amount = investment_amount - (investment_amount * 5 / 100);

        user.balance = total_investment_amount;

        Ok(())
    }

    else {
        panic!("You're trying to invest less than it's needed! The minimum investment is {}.", MIN_INVESTMENT);
    }
}


pub fn withdraw(ctx: Context<CreatePDAUserAccount>) -> ProgramResult {
    Ok(())
}
