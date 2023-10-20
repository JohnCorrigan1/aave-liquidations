use crate::pb;
use pb::pool;
use substreams::store::{StoreAdd, StoreAddFloat64, StoreAddInt64, StoreNew};

#[substreams::handlers::store]
pub fn store_liquidations(liquidations: pool::Liquidations, s: StoreAddInt64) {
    s.add(
        0,
        "num_liquidations",
        liquidations.liquidations.len() as i64,
    )
}

#[substreams::handlers::store]
pub fn store_wsteth_liquidations(liquidations: pool::TokenLiquidations, s: StoreAddFloat64) {
    let mut steth_count: f64 = 0.0;
    for liquidation in &liquidations.token_liquidations {
        steth_count += liquidation.liquidated_collateral_amount;
    }
    s.add(
        0,
        "wsteth_liquidations",
        liquidations.token_liquidations.len() as f64,
    );
    s.add(0, "steth_liquidated", steth_count);
}

#[substreams::handlers::store]
pub fn store_weth_liquidations(liquidations: pool::TokenLiquidations, s: StoreAddFloat64) {
    let mut eth_count: f64 = 0.0;
    for liquidation in &liquidations.token_liquidations {
        eth_count += liquidation.liquidated_collateral_amount;
    }
    s.add(
        0,
        "num_liquidations",
        liquidations.token_liquidations.len() as f64,
    );
    s.add(0, "eth_liquidated", eth_count);
}

#[substreams::handlers::store]
pub fn store_wbtc_liquidations(liquidations: pool::TokenLiquidations, s: StoreAddFloat64) {
    let mut btc_count: f64 = 0.0;
    for liquidation in &liquidations.token_liquidations {
        btc_count += liquidation.liquidated_collateral_amount;
    }
    s.add(
        0,
        "wbtc_liquidations",
        liquidations.token_liquidations.len() as f64,
    );
    s.add(0, "btc_liquidated", btc_count);
}

#[substreams::handlers::store]
pub fn store_aave_liquidations(liquidations: pool::TokenLiquidations, s: StoreAddFloat64) {
    let mut aave_count: f64 = 0.0;
    for liquidation in &liquidations.token_liquidations {
        aave_count += liquidation.liquidated_collateral_amount;
    }
    s.add(
        0,
        "aave_liquidations",
        liquidations.token_liquidations.len() as f64,
    );
    s.add(0, "aave_liquidated", aave_count);
}
