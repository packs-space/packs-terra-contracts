use cosmwasm_std::{CosmosMsg, DepsMut, Empty, MessageInfo, Response};

use crate::error::ContractError;
use crate::state::config::{Config, CONFIG};

pub fn execute(
    deps: DepsMut,
    info: MessageInfo,
    msgs: Vec<CosmosMsg<Empty>>,
) -> Result<Response, ContractError> {
    let config: Config = CONFIG.load(deps.storage)?;

    let gov_contract_address = config
        .gov_contract_address
        .ok_or(ContractError::NoGovModule {})?;

    // Check that the message has come from gov module
    if gov_contract_address != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    Ok(Response::default()
        .add_attribute("action", "execute_proposal_hook")
        .add_messages(msgs))
}
