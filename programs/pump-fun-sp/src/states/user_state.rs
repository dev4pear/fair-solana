use anchor_lang::prelude::*;

#[account]
pub struct UserState {
  pub tokens: Vec<Pubkey>,
  pub wallet: Pubkey,
}