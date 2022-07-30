#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult,
    SubMsg, WasmMsg,
};
use cw2::set_contract_version;
use cw721_base::ExecuteMsg as Cw721ExecuteMsg;
use cw_utils::parse_reply_instantiate_data;
use packs::lns::Metadata;

use crate::config::{Config, CONFIG};
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, MintMsg, QueryMsg};
use crate::state::{State, STATE};

const CONTRACT_NAME: &str = "crates.io:packs-dao-factory";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const INSTANTIATE_DAO_REPLY_ID: u64 = 0;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    CONFIG.save(deps.storage, &Config { lns: msg.lns })?;

    Ok(Response::new().add_attribute("action", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::InstantiateDao {
            code_id,
            label,
            msg,
            slug,
            metadata,
        } => execute_instantiate_dao(deps, env, code_id, msg, label, slug, metadata),
    }
}

pub fn execute_instantiate_dao(
    deps: DepsMut,
    env: Env,
    code_id: u64,
    msg: Binary,
    label: String,
    slug: String,
    metadata: Metadata,
) -> Result<Response, ContractError> {
    let msg = WasmMsg::Instantiate {
        admin: Some(env.contract.address.to_string()),
        code_id: code_id,
        msg: msg,
        funds: vec![],
        label: label,
    };

    STATE.save(deps.storage, &State { slug, metadata })?;

    let msg = SubMsg::reply_on_success(msg, INSTANTIATE_DAO_REPLY_ID);

    Ok(Response::new()
        .add_attribute("action", "instantiate_dao")
        .add_submessage(msg))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    match msg.id {
        INSTANTIATE_DAO_REPLY_ID => {
            let res = parse_reply_instantiate_data(msg);
            match res {
                Ok(res) => {
                    let dao_contract = deps.api.addr_validate(&res.contract_address)?;
                    // Transfer admin status to the DAO
                    let update_admin_msg = WasmMsg::UpdateAdmin {
                        contract_addr: dao_contract.to_string(),
                        admin: dao_contract.to_string(),
                    };

                    // Register DAO in LNS
                    let config = CONFIG.load(deps.storage)?;
                    let state = STATE.load(deps.storage)?;

                    let mut extension = state.metadata;
                    extension.contract_address = Some(dao_contract.to_string());

                    let lns_msg = CosmosMsg::Wasm(WasmMsg::Execute {
                        contract_addr: config.lns.to_string(),
                        msg: to_binary(&Cw721ExecuteMsg::Mint(MintMsg {
                            token_id: state.slug,
                            owner: dao_contract.to_string(),
                            token_uri: None,
                            extension,
                        }))?,
                        funds: vec![],
                    });

                    Ok(Response::default()
                        .add_attribute("dao_contract_address", dao_contract)
                        .add_message(update_admin_msg)
                        .add_message(lns_msg))
                }
                Err(_) => Err(ContractError::DaoContractInstantiateError {}),
            }
        }
        _ => Err(ContractError::UnknownReplyId { id: msg.id }),
    }
}
