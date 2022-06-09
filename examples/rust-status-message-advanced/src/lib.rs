use contract_utils::near_sdk::{
    self,
    borsh::{self, BorshDeserialize, BorshSerialize},
    near_bindgen,
    serde::{Deserialize, Serialize},
    witgen,
};

pub use contract_utils::prelude::*;

mod views;

/// A message that contains some text
#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
#[serde(crate = "near_sdk::serde")]
#[witgen]
pub struct Message {
    /// Inner string value
    /// @pattern ^TEXT:
    text: String,
}

#[near_bindgen("STATE")]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    message: Message,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            message: Message {
                text: "initial text".into(),
            },
        }
    }
}

#[near_bindgen]
impl Contract {
    /// A change call to set the message
    #[payable]
    pub fn set_message(&mut self, message: Message) {
        self.assert_owner();
        self.message = message;
    }
}