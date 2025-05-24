use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::types::{Coordinate, PingInfo, TrafficEvent};



#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {

    Ping { 
        info: PingInfo,
    },
    ShareEvent { 
        event_type: TrafficEvent,
        coordinate: Coordinate,
        timestamps: u64,
    },

    // TODO: add an event to compute best path
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    GetCount {},
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct CountResponse {
    pub count: i32,
}
