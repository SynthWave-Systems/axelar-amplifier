#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

use connection_router::state;

use crate::error::ContractError;
use crate::execute;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, CONFIG};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let config = Config {
        service_name: msg.service_name,
        service_registry: deps.api.addr_validate(&msg.service_registry_address)?,
        source_gateway_address: msg.source_gateway_address,
        voting_threshold: msg.voting_threshold,
        block_expiry: msg.block_expiry,
        confirmation_height: msg.confirmation_height,
    };
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_event(config.into()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::VerifyMessages { messages } => {
            if messages.is_empty() {
                return Err(ContractError::EmptyMessages {});
            }

            // todo: extract to conversion function
            let messages = messages
                .into_iter()
                .map(state::Message::try_from)
                .collect::<Result<Vec<state::Message>, _>>()?;

            if messages
                .iter()
                .any(|message| !message.source_chain.eq(&messages[0].source_chain))
            {
                return Err(ContractError::SourceChainMismatch {});
            }

            execute::verify_messages(deps, env, messages)
        }
        ExecuteMsg::Vote { poll_id, votes } => execute::vote(deps, info, poll_id, votes),
        ExecuteMsg::EndPoll { poll_id } => execute::end_poll(deps, poll_id),
    }
}
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    todo!()
}