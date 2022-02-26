use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;

declare_id!("4P7wEEmpejjYFxkMk6dFJH4siA7E5KQF7SRc6fSLnai9");

#[program]
pub mod test_payment {

    use anchor_lang::solana_program::{
        lamports,
        program::{invoke, invoke_signed},
        system_instruction::{transfer , assign_with_seed, assign}
    };

    use super::*;
    pub fn initialize(ctx: Context<Initialize>, bump: u8) -> ProgramResult {
        let lock_account = &mut ctx.accounts.lock_account;
        //let tx  = &assign(lock_account.to_account_info().key, ctx.accounts.owner.to_account_info().key);
        lock_account.owner = *ctx.accounts.owner.key;
        lock_account.locked = true;
        lock_account.bump = bump;
        Ok(())
    }

    pub fn payin(ctx: Context<Payin>) -> ProgramResult {
        let lock_account = &mut ctx.accounts.lock_account;
        let transfer_instruction = &transfer(
            &lock_account.owner,
            &lock_account.to_account_info().key,
            10000,
        );
        msg!("Paying in {}", 10000);
        invoke(
            transfer_instruction,
            &[
                ctx.accounts.owner.to_account_info(),
                lock_account.to_account_info(),       
            ]
        )
    }
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct Initialize<'info> {
    #[account(init,
    payer=owner,
    space=8 + 32 + 32 + 1 + 1)
    ]
    pub lock_account: Account<'info, LockAccount>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Payin<'info> {
    #[account(mut, has_one = owner)]
    pub lock_account: Account<'info, LockAccount>,
    #[account(mut)]
    pub owner: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct LockAccount {
    pub owner: Pubkey,
    pub locked: bool,
    bump: u8,
}