use anchor_lang::prelude::*;

declare_id!("CBC3aBwHwrRBxuq1zUmJ2DvJ5M8hLDDMgeGgYXxiwNmZ");

fn stream_id_seed(id: &str) -> &[u8] {
  let bytes = id.as_bytes();
  if bytes.len() > 32 { &bytes[0..32] } else { bytes }
}

#[program]
pub mod live_streaming {
  use super::*;

  pub fn initialize_live_stream(ctx: Context<InitializeLiveStream>, id: String) -> Result<()> {
    let live_stream = &mut ctx.accounts.live_stream;
    let creator = &ctx.accounts.creator;
    live_stream.creator = creator.key();
    live_stream.id = id;
    live_stream.bump = *ctx.bumps.get("live_stream").unwrap();
    Ok(())
  }
}

/* ACCOUNTS */
#[account]
pub struct LiveStream {
  pub creator: Pubkey,
  pub id: String,
  pub bump: u8
}

// - init live stream with a PDA. seeds should be ["live_stream", creator.key().as_ref(), id], where
//   `id` is a provided String value
// - live stream PDA account stores the creator public key, so it can be verified for payment
#[derive(Accounts)]
#[instruction(id: String)]
pub struct InitializeLiveStream<'info> {
  #[account(mut)]
  pub creator: Signer<'info>,
  #[account(init,
    payer = creator,
    seeds = [b"live_stream", stream_id_seed(&id), creator.key().as_ref()],
    space = 8 + 64 + 64,
    bump
  )]
  pub live_stream: Account<'info, LiveStream>,
  pub system_program: Program<'info, System>
}

// - user requests access to stream via an instruction
// - the instruction takes in the creator public key and finds the PDA with ["live_stream, creator.key().as_ref(), id"]
//   where the id is the String stream id
// - the instruction transfers money from the user to the creator, and only then does it create an account
//     - is it possible to conditionally create an account? if not, then set some "paid" value to true/false based on token transfer
//     - maybe transfer to the stream PDA instead as an escrow? i think only our program would allow that transfer since it's
//       the only one that can sign for the PDA
// #[derive(Accounts)]
// #[instruction(creator: Pubkey, id: String)]
// pub struct UserLiveStreamAccess<'info> {
//   #[account(mut)]
//   pub user: Signer<'info>,
//   #[account(mut, seeds = [b"live_stream", creator.key().as_ref(), id])]
//   pub live_stream: Account<'info, LiveStream>,
//   pub system_program: Program<'info, System>
// }