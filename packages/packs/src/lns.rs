use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cw20::Logo;

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
pub struct Metadata {
    pub image: Option<String>,
    pub image_data: Option<Logo>,
    pub email: Option<String>,
    pub external_url: Option<String>,
    pub public_name: Option<String>,
    pub public_bio: Option<String>,
    pub twitter_id: Option<String>,
    pub discord_id: Option<String>,
    pub telegram_id: Option<String>,
    pub keybase_id: Option<String>,
    pub validator_operator_address: Option<String>,
    pub contract_address: Option<String>,
    /// For future compatibility, we want to support
    /// a recursive lookup of tokens that constitutes a path
    /// somewhat like a DNS
    /// if this is None then it is a base token
    pub parent_token_id: Option<String>,
    /// A public key
    pub pgp_public_key: Option<String>,
}
