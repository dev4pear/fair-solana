use anchor_lang::{ prelude::*, solana_program::{ program::invoke, system_instruction } };
use anchor_spl::{associated_token::{self, AssociatedToken, Create}, token::{ self, Mint, MintTo, Token}};
use crate::{ FEE_LAMPORTS, FEE_WALLET };
use std::str::FromStr;

#[derive(Accounts)]
#[instruction(_decimals: u8)]
pub struct CreateToken<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    /// CHECK:
    #[account(mut, address = Pubkey::from_str(FEE_WALLET).unwrap())]
    pub fee_acc: AccountInfo<'info>,

    /// CHECK:
    #[account(mut)]
    pub user_token: AccountInfo<'info>,

    #[account(
        init,
        payer = signer,
        mint::decimals = _decimals,
        mint::authority = signer,
    )]
    pub mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreateToken<'info> {
    fn transfer_fee(&self, fee: u64) -> Result<()> {
        invoke(
            &system_instruction::transfer(self.signer.key, self.fee_acc.key, fee),
            &[
                self.signer.to_account_info(),
                self.fee_acc.clone(),
                self.system_program.to_account_info(),
            ]
        ).map_err(Into::into)
    }

    fn mint_ctx(&self) -> CpiContext<'info, 'info, 'info, 'info, MintTo<'info>> {
        CpiContext::new(self.token_program.to_account_info(), MintTo {
            mint: self.mint.to_account_info(),
            to: self.user_token.to_account_info(),
            authority: self.signer.to_account_info(),
        })
    }
    pub fn create_ctx(&self) -> CpiContext<'info, 'info, 'info, 'info, Create<'info>> {
        CpiContext::new(self.associated_token_program.to_account_info(), Create {
            payer: self.signer.to_account_info(),
            associated_token: self.user_token.clone(),
            authority: self.signer.to_account_info(),
            mint: self.mint.to_account_info(),
            system_program: self.system_program.to_account_info(),
            token_program: self.token_program.to_account_info(),
        })
    }
}

pub fn create_token_handler(ctx: Context<CreateToken>, _decimals: u8, amount: u64) -> Result<()> {
    if ctx.accounts.user_token.data_is_empty() {
        associated_token::create(ctx.accounts.create_ctx())?;
    }
    // mint token
    token::mint_to(ctx.accounts.mint_ctx(), amount)?;
    ctx.accounts.transfer_fee(FEE_LAMPORTS)?;
    msg!("Token is created");
    Ok(())
}
