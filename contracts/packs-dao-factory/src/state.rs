use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cw_storage_plus::Item;
use packs::lns::Metadata;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema, Default)]
pub struct State {
    pub slug: String,
    pub metadata: Metadata,
}

pub const STATE: Item<State> = Item::new("state");
