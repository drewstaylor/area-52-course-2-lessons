use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use universe::species::{SapienceScale, Sapient, Traveler};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum QueryMsg {
    JumpRingPreCheck { traveler: Traveler },
    MinimumSapience {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum ExecuteMsg {
    SetPlanetName { to: String },
    SetSapientNames { to: Vec<Sapient> },
    SetMinimumSapience { to: SapienceScale },
    SetPassportContract { contract: Addr },
    SetPotionContract { contract: Addr },
    JumpRingTravel { to: Addr, traveler: Addr, },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub planet_name: String,
    pub planet_sapients: Vec<Sapient>,
    pub minimum_sapience: SapienceScale,
    pub passport_contract: Addr,
    pub potion_contract: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JumpRingCheckResponse {
    pub valid: bool,
}