use anchor_lang::prelude::*;
//use anchor_spl::token_2022::{self, TransferChecked};
//use anchor_spl::token_interface::{Mint, Token2022, TokenAccount};
use anchor_spl::{associated_token::AssociatedToken, token::{self, Mint, Token, TokenAccount, Transfer}};

declare_id!("ASR2KSdgBuAu9HRJtEEMhrtSXSXVySJFhBzZkx1z57At");

#[program]
pub mod token_transfer {
    use super::*;

    pub fn transfer_spl_tokens(ctx: Context <TokenTransfer>, amount:u64) -> Result<()> {
        let cpi_accounts= Transfer {
            from: ctx.accounts.from_ata.to_account_info(),
            to: ctx.accounts.to_ata.to_account_info(),
            authority: ctx.accounts.from.to_account_info(),
        };
        let cpi_program =ctx.accounts.token_program.to_account_info();

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        
        token::transfer(cpi_ctx, amount)?;
        msg!("From ATA: {:?}", ctx.accounts.from_ata.key());
        msg!("To ATA: {:?}", ctx.accounts.to_ata.key());
        msg!("Amount: {:?}", amount);
        msg!("From Signer: {:?}", ctx.accounts.from.key());

                
        Ok(())

    }
}

#[derive(Accounts)]
pub struct TokenTransfer<'info> {
    #[account(
        init_if_needed,
        payer = from,
        associated_token::mint = mint,
        associated_token::authority = from,

    )]
    pub from_ata: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = from,
        associated_token::mint = mint,
        associated_token::authority = receiver,

    )]
    pub to_ata: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub from: Signer<'info>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    pub receiver: UncheckedAccount<'info >,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>
}