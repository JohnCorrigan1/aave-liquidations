mod abi;
mod pb;
use hex_literal::hex;
use pb::pool;
use substreams::Hex;
use substreams_ethereum::pb::sf::ethereum::r#type::v2 as eth;

const POOL_CONTRACT: [u8; 20] = hex!("87870Bca3F3fD6335C3F4ce8392D69350B4fA4E2");

substreams_ethereum::init!();

/// Extracts transfers events from the contract
#[substreams::handlers::map]
fn map_liquidations(
    blk: eth::Block,
) -> Result<Option<pool::Liquidations>, substreams::errors::Error> {
    let liquidations: Vec<_> = blk
        .events::<abi::pool::events::LiquidationCall>(&[&POOL_CONTRACT])
        .map(|(liquidation, log)| {
            substreams::log::info!("Liquidation seen");

            pool::Liquidation {
                trx_hash: Hex::encode(&log.receipt.transaction.hash),
                block_num: blk.number.to_string(),
                collateral_asset: Hex::encode(&liquidation.collateral_asset),
                debt_asset: Hex::encode(&liquidation.debt_asset),
                user: Hex::encode(&liquidation.user),
                debt_to_cover: liquidation.debt_to_cover.to_string(),
                liquidated_collateral_amount: liquidation.liquidated_collateral_amount.to_string(),
                liquidator: Hex::encode(&liquidation.liquidator),
                receive_a_token: liquidation.receive_a_token,
            }
        })
        .collect();
    if liquidations.len() == 0 {
        return Ok(None);
    }

    Ok(Some(pool::Liquidations { liquidations }))
}

#[substreams::handlers::map]
fn map_wbtc_liquidations(
    liquidations: pool::Liquidations,
) -> Result<Option<pool::WbtcLiquidations>, substreams::errors::Error> {
    let wbtc_liquidations: Vec<_> = liquidations
        .liquidations
        .iter()
        .filter_map(|liquidation| {
            substreams::log::info!("asset: {}", liquidation.collateral_asset);
            substreams::log::info!(
                "asset hexed: {}",
                Hex::encode("0x2260FAC5E5542a773Aa44fBCfeDf7C193bc2C599")
            );
            if liquidation.collateral_asset
                == "2260fac5e5542a773aa44fbcfedf7c193bc2c599".to_string()
            {
                substreams::log::info!("Liquidation seen");
                Some(pool::WbtcLiquidation {
                    trx_hash: liquidation.trx_hash.clone(),
                    block_num: liquidation.block_num.clone(),
                    collateral_asset: liquidation.collateral_asset.clone(),
                    debt_asset: liquidation.debt_asset.clone(),
                    user: liquidation.user.clone(),
                    debt_to_cover: liquidation.debt_to_cover.clone(),
                    liquidated_collateral_amount: liquidation
                        .liquidated_collateral_amount
                        .clone()
                        .parse::<f32>()
                        .unwrap()
                        / 1e8,
                    liquidator: liquidation.liquidator.clone(),
                    receive_a_token: liquidation.receive_a_token,
                })
            } else {
                None
            }
        })
        .collect();

    if wbtc_liquidations.len() == 0 {
        return Ok(None);
    }

    Ok(Some(pool::WbtcLiquidations { wbtc_liquidations }))
}

#[substreams::handlers::map]
fn map_aave_liquidations(
    liquidations: pool::Liquidations,
) -> Result<Option<pool::AaveLiquidations>, substreams::errors::Error> {
    let aave_liquidations: Vec<_> = liquidations
        .liquidations
        .iter()
        .filter_map(|liquidation| {
            substreams::log::info!("asset: {}", liquidation.collateral_asset);
            substreams::log::info!(
                "asset hexed: {}",
                Hex::encode("0x2260FAC5E5542a773Aa44fBCfeDf7C193bc2C599")
            );
            if liquidation.collateral_asset
                == "2260fac5e5542a773aa44fbcfedf7c193bc2c599".to_string()
            {
                substreams::log::info!("Liquidation seen");
                Some(pool::AaveLiquidation {
                    trx_hash: liquidation.trx_hash.clone(),
                    block_num: liquidation.block_num.clone(),
                    collateral_asset: liquidation.collateral_asset.clone(),
                    debt_asset: liquidation.debt_asset.clone(),
                    user: liquidation.user.clone(),
                    debt_to_cover: liquidation.debt_to_cover.clone(),
                    liquidated_collateral_amount: liquidation
                        .liquidated_collateral_amount
                        .clone()
                        .parse::<f64>()
                        .unwrap()
                        / 1e8,
                    liquidator: liquidation.liquidator.clone(),
                    receive_a_token: liquidation.receive_a_token,
                })
            } else {
                None
            }
        })
        .collect();

    if aave_liquidations.len() == 0 {
        return Ok(None);
    }

    Ok(Some(pool::AaveLiquidations { aave_liquidations }))
}
