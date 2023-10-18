mod abi;
mod pb;
use hex_literal::hex;
use pb::eth::erc721::v1 as erc721;
use pb::liquidations as pool;
use substreams::{key, prelude::*};
use substreams::{log, store::StoreAddInt64, Hex};
use substreams_database_change::pb::database::DatabaseChanges;
use substreams_database_change::tables::Tables;
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
                block_num: log.block_index().to_string(),
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
const NULL_ADDRESS: &str = "0000000000000000000000000000000000000000";

/// Store the total balance of NFT tokens for the specific TRACKED_CONTRACT by holder
#[substreams::handlers::store]
fn store_liquidations(transfers: erc721::Transfers, s: StoreAddInt64) {
    log::info!("NFT holders state builder");
    for transfer in transfers.transfers {
        if transfer.from != NULL_ADDRESS {
            log::info!("Found a transfer out {}", Hex(&transfer.trx_hash));
            s.add(transfer.ordinal, generate_key(&transfer.from), -1);
        }

        if transfer.to != NULL_ADDRESS {
            log::info!("Found a transfer in {}", Hex(&transfer.trx_hash));
            s.add(transfer.ordinal, generate_key(&transfer.to), 1);
        }
    }
}

#[substreams::handlers::map]
fn db_out(
    clock: substreams::pb::substreams::Clock,
    transfers: erc721::Transfers,
    owner_deltas: Deltas<DeltaInt64>,
) -> Result<DatabaseChanges, substreams::errors::Error> {
    let mut tables = Tables::new();
    for transfer in transfers.transfers {
        tables
            .create_row(
                "transfer",
                format!("{}-{}", &transfer.trx_hash, transfer.ordinal),
            )
            .set("trx_hash", transfer.trx_hash)
            .set("from", transfer.from)
            .set("to", transfer.to)
            .set("token_id", transfer.token_id)
            .set("ordinal", transfer.ordinal);
    }

    for delta in owner_deltas.into_iter() {
        let holder = key::segment_at(&delta.key, 1);
        let contract = key::segment_at(&delta.key, 2);

        tables
            .create_row("owner_count", format!("{}-{}", contract, holder))
            .set("contract", contract)
            .set("holder", holder)
            .set("balance", delta.new_value)
            .set("block_number", clock.number);
    }

    Ok(tables.to_database_changes())
}

// Bored Ape Club Contract
const TRACKED_CONTRACT: [u8; 20] = hex!("bc4ca0eda7647a8ab7c2061c2e118a18a936f13d");

/// Extracts transfers events from the contract
#[substreams::handlers::map]
fn map_transfers(blk: eth::Block) -> Result<Option<erc721::Transfers>, substreams::errors::Error> {
    let transfers: Vec<_> = blk
        .events::<abi::erc721::events::Transfer>(&[&TRACKED_CONTRACT])
        .map(|(transfer, log)| {
            substreams::log::info!("NFT Transfer seen");

            erc721::Transfer {
                trx_hash: Hex::encode(&log.receipt.transaction.hash),
                from: Hex::encode(&transfer.from),
                to: Hex::encode(&transfer.to),
                token_id: transfer.token_id.to_u64(),
                ordinal: log.block_index() as u64,
            }
        })
        .collect();
    if transfers.len() == 0 {
        return Ok(None);
    }

    Ok(Some(erc721::Transfers { transfers }))
}

//const NULL_ADDRESS: &str = "0000000000000000000000000000000000000000";

/// Store the total balance of NFT tokens for the specific TRACKED_CONTRACT by holder
#[substreams::handlers::store]
fn store_transfers(transfers: erc721::Transfers, s: StoreAddInt64) {
    log::info!("NFT holders state builder");
    for transfer in transfers.transfers {
        if transfer.from != NULL_ADDRESS {
            log::info!("Found a transfer out {}", Hex(&transfer.trx_hash));
            s.add(transfer.ordinal, generate_key(&transfer.from), -1);
        }

        if transfer.to != NULL_ADDRESS {
            log::info!("Found a transfer in {}", Hex(&transfer.trx_hash));
            s.add(transfer.ordinal, generate_key(&transfer.to), 1);
        }
    }
}

#[substreams::handlers::map]
fn db_out_nft(
    clock: substreams::pb::substreams::Clock,
    transfers: erc721::Transfers,
    owner_deltas: Deltas<DeltaInt64>,
) -> Result<DatabaseChanges, substreams::errors::Error> {
    let mut tables = Tables::new();
    for transfer in transfers.transfers {
        tables
            .create_row(
                "transfer",
                format!("{}-{}", &transfer.trx_hash, transfer.ordinal),
            )
            .set("trx_hash", transfer.trx_hash)
            .set("from", transfer.from)
            .set("to", transfer.to)
            .set("token_id", transfer.token_id)
            .set("ordinal", transfer.ordinal);
    }

    for delta in owner_deltas.into_iter() {
        let holder = key::segment_at(&delta.key, 1);
        let contract = key::segment_at(&delta.key, 2);

        tables
            .create_row("owner_count", format!("{}-{}", contract, holder))
            .set("contract", contract)
            .set("holder", holder)
            .set("balance", delta.new_value)
            .set("block_number", clock.number);
    }

    Ok(tables.to_database_changes())
}

fn generate_key(holder: &String) -> String {
    return format!("total:{}:{}", holder, Hex(TRACKED_CONTRACT));
}
