use crate::state::config::{Config, CONFIG};
use cosmwasm_std::{Addr, Deps};

use crate::error::ContractError;

pub fn proposal_modules(
    deps: Deps,
    start_at: Option<String>,
    limit: Option<u32>,
) -> Result<Vec<Addr>, ContractError> {
    let config: Config = CONFIG.load(deps.storage)?;

    let gov_contract_address = config
        .gov_contract_address
        .ok_or(ContractError::NoGovModule {})?;

    Ok(vec![gov_contract_address])
}

pub fn voting_module(deps: Deps) -> Result<Addr, ContractError> {
    let config: Config = CONFIG.load(deps.storage)?;

    let group_contract_address = config
        .group_contract_address
        .ok_or(ContractError::NoGroupModule {})?;

    Ok(group_contract_address)
}
