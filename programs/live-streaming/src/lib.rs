use anchor_lang::prelude::*;

declare_id!("5MXnC6dAHooETyUNL2toGukFCPsVZ7sxDKiRCrij6sMS");

#[program]
pub mod live_streaming {
  use super::*;

  pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    Ok(())
  }
}

#[derive(Accounts)]
pub struct Initialize {}
