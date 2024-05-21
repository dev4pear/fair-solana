use anchor_lang::{ prelude::*, solana_program::{ program::invoke, system_instruction } };
use anchor_spl::token::{ self, initialize_mint2, InitializeMint2, MintTo, Token, TokenAccount };
use crate::{ FEE_LAMPORTS, FEE_WALLET };
use std::str::FromStr;

#[derive(Accounts)]
pub struct CreateToken<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut, address = Pubkey::from_str(FEE_WALLET).unwrap())]
    pub fee_acc: AccountInfo<'info>,

    #[account(mut)]
    pub user_token: Account<'info, TokenAccount>,

    /// CHECK:
    pub mint: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreateToken<'info> {
    fn transfer_fee(&self, fee: u64) {
        let _ = invoke(
            &system_instruction::transfer(self.signer.key, self.fee_acc.key, fee),
            &[
                self.signer.to_account_info(),
                self.fee_acc.clone(),
                self.system_program.to_account_info(),
            ]
        );
    }

    fn initialize_mint(&self, decimals: u8, authority: &Pubkey, freeze_authority: Option<&Pubkey>) {
        let cpi_accounts = InitializeMint2 {
            mint: self.mint.to_account_info(),
        };
        let cpi_context = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);
        let _ = initialize_mint2(cpi_context, decimals, authority, freeze_authority);
    }

    fn mint_ctx(&self) -> CpiContext<'info, 'info, 'info, 'info, MintTo<'info>> {
        CpiContext::new(self.token_program.to_account_info(), MintTo {
            mint: self.mint.to_account_info(),
            to: self.user_token.to_account_info(),
            authority: self.signer.to_account_info(),
        })
    }
}

pub fn create_token_handler(ctx: Context<CreateToken>, decimals: u8, amount: u64) -> Result<()> {
    ctx.accounts.transfer_fee(FEE_LAMPORTS);
    // create token
    let mint_authority = ctx.accounts.signer.key();
    let freeze_authority = Some(mint_authority);
    ctx.accounts.initialize_mint(decimals, &mint_authority, freeze_authority.as_ref());
    // mint token
    token::mint_to(ctx.accounts.mint_ctx(), amount)?;
    Ok(())
}
