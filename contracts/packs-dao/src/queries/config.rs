use cosmwasm_std::Deps;
use packs::dao::ConfigResponse;

use crate::error::ContractError;
use crate::state::config::{Config, CONFIG};

pub fn config(deps: Deps) -> Result<ConfigResponse, ContractError> {
    let config: Config = CONFIG.load(deps.storage)?;

    Ok(ConfigResponse {
        name: config.name,
        logo: config.logo,
        metadata: config.metadata,
        group_contract_address: config.group_contract_address.map(|addr| addr.to_string()),
        gov_contract_address: config.gov_contract_address.map(|addr| addr.to_string()),
    })
}
