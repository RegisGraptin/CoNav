use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// Scale factor for coordinates
// @dev: avoid storing & managing float numbers
const COORD_PRECISION: i32 = 1_000_000;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub enum TrafficEvent {
    TrafficJam,
    Accident,
    RoadClosure,
    Police,
    Hazard,
    Construction,
    Other(String),
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct Coordinate {
    pub lat: i32,
    pub lon: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct PingInfo {
    pub timestamp: u64,
    pub speed: u32,
    pub direction: u16,
    pub coordinate: Coordinate,
}


impl Coordinate {
    pub fn tile_id(&self) -> u32 {
        // Define tile granularity: 0.01° -> 100 grid per degree
        let lat_idx = (self.lat + 90_000) / 100;  // shift to positive
        let lon_idx = (self.lon + 180_000) / 100;

        // Combine into a unique u32 using bitwise or simple packing
        (lat_idx as u32) << 16 | (lon_idx as u32)
    }
}