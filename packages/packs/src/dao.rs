use cosmwasm_std::{Binary, CosmosMsg, Empty};
use cw_core_macros::voting_query;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ModuleInstantiateMsg {
    pub code_id: u64,
    pub msg: Binary,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub name: String,
    pub logo: Option<String>,
    pub metadata: Option<String>,
    pub group_module: ModuleInstantiateMsg,
    pub gov_module: ModuleInstantiateMsg,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Configure {
        name: Option<String>,
        logo: Option<String>,
        metadata: Option<String>,
    },
    ExecuteProposalHook {
        msgs: Vec<CosmosMsg<Empty>>,
    },
}

#[voting_query]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// Gets the contract's config. Returns Config.
    Config {},
    /// Gets the proposal modules associated with the contract. Returns Vec<Addr>.
    ProposalModules {
        start_at: Option<String>,
        limit: Option<u32>,
    },
    /// Gets the contract's voting module. Returns Addr.
    VotingModule {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ConfigResponse {
    pub name: String,
    pub logo: Option<String>,
    pub metadata: Option<String>,
    pub group_contract_address: Option<String>,
    pub gov_contract_address: Option<String>,
}
