use namada_tx_prelude::*;

pub type MintTokenLimit = token::Amount;
pub type ThroughtputTokenLimit = token::Amount;

#[transaction]
fn apply_tx(ctx: &mut Ctx, _tx_data: BatchedTx) -> TxResult {
  
    // Enable IBC deposit/withdraws limits for native token NAM
    let native_token = tx_host_env::with(|tx_env| {
       let native_token = tx_env.state.in_mem().native_token.clone();
       native_token
    });

    let mint_limit_token_key = ibc::mint_limit_key(&native_token);
    ctx.write(&mint_limit_token_key, MintTokenLimit::from_u64(50000000000))?;

    let throughput_limit_token_key = ibc::throughput_limit_key(&native_token);
    ctx.write(&throughput_limit_token_key, ThroughtputTokenLimit::from_u64(50000000000))?;

    Ok(())
}
