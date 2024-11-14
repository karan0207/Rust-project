use cosmwasm_std::{Deps, DepsMut, Env, MessageInfo, Response, StdResult, entry_point};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let state = State {
        message: msg.message,
    };
    STATE.save(deps.storage, &state)?;
    Ok(Response::default())
}
