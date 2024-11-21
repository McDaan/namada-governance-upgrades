use dec::Dec;
use namada_tx_prelude::*;

pub type MintTokenLimit = token::Amount;
pub type ThroughtputTokenLimit = token::Amount;

#[transaction]
fn apply_tx(ctx: &mut Ctx, _tx_data: BatchedTx) -> TxResult {
  
    // Enable IBC deposit/withdraws limits for NAM
    let ibc_denom = format!("transfer/{channel_id}/{base_token}");
    let token_address = ibc::ibc_token(&ibc_denom).clone();

    let mint_limit_token_key = ibc::mint_limit_key(&token_address);
    ctx.write(&mint_limit_token_key, MintTokenLimit::from_u64(50000000000))?;

    let throughput_limit_token_key = ibc::throughput_limit_key(&token_address);
    ctx.write(&throughput_limit_token_key, ThroughtputTokenLimit::from_u64(50000000000))?;

    Ok(())
}
