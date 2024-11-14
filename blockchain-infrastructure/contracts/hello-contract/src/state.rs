use cosmwasm_std::Storage;
use cosmwasm_storage::{singleton, singleton_read, Singleton};
use serde::{Deserialize, Serialize};

pub const STATE: Singleton<State> = Singleton::new(b"state");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct State {
    pub message: String,
}
