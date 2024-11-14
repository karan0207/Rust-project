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

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::SetMessage { message } => try_set_message(deps, info, message),
    }
}

fn try_set_message(
    deps: DepsMut,
    _info: MessageInfo,
    message: String,
) -> StdResult<Response> {
    STATE.update(deps.storage, |mut state| -> StdResult<_> {
        state.message = message.clone();
        Ok(state)
    })?;
    Ok(Response::default())
}


#[entry_point]
pub fn query(deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<String> {
    let state = STATE.load(deps.storage)?;
    Ok(state.message)
}