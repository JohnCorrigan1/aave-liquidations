use crate::pb;
use pb::pool;
use std::str::FromStr;
use substreams::{errors::Error, pb::substreams::Clock, scalar::BigInt};
use substreams_database_change::pb::database::DatabaseChanges;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables;

#[substreams::handlers::map]
pub fn graph_out(
    clock: Clock,
    liquidations: pool::Liquidations,
) -> Result<EntityChanges, substreams::errors::Error> {
    let mut tables = Tables::new();

    for liquidation in liquidations.liquidations {
        // let block_num = &clock.number.to_string();
        let timestamp = &clock.clone().timestamp.unwrap();
        let key = format!(
            "{}:{}:{}",
            liquidation.collateral_asset, liquidation.liquidator, liquidation.trx_hash
        );

        tables
            .create_row("Liquidation", key)
            .set("txHash", liquidation.trx_hash)
            .set("liquidator", liquidation.liquidator)
            .set("liquidatedAddress", liquidation.user)
            .set("collateralAsset", liquidation.collateral_asset)
            .set("debtAsset", liquidation.debt_asset)
            .set(
                "debtToCover",
                BigInt::from_str(&liquidation.debt_to_cover).unwrap(),
            )
            .set(
                "liquidatedCollateralAmount",
                BigInt::from_str(&liquidation.liquidated_collateral_amount).unwrap(),
            )
            .set("receiveToken", liquidation.receive_a_token)
            .set(
                "blockNumber",
                BigInt::from_str(&liquidation.block_num).unwrap(),
            )
            .set("timestamp", &timestamp.clone().to_string());
    }

    Ok(tables.to_entity_changes())
}

#[substreams::handlers::map]
pub fn db_out(clock: Clock, liquidations: pool::Liquidations) -> Result<DatabaseChanges, Error> {
    let mut tables = substreams_database_change::tables::Tables::new();
    let block_num = clock.number.to_string();
    let timestamp = clock.timestamp.unwrap().seconds.to_string();

    for liquidation in liquidations.liquidations {
        let key = format!(
            "{}:{}:{}",
            liquidation.collateral_asset, liquidation.liquidator, liquidation.trx_hash
        );

        tables
            .create_row("liquidations", key)
            .set("trx_hash", liquidation.trx_hash)
            .set("liquidator", liquidation.liquidator)
            .set("liquidated_address", liquidation.user)
            .set("debt_to_cover", liquidation.debt_to_cover)
            .set(
                "liquidated_collateral_amount",
                liquidation.liquidated_collateral_amount,
            )
            .set("receive_a_token", liquidation.receive_a_token)
            .set("block_num", &block_num)
            .set("timestamp", &timestamp);
    }

    Ok(tables.to_database_changes())
}
