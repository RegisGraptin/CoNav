use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::types::{Coordinate, EventInfo, PingInfo, TrafficEvent};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {

    Ping { 
        info: PingInfo,
    },
    ShareEvent { 
        coordinate: Coordinate,
        event: EventInfo,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetTrafficInfo {
        coordinate: Coordinate
    },
    GetTrafficEvent {
        coordinate: Coordinate
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct TrafficEventResponse {
    pub events: Vec<EventInfo>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct TrafficInfoResponse {
    pub infos: Vec<TrafficEvent>,
}




// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct CountResponse {
    pub count: i32,
}
