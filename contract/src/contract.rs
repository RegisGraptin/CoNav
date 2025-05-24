use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
};

use crate::msg::{CountResponse, ExecuteMsg, InstantiateMsg, QueryMsg, TrafficEventResponse, TrafficInfoResponse};
use crate::state::{config_read, State, ROAD_SEGMENTS, ROAD_SEGMENT_EVENTS, USER_PINGS, USER_PINGS_KEY};
use crate::types::{Coordinate, EventInfo, PingInfo, TrafficEvent, TrafficInfo};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    // let state = State {
    //     owner: info.sender.clone(),
    // };

    // deps.api
    //     .debug(format!("Contract was initialized by {}", info.sender).as_str());
    // config(deps.storage).save(&state)?;

    Ok(Response::default())
}

#[entry_point]
pub fn execute(deps: DepsMut, env: Env, msg_info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    match msg {
        ExecuteMsg::Ping { 
            info: user_info 
        } => try_ping(deps, msg_info, env, user_info),
        ExecuteMsg::ShareEvent {
            coordinate,
            event
        } => try_share_event(deps, msg_info, env, coordinate, event)
    }
}

pub fn try_ping(
    deps: DepsMut, 
    msg_info: MessageInfo, 
    env: Env, 
    user_info: PingInfo,
) -> StdResult<Response> {
    // Get user address
    let sender_address = msg_info.sender.clone();

    // Compute the tile map coordinate
    let tile_id = user_info.coordinate.tile_id();

    // Load previous ping info from the user
    let mut pings = USER_PINGS
        .get(deps.storage, &sender_address)
        .unwrap_or_else(|| Vec::new());

    // Add the new one & save it
    pings.push(user_info.clone());
    USER_PINGS.insert(deps.storage, &sender_address, &pings)?;


    // Load existing pings for the segment or start a new list
    let mut segment_pings = ROAD_SEGMENTS
        .get(deps.storage, &tile_id)
        .unwrap_or_else(|| Vec::new());

    let now = env.block.time.seconds();
    let max_age = 60 * 30; // e.g., keep last 30 minutes

    segment_pings.retain(|ping| now - ping.timestamp <= max_age);
    segment_pings.push(user_info.clone());
    ROAD_SEGMENTS.insert(deps.storage, &tile_id, &segment_pings)?;

    Ok(Response::default())
}

pub fn try_share_event(
    deps: DepsMut, 
    msg_info: MessageInfo, 
    env: Env, 
    coordinate: Coordinate,
    event: EventInfo,
) -> StdResult<Response> {

    let tile_id = coordinate.tile_id();
        
    let mut events = ROAD_SEGMENT_EVENTS
        .get(deps.storage, &tile_id)
        .unwrap_or_else(|| Vec::new());

    // Add the new one & save it
    events.push(event.clone());
    ROAD_SEGMENT_EVENTS.insert(deps.storage, &tile_id, &events)?;
    
    Ok(Response::default())
}





#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetTrafficEvent { coordinate } => to_binary(&query_traffic_event(deps, env, coordinate)?),
        QueryMsg::GetTrafficInfo { coordinate } => to_binary(&query_traffic_info(deps, env, coordinate)?),
    }
}

fn query_traffic_event(
    deps: Deps, 
    env: Env, 
    coordinate: Coordinate
) -> StdResult<TrafficEventResponse> {

    let tile_id = coordinate.tile_id();
    
    let mut events = ROAD_SEGMENT_EVENTS
        .get(deps.storage, &tile_id)
        .unwrap_or_else(|| Vec::new());

    let now = env.block.time.seconds();
    let max_age = 60 * 30; // e.g., keep last 30 minutes

    events.retain(|event| now - event.timestamps <= max_age);
    
    Ok(TrafficEventResponse { events: events })
}

fn query_traffic_info(
    deps: Deps, 
    env: Env, 
    coordinate: Coordinate
) -> StdResult<TrafficInfoResponse> {

    let tile_id = coordinate.tile_id();

    // Fetch the road segment information
    let mut infos = ROAD_SEGMENTS
        .get(deps.storage, &tile_id)
        .unwrap_or_else(|| Vec::new());

    // Filter on the last one
    let now = env.block.time.seconds();
    let max_age = 60 * 30; // e.g., keep last 30 minutes

    infos.retain(|info| now - info.timestamp <= max_age);
    
    // Compute the data
    let count: u32 = infos.len() as u32;
    let total_speed: u32 = infos.iter().map(|info| info.speed).sum();
    let avg_speed = total_speed / count;

    Ok(TrafficInfoResponse { 
        info: TrafficInfo {
            count, 
            avg_speed
        }
    })
}


// #[cfg(test)]
// mod tests {
//     use super::*;
//     use cosmwasm_std::testing::*;
//     use cosmwasm_std::{from_binary, Coin, StdError, Uint128};

//     #[test]
//     fn proper_initialization() {
//         let mut deps = mock_dependencies();
//         let info = mock_info(
//             "creator",
//             &[Coin {
//                 denom: "earth".to_string(),
//                 amount: Uint128::new(1000),
//             }],
//         );
//         let init_msg = InstantiateMsg { count: 17 };

//         // we can just call .unwrap() to assert this was a success
//         let res = instantiate(deps.as_mut(), mock_env(), info, init_msg).unwrap();

//         assert_eq!(0, res.messages.len());

//         // it worked, let's query the state
//         let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
//         let value: CountResponse = from_binary(&res).unwrap();
//         assert_eq!(17, value.count);
//     }

//     #[test]
//     fn increment() {
//         let mut deps = mock_dependencies_with_balance(&[Coin {
//             denom: "token".to_string(),
//             amount: Uint128::new(2),
//         }]);
//         let info = mock_info(
//             "creator",
//             &[Coin {
//                 denom: "token".to_string(),
//                 amount: Uint128::new(2),
//             }],
//         );
//         let init_msg = InstantiateMsg { count: 17 };

//         let _res = instantiate(deps.as_mut(), mock_env(), info, init_msg).unwrap();

//         // anyone can increment
//         let info = mock_info(
//             "anyone",
//             &[Coin {
//                 denom: "token".to_string(),
//                 amount: Uint128::new(2),
//             }],
//         );

//         let exec_msg = ExecuteMsg::Increment {};
//         let _res = execute(deps.as_mut(), mock_env(), info, exec_msg).unwrap();

//         // should increase counter by 1
//         let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
//         let value: CountResponse = from_binary(&res).unwrap();
//         assert_eq!(18, value.count);
//     }

//     #[test]
//     fn reset() {
//         let mut deps = mock_dependencies_with_balance(&[Coin {
//             denom: "token".to_string(),
//             amount: Uint128::new(2),
//         }]);
//         let info = mock_info(
//             "creator",
//             &[Coin {
//                 denom: "token".to_string(),
//                 amount: Uint128::new(2),
//             }],
//         );
//         let init_msg = InstantiateMsg { count: 17 };

//         let _res = instantiate(deps.as_mut(), mock_env(), info, init_msg).unwrap();

//         // not anyone can reset
//         let info = mock_info(
//             "anyone",
//             &[Coin {
//                 denom: "token".to_string(),
//                 amount: Uint128::new(2),
//             }],
//         );
//         let exec_msg = ExecuteMsg::Reset { count: 5 };

//         let res = execute(deps.as_mut(), mock_env(), info, exec_msg);

//         match res {
//             Err(StdError::GenericErr { .. }) => {}
//             _ => panic!("Must return unauthorized error"),
//         }

//         // only the original creator can reset the counter
//         let info = mock_info(
//             "creator",
//             &[Coin {
//                 denom: "token".to_string(),
//                 amount: Uint128::new(2),
//             }],
//         );
//         let exec_msg = ExecuteMsg::Reset { count: 5 };

//         let _res = execute(deps.as_mut(), mock_env(), info, exec_msg).unwrap();

//         // should now be 5
//         let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
//         let value: CountResponse = from_binary(&res).unwrap();
//         assert_eq!(5, value.count);
//     }
// }
