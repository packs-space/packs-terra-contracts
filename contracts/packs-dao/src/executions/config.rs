use cosmwasm_std::{attr, DepsMut, Env, MessageInfo, Response};

use crate::error::ContractError;
use crate::state::config::{Config, CONFIG};

pub fn configure(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    name: Option<String>,
    logo: Option<String>,
    metadata: Option<String>,
) -> Result<Response, ContractError> {
    if info.sender != env.contract.address {
        return Err(ContractError::Unauthorized {});
    }

    let mut config: Config = CONFIG.load(deps.storage)?;

    if let Some(name) = name {
        config.name = name;
    }

    if let Some(_) = logo {
        config.logo = logo;
    }

    if let Some(_) = metadata {
        config.metadata = metadata;
    }

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attributes(vec![
        attr("action", "configure"),
        attr("sender", info.sender.to_string()),
    ]))
}
