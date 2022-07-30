use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub name: String,
    pub logo: Option<String>,
    pub metadata: Option<String>,
    pub group_contract_address: Option<Addr>,
    pub gov_contract_address: Option<Addr>,
}

pub const CONFIG: Item<Config> = Item::new("config");
