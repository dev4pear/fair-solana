use anchor_lang::prelude::*;
use crate::instructions::*;

mod instructions;
pub mod states;

declare_id!("81Pjgpp9Q8zjQrNqtVDjjzTLK4U568u1UAmN5WoRXb5m");

pub const FEE_WALLET: &str = "JCsE6i2uDDiTkjPozHpQakp4CceHahXMWEWVcdfoEnC8";
pub const FEE_LAMPORTS: u64 = 200_000_000;

#[program]
pub mod pump_fun_sp {
    use super::*;

    pub fn create_token(ctx: Context<CreateToken>, decimals: u8, amount: u64) -> Result<()> {
        create_token_handler(ctx, decimals, amount)
    }
}

