use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Storage};
use cosmwasm_storage::{singleton, singleton_read, ReadonlySingleton, Singleton};

use secret_toolkit::storage::{Keymap};

use crate::types::PingInfo;

pub static CONFIG_KEY: &[u8] = b"config";
pub static USER_PINGS_KEY: &[u8] = b"user_pings";
pub static ROAD_SEGMENTS_KEY: &[u8] = b"road_segments";

pub static USER_PINGS: Keymap<Addr, Vec<PingInfo>> = Keymap::new(USER_PINGS_KEY);

// Coordinate will be used to create a unique ID refering to a tile from a map
// Each tile will have a list of previous ping information allowing us to measure
// and compute traffic information
pub static ROAD_SEGMENTS: Keymap<u32, Vec<PingInfo>> = Keymap::new(ROAD_SEGMENTS_KEY);


#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct State {}


pub fn config(storage: &mut dyn Storage) -> Singleton<State> {
    singleton(storage, CONFIG_KEY)
}

pub fn config_read(storage: &dyn Storage) -> ReadonlySingleton<State> {
    singleton_read(storage, CONFIG_KEY)
}
