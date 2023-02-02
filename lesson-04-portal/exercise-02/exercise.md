<!---
Course: 2
Lesson: 4
Exercise: 2

Title: Preparing Portal to Become the Minter Part 2
Filename: contract.rs

Storyline placeholder:
>
-->

Moving right along, let's enable these two new state saving entry points in `contract.rs`. 

Since the decentralized identity system won't work without the contract addressses of Potion and `passport-token`, we also need to add them to `instantiate`.

After adding our new parameters the messages for `instantiate` and `execute`, in Portal, look like this

```rs
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
```

Let's go ahead and enable these messages as contract entry points in `contract.rs`.

# Exercise

1. Add the two new required imports (`set_passport_contract` and `set_potion_contract`) to the `use` statement for `crate::execute_fns`. To keep our imports tidy, add them in alphabetical order
2. In the `match` statement for the `execute` entry point, add the `ExecuteMsg`s for the functions you imported in step 1
3. In the `instantiate` entry point, modify the `config` variable so it has all the required fields of `Config` (have a look at `state.rs` if you're confused)

# Starter

```rs
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::query_fns::{jump_ring_check, minimum_sapience};
use crate::execute_fns::{
    initiate_jump_ring_travel, set_minimum_sapience, // Add the first import on this line
    set_planet_name, set_sapient_names, // Add the second import on this line
};
use crate::state::{Config, CONFIG};

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::JumpRingPreCheck { traveler } => jump_ring_check(traveler),
        QueryMsg::MinimumSapience {} => minimum_sapience(deps),
    }
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SetPlanetName { to } => set_planet_name(to, deps, info),
        ExecuteMsg::SetSapientNames { to } => set_sapient_names(to, deps, info),
        ExecuteMsg::SetMinimumSapience { to } => set_minimum_sapience(to, deps, info),
        // `ExecuteMsg` for setting the passport contract goes here
        // `ExecuteMsg` for setting the potion contract goes here
        ExecuteMsg::JumpRingTravel { to } => initiate_jump_ring_travel(to, deps, env, info),
    }
}

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let config = Config {
        owner: info.sender,
        planet_name: msg.planet_name,
        planet_sapients: msg.planet_sapients,
        minimum_sapience: msg.minimum_sapience,
        // Add `passport_contract` here
        // Add `potion_contract`
    };
    CONFIG.save(deps.storage, &config)?;
    Ok(Response::new()
        .add_attribute("owner", config.owner)
        .add_attribute("minimum_sapience", config.minimum_sapience.as_str()))
}
```

# Answer

```rs
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::query_fns::{jump_ring_check, minimum_sapience};
use crate::execute_fns::{
    initiate_jump_ring_travel, set_minimum_sapience, set_passport_contract, 
    set_planet_name, set_potion_contract, set_sapient_names,
};
use crate::state::{Config, CONFIG};

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::JumpRingPreCheck { traveler } => jump_ring_check(traveler),
        QueryMsg::MinimumSapience {} => minimum_sapience(deps),
    }
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SetPlanetName { to } => set_planet_name(to, deps, info),
        ExecuteMsg::SetSapientNames { to } => set_sapient_names(to, deps, info),
        ExecuteMsg::SetMinimumSapience { to } => set_minimum_sapience(to, deps, info),
        ExecuteMsg::SetPassportContract { contract } => set_passport_contract(contract, deps, info),
        ExecuteMsg::SetPotionContract { contract } => set_potion_contract(contract, deps, info),
        ExecuteMsg::JumpRingTravel { to, traveler } => initiate_jump_ring_travel(to, traveler, deps, env, info),
    }
}

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let config = Config {
        owner: info.sender,
        planet_name: msg.planet_name,
        planet_sapients: msg.planet_sapients,
        minimum_sapience: msg.minimum_sapience,
        passport_contract: msg.passport_contract,
        potion_contract: msg.potion_contract,
    };
    CONFIG.save(deps.storage, &config)?;
    Ok(Response::new()
        .add_attribute("owner", config.owner)
        .add_attribute("minimum_sapience", config.minimum_sapience.as_str()))
}
```
