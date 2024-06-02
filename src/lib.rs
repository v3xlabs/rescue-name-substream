mod abi;
mod pb;
use hex_literal::hex;
use pb::contract::v1 as contract;
use substreams::Hex;
use substreams_database_change::pb::database::DatabaseChanges;
use substreams_database_change::tables::Tables as DatabaseChangeTables;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables as EntityChangesTables;
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::Event;

#[allow(unused_imports)]
use num_traits::cast::ToPrimitive;
use std::str::FromStr;
use substreams::scalar::BigDecimal;

substreams_ethereum::init!();

const RESCUENAME_TRACKED_CONTRACT: [u8; 20] = hex!("8c82dd2f5Ad2E4F70d2710Cc5290e0D80e42191B");

fn map_rescuename_events(blk: &eth::Block, events: &mut contract::Events) {
    events.rescuename_name_addeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == RESCUENAME_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::rescuename_contract::events::NameAdded::match_and_decode(log) {
                        return Some(contract::RescuenameNameAdded {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            name: event.name,
                            vault: event.vault.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.rescuename_name_removeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == RESCUENAME_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::rescuename_contract::events::NameRemoved::match_and_decode(log) {
                        return Some(contract::RescuenameNameRemoved {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            name: event.name,
                            vault: event.vault.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.rescuename_ownership_transferreds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == RESCUENAME_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::rescuename_contract::events::OwnershipTransferred::match_and_decode(log) {
                        return Some(contract::RescuenameOwnershipTransferred {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            new_owner: event.new_owner,
                            user: event.user,
                        });
                    }

                    None
                })
        })
        .collect());
    events.rescuename_rescue_name_vault_createds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == RESCUENAME_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::rescuename_contract::events::RescueNameVaultCreated::match_and_decode(log) {
                        return Some(contract::RescuenameRescueNameVaultCreated {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            owner: event.owner,
                            vault_id: event.vault_id.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
}

fn db_rescuename_out(events: &contract::Events, tables: &mut DatabaseChangeTables) {
    // Loop over all the abis events to create table changes
    events.rescuename_name_addeds.iter().for_each(|evt| {
        tables
            .create_row("rescuename_name_added", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("name", &evt.name)
            .set("vault", BigDecimal::from_str(&evt.vault).unwrap());
    });
    events.rescuename_name_removeds.iter().for_each(|evt| {
        tables
            .create_row("rescuename_name_removed", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("name", &evt.name)
            .set("vault", BigDecimal::from_str(&evt.vault).unwrap());
    });
    events.rescuename_ownership_transferreds.iter().for_each(|evt| {
        tables
            .create_row("rescuename_ownership_transferred", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("new_owner", Hex(&evt.new_owner).to_string())
            .set("user", Hex(&evt.user).to_string());
    });
    events.rescuename_rescue_name_vault_createds.iter().for_each(|evt| {
        tables
            .create_row("rescuename_rescue_name_vault_created", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("owner", Hex(&evt.owner).to_string())
            .set("vault_id", BigDecimal::from_str(&evt.vault_id).unwrap());
    });
}


fn graph_rescuename_out(events: &contract::Events, tables: &mut EntityChangesTables) {
    // Loop over all the abis events to create table changes
    events.rescuename_name_addeds.iter().for_each(|evt| {
        tables
            .create_row("rescuename_name_added", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("name", &evt.name)
            .set("vault", BigDecimal::from_str(&evt.vault).unwrap());
    });
    events.rescuename_name_removeds.iter().for_each(|evt| {
        tables
            .create_row("rescuename_name_removed", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("name", &evt.name)
            .set("vault", BigDecimal::from_str(&evt.vault).unwrap());
    });
    events.rescuename_ownership_transferreds.iter().for_each(|evt| {
        tables
            .create_row("rescuename_ownership_transferred", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("new_owner", Hex(&evt.new_owner).to_string())
            .set("user", Hex(&evt.user).to_string());
    });
    events.rescuename_rescue_name_vault_createds.iter().for_each(|evt| {
        tables
            .create_row("rescuename_rescue_name_vault_created", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("owner", Hex(&evt.owner).to_string())
            .set("vault_id", BigDecimal::from_str(&evt.vault_id).unwrap());
    });
}

#[substreams::handlers::map]
fn map_events(blk: eth::Block) -> Result<contract::Events, substreams::errors::Error> {
    let mut events = contract::Events::default();
    map_rescuename_events(&blk, &mut events);
    Ok(events)
}

#[substreams::handlers::map]
fn db_out(events: contract::Events) -> Result<DatabaseChanges, substreams::errors::Error> {
    // Initialize Database Changes container
    let mut tables = DatabaseChangeTables::new();
    db_rescuename_out(&events, &mut tables);
    Ok(tables.to_database_changes())
}

#[substreams::handlers::map]
fn graph_out(events: contract::Events) -> Result<EntityChanges, substreams::errors::Error> {
    // Initialize Database Changes container
    let mut tables = EntityChangesTables::new();
    graph_rescuename_out(&events, &mut tables);
    Ok(tables.to_entity_changes())
}
