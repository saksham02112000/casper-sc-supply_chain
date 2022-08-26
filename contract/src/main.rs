#![no_std]
#![no_main]

use casper_contract::contract_api::{account, runtime, storage, system};
use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::{runtime_args, CLType, CLValue, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Key, Parameter, ApiError, RuntimeArgs};

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;

use alloc::string::String;
use crate::alloc::string::ToString;
use alloc::vec;

// use casper_contract::{
//     contract_api::{runtime, storage},
//     unwrap_or_revert::UnwrapOrRevert,
// };
// use casper_types::{ApiError, Key};

#[derive(Debug)]
struct SupplyChain {
    order_name: String,
    temp_readings: u8,
}

// const KEY_NAME: &str = "my-key-name";
// const RUNTIME_ARG_NAME: &str = "message";
// Creating constants for the various contract entry points.
const ENTRY_POINT_INIT: &str = "init";
const ENTRY_POINT_INSERT_ORDER: &str = "insert_order";
const GET_POINT_INSERT_ORDER: &str = "get_order_details";

const SUPPLY_CHAIN_HISTORY: &str = "supply_chain_history";
const HISTORY: &str = "history";
const COUNTER: &u64 = &0u64;
const ORDER_PLACING_ACCOUNT: &str = "order_placing_account";

/// An error enum which can be converted to a `u16` so it can be returned as an `ApiError::User`.
#[repr(u16)]
enum FundRaisingError {
    MissingLedgerSeedURef = 0,
    // KeyMismatch = 1,
}

impl From<FundRaisingError> for ApiError {
    fn from(error: FundRaisingError) -> Self {
        ApiError::User(error as u16)
    }
}

#[no_mangle]
pub extern "C" fn init() {
    // let fundraising_purse = system::create_purse();
    // runtime::put_key(FUNDRAISING_PURSE, fundraising_purse.into());
    // Create a dictionary to track the mapping of account hashes to number of donations made.
    storage::new_dictionary(HISTORY).unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn get_order_details(dictionary_key: String) {
    let history_seed_uref = *runtime::get_key(HISTORY)
        .unwrap_or_revert_with(FundRaisingError::MissingLedgerSeedURef)
        .as_uref()
        .unwrap_or_revert();
    let history_info = if let Some(history_arr) =
    storage::dictionary_get::<u64>(history_seed_uref, &dictionary_key)
        .unwrap_or_revert()
    {
        history_arr
    } else {
        0
    };
    runtime::ret(CLValue::from_t(history_info).unwrap_or_revert())
}

#[no_mangle]
pub extern "C" fn insert_order(dictionary_item_data: String, value: i64) {

    // match storage::dictionary_get::<u64> { counter_seed_ure, & }
    // Acquiring the LEDGER seed URef to properly assign the dictionary item.
    let history_seed_uref = *runtime::get_key(HISTORY)
        .unwrap_or_revert_with(FundRaisingError::MissingLedgerSeedURef)
        .as_uref()
        .unwrap_or_revert();

    // This identifies an item within the dictionary and either creates or updates the associated value.
    match storage::dictionary_get::<u64>(history_seed_uref, &dictionary_item_data ).unwrap_or_revert()
    // match storage::dictionary_get::<u64>(ledger_seed_uref, &dictionary_item_key).unwrap_or_revert()
    {
        None => storage::dictionary_put(history_seed_uref, &dictionary_item_data, value),
        Some(current_number_of_donations) => storage::dictionary_put(
            history_seed_uref,
            &dictionary_item_data,
            value,
        ),
    }
}


#[no_mangle]
pub extern "C" fn call() {
    // // The key shouldn't already exist in the named keys.
    // let missing_key = runtime::get_key(KEY_NAME);
    // if missing_key.is_some() {
    //     runtime::revert(Error::KeyAlreadyExists);
    // }
    //
    // // This contract expects a single runtime argument to be provided.  The arg is named "message"
    // // and will be of type `String`.
    // let value: String = runtime::get_named_arg(RUNTIME_ARG_NAME);
    //
    // // Store this value under a new unforgeable reference a.k.a `URef`.
    // let value_ref = storage::new_uref(value);
    //
    // // Store the new `URef` as a named key with a name of `KEY_NAME`.
    // let key = Key::URef(value_ref);
    // runtime::put_key(KEY_NAME, key);
    //
    // // The key should now be able to be retrieved.  Note that if `get_key()` returns `None`, then
    // // `unwrap_or_revert()` will exit the process, returning `ApiError::None`.
    // let retrieved_key = runtime::get_key(KEY_NAME).unwrap_or_revert();
    // if retrieved_key != key {
    //     runtime::revert(Error::KeyMismatch);
    // }

    let init_entry_point = EntryPoint::new(
        ENTRY_POINT_INIT,
        vec![],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );

    // This establishes the `donate` entry point for callers looking to donate.
    let insert_order_entry_point = EntryPoint::new(
        ENTRY_POINT_INSERT_ORDER,
        // Vec::new(),
        vec![],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );


    let get_order_entry_point = EntryPoint::new(
        GET_POINT_INSERT_ORDER,
        // Vec::new(),
        vec![],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );

    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(init_entry_point);
    entry_points.add_entry_point(insert_order_entry_point);
    entry_points.add_entry_point(get_order_entry_point);


    let (contract_hash, _contract_version) = storage::new_contract(
        entry_points,
        None,
        Some("supply_chain_package_hash".to_string()),
        Some("supply_chain_access_uref".to_string()),
    );

    runtime::put_key("supply_chain_contract_hash", contract_hash.into());
    // Call the init entry point to setup and create the fundraising purse
    // and the ledger to track donations made.
    runtime::call_contract::<()>(contract_hash, ENTRY_POINT_INIT, runtime_args! {})

}
