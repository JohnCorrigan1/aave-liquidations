use crate::abi;
use crate::pb;
use hex_literal::hex;
use pb::pool;
use substreams::Hex;
use substreams_ethereum::pb::sf::ethereum::r#type::v2 as eth;

const POOL_CONTRACT: [u8; 20] = hex!("87870Bca3F3fD6335C3F4ce8392D69350B4fA4E2");

substreams_ethereum::init!();

#[substreams::handlers::map]
pub fn map_liquidations(
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
pub fn map_wbtc_liquidations(
    liquidations: pool::Liquidations,
) -> Result<Option<pool::TokenLiquidations>, substreams::errors::Error> {
    let token_liquidations: Vec<_> = liquidations
        .liquidations
        .iter()
        .filter_map(|liquidation| {
            if liquidation.collateral_asset
                == "2260fac5e5542a773aa44fbcfedf7c193bc2c599".to_string()
            {
                substreams::log::info!("Liquidation seen");
                Some(pool::TokenLiquidation {
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

    if token_liquidations.len() == 0 {
        return Ok(None);
    }

    Ok(Some(pool::TokenLiquidations { token_liquidations }))
}

#[substreams::handlers::map]
pub fn map_aave_liquidations(
    liquidations: pool::Liquidations,
) -> Result<Option<pool::TokenLiquidations>, substreams::errors::Error> {
    let token_liquidations: Vec<_> = liquidations
        .liquidations
        .iter()
        .filter_map(|liquidation| {
            if liquidation.collateral_asset
                == "7fc66500c84a76ad7e9c93437bfc5ac33e2ddae9".to_string()
            {
                substreams::log::info!("aave liquidations found");
                Some(pool::TokenLiquidation {
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
                        / 1e18,
                    liquidator: liquidation.liquidator.clone(),
                    receive_a_token: liquidation.receive_a_token,
                })
            } else {
                None
            }
        })
        .collect();

    if token_liquidations.len() == 0 {
        return Ok(None);
    }

    Ok(Some(pool::TokenLiquidations { token_liquidations }))
}

#[substreams::handlers::map]
pub fn map_weth_liquidations(
    liquidations: pool::Liquidations,
) -> Result<Option<pool::TokenLiquidations>, substreams::errors::Error> {
    let token_liquidations: Vec<_> = liquidations
        .liquidations
        .iter()
        .filter_map(|liquidation| {
            if liquidation.collateral_asset
                == "c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2".to_string()
            {
                substreams::log::info!("weth liquidations found");
                Some(pool::TokenLiquidation {
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
                        / 1e18,
                    liquidator: liquidation.liquidator.clone(),
                    receive_a_token: liquidation.receive_a_token,
                })
            } else {
                None
            }
        })
        .collect();

    if token_liquidations.len() == 0 {
        return Ok(None);
    }

    Ok(Some(pool::TokenLiquidations { token_liquidations }))
}

#[substreams::handlers::map]
pub fn map_wsteth_liquidations(
    liquidations: pool::Liquidations,
) -> Result<Option<pool::TokenLiquidations>, substreams::errors::Error> {
    let token_liquidations: Vec<_> = liquidations
        .liquidations
        .iter()
        .filter_map(|liquidation| {
            if liquidation.collateral_asset
                == "7f39c581f595b53c5cb19bd0b3f8da6c935e2ca0".to_string()
            {
                substreams::log::info!("wsteth liquidations found");
                Some(pool::TokenLiquidation {
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
                        / 1e18,
                    liquidator: liquidation.liquidator.clone(),
                    receive_a_token: liquidation.receive_a_token,
                })
            } else {
                None
            }
        })
        .collect();

    if token_liquidations.len() == 0 {
        return Ok(None);
    }

    Ok(Some(pool::TokenLiquidations { token_liquidations }))
}
