//! # Status Message Contract
//!
//! This is an example contract using owneable and deployable components
//!

use contract_utils::near_sdk::{
    self,
    borsh::{self, BorshDeserialize, BorshSerialize},
    serde::{Deserialize, Serialize},
};

/// Uses ownable to check owner before deploying contract
pub use contract_utils::prelude::*;

const MESSAGE_KEY: &str = "MESSAGE";

#[derive(BorshSerialize, BorshDeserialize, Deserialize, Serialize, Default)]
#[serde(crate = "near_sdk::serde")]
pub struct Message {
    text: String,
}

impl IntoKey for Message {
    fn into_storage_key() -> Vec<u8> {
        MESSAGE_KEY.as_bytes().to_vec()
    }
}

#[no_mangle]
pub fn update_message() {
    // any type can assert owner (weird Rust ;-))
    "a".assert_owner();

    // Deserialize input into Message
    let msg: Message = near_sdk::serde_json::from_slice(
        &near_sdk::env::input().expect("Expected input since method has arguments."),
    )
    .expect("Failed to deserialize input from JSON.");

    // set new message and get old message
    let old_message = Message::set_lazy(msg);

    // Serialize old message
    let result = near_sdk::serde_json::to_vec(&old_message)
        .expect("Failed to serialize the return value using JSON.");

    // Return serailazed result
    near_sdk::env::value_return(&result)
}

#[no_mangle]
pub fn get_message() {
    // Get message instance from storage and fail if doesn't exist
    let message = Message::get_lazy().get().unwrap();

    // Serialize Message
    let result = near_sdk::serde_json::to_vec(&message)
        .expect("Failed to serialize the return value using JSON.");

    // Return serelaized result
    near_sdk::env::value_return(&result)
}
