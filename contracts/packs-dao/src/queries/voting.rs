use cosmwasm_std::Deps;
use cw2::get_contract_version;
use cw_core_interface::voting;

use crate::error::ContractError;
use crate::state::config::{Config, CONFIG};

pub fn total_power_at_height(
    deps: Deps,
    height: Option<u64>,
) -> Result<voting::TotalPowerAtHeightResponse, ContractError> {
    let config: Config = CONFIG.load(deps.storage)?;

    let group_contract_address = config
        .group_contract_address
        .ok_or(ContractError::NoGroupModule {})?;

    let total_power: voting::TotalPowerAtHeightResponse = deps.querier.query_wasm_smart(
        group_contract_address,
        &voting::Query::TotalPowerAtHeight { height },
    )?;

    Ok(total_power)
}

pub fn voting_power_at_height(
    deps: Deps,
    address: String,
    height: Option<u64>,
) -> Result<voting::VotingPowerAtHeightResponse, ContractError> {
    let config: Config = CONFIG.load(deps.storage)?;

    let group_contract_address = config
        .group_contract_address
        .ok_or(ContractError::NoGroupModule {})?;

    let voting_power: voting::VotingPowerAtHeightResponse = deps.querier.query_wasm_smart(
        group_contract_address,
        &voting::Query::VotingPowerAtHeight { height, address },
    )?;

    Ok(voting_power)
}

pub fn info(deps: Deps) -> Result<cw_core_interface::voting::InfoResponse, ContractError> {
    let info = get_contract_version(deps.storage)?;
    Ok(cw_core_interface::voting::InfoResponse { info })
}
