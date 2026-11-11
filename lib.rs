use anchor_lang::prelude::*;
declare_id!("4rQivUeHSyh8D4CxA6bVwwqhHQafAvxwHxT84pAjeFXu");

#[program]
pub mod hello_ai {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, message: String) -> Result<()> {
        ctx.accounts.note.author = ctx.accounts.signer.key();
        ctx.accounts.note.message = message;
        msg!("gm from Solana devnet!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = signer, space = 8 + 32 + 4 + 100)]
    pub note: Account<'info, Note>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Note {
    pub author: Pubkey,
    pub message: String,
}
