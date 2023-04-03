use anchor_lang::prelude::*;

declare_id!("CBC3aBwHwrRBxuq1zUmJ2DvJ5M8hLDDMgeGgYXxiwNmZ");

#[program]
pub mod live_streaming {
  use super::*;

  pub fn initialize_live_stream(ctx: Context<InitializeLiveStream>) -> Result<()> {
    Ok(())
  }
}


// - init live stream with a PDA. seeds should be ["live_stream", creator.key().as_ref(), id], where
//   `id` is a provided String value
// - live stream PDA account stores the creator public key, so it can be verified for payment
#[derive(Accounts)]
pub struct InitializeLiveStream<'info> {
  #[account(mut)]
  pub creator: Signer<'info>
}

// - user requests access to stream via an instruction
// - the instruction takes in the creator public key and finds the PDA with ["live_stream, creator.key().as_ref(), id"]
//   where the id is the String stream id
// - the instruction transfers money from the user to the creator, and only then does it create an account
//     - is it possible to conditionally create an account? if not, then set some "paid" value to true/false based on token transfer
//     - maybe transfer to the stream PDA instead as an escrow? i think only our program would allow that transfer since it's
//       the only one that can sign for the PDA
#[derive(Accounts)]
pub struct UserLiveStreamAccess<'info> {
  #[account(mut)]
  pub user: Signer<'info>
}