#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Reply, ReplyOn, Response, StdError, SubMsg,
    WasmMsg,
};
use cw2::set_contract_version;
use cw_utils::parse_reply_instantiate_data;
use packs::dao::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};

use crate::error::ContractError;
use crate::executions;
use crate::queries;
use crate::state::config::{Config, CONFIG};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:packs-dao";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// replies
const GROUP_MODULE_REPLY_ID: u64 = 0;
const GOV_MODULE_REPLY_ID: u64 = 1;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let config = Config {
        name: msg.name,
        logo: msg.logo,
        metadata: msg.metadata,
        group_contract_address: None,
        gov_contract_address: None,
    };
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_submessage(SubMsg {
            msg: WasmMsg::Instantiate {
                admin: Some(env.contract.address.to_string()),
                code_id: msg.group_module.code_id,
                msg: msg.group_module.msg,
                funds: vec![],
                label: "packs-dao-group-module".to_string(),
            }
            .into(),
            gas_limit: None,
            id: GROUP_MODULE_REPLY_ID,
            reply_on: ReplyOn::Success,
        })
        .add_submessage(SubMsg {
            msg: WasmMsg::Instantiate {
                admin: Some(env.contract.address.to_string()),
                code_id: msg.gov_module.code_id,
                msg: msg.gov_module.msg,
                funds: vec![],
                label: "packs-dao-gov-module".to_string(),
            }
            .into(),
            gas_limit: None,
            id: GOV_MODULE_REPLY_ID,
            reply_on: ReplyOn::Success,
        }))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Configure {
            name,
            logo,
            metadata,
        } => executions::config::configure(deps, env, info, name, logo, metadata),
        ExecuteMsg::ExecuteProposalHook { msgs } => {
            executions::proposal_hook::execute(deps, info, msgs)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    match msg {
        QueryMsg::Config {} => Ok(to_binary(&queries::config::config(deps)?)?),
        QueryMsg::ProposalModules { start_at, limit } => Ok(to_binary(
            &queries::module::proposal_modules(deps, start_at, limit)?,
        )?),
        QueryMsg::TotalPowerAtHeight { height } => Ok(to_binary(
            &queries::voting::total_power_at_height(deps, height)?,
        )?),
        QueryMsg::VotingModule {} => Ok(to_binary(&queries::module::voting_module(deps)?)?),
        QueryMsg::VotingPowerAtHeight { address, height } => Ok(to_binary(
            &queries::voting::voting_power_at_height(deps, address, height)?,
        )?),
        QueryMsg::Info {} => Ok(to_binary(&queries::voting::info(deps)?)?),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    // Don't do any state migrations.
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    match msg.id {
        GROUP_MODULE_REPLY_ID => {
            let res = parse_reply_instantiate_data(msg)?;
            let group_contract_address = deps.api.addr_validate(&res.contract_address)?;
            CONFIG.update(deps.storage, |mut config| -> Result<_, ContractError> {
                config.group_contract_address = Some(group_contract_address);
                Ok(config)
            })?;
            Ok(Response::default().add_attribute("group_module".to_string(), res.contract_address))
        }
        GOV_MODULE_REPLY_ID => {
            let res = parse_reply_instantiate_data(msg)?;
            let gov_contract_address = deps.api.addr_validate(&res.contract_address)?;
            CONFIG.update(deps.storage, |mut config| -> Result<_, ContractError> {
                config.gov_contract_address = Some(gov_contract_address);
                Ok(config)
            })?;
            Ok(Response::default().add_attribute("gov_module".to_string(), res.contract_address))
        }
        _ => Err(ContractError::Std(StdError::generic_err(
            "invalid reply id or result",
        ))),
    }
}
