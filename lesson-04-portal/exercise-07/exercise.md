<!---
Course: 2
Lesson: 4
Exercise: 7

Title: Finalizing the Mint Passport Entry Point
Filename: contract.rs
-->

> Just as you're ready to put the final touches on `mint_passport` your memory of the fever dream is fading. What did it say in **Appendix cbc471**? Something about on-flight boarding protocols and security in ` initiate_jump_ring_travel`, or was it in `jump_ring_check`. You're having trouble remembering... _(to be continued)_

Now that our [entry point](https://docs.rs/cosmwasm-std/latest/cosmwasm_std/attr.entry_point.html) function is completed, we can import and use it in `execute`.

Take a moment to review the function arguments for `mint_passport`:

```rs
pub fn mint_passport(
    msg: MintMsg,
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> { ... }
```

Its `MintMsg` type (which is different from the `MintMsg<Extension>` of `passport-token`) is the type we've been working with in the previous exercises:

```rs
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MintMsg {
    pub name: String,
    pub description: String,
    pub image: String,
    pub dna: String,
    pub species: String,
    pub sapience_level: SapienceScale,
    pub identity: Addr,
}
```

Which is the on-chain `cw721` metadata of each passport NFT. 

# Exercise 

Let's go ahead and import `mint_passport` and make sure we call it with the correct parameters (`msg`, `deps`, `env`, and `info`). Once the `ExecuteMsg` for `MintPassport` has been added to `execute` in `contract.rs`, minting passports (and our passport controls) will be enabled!

1. Locate the imports for the `execute_fns` crate and import the `mint_passport` function we recently finalized.
2. In the `execute` entry point, add the `ExecuteMsg` for `MintPassport` and point it to the `mint_passport` function imported in step 1. Have a look at `msg.rs` and `execute_fns.rs` if you're confused about any parameters.

# Starter

```rs
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::query_fns::{jump_ring_check, minimum_sapience};
use crate::execute_fns::{
     initiate_jump_ring_travel, /* Import minting here */, set_minimum_sapience, set_passport_contract, 
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
        // Add the `ExecuteMsg` for `MintPassport` here
        ExecuteMsg::JumpRingTravel { to } =>  initiate_jump_ring_travel(to, deps, env, info),
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

# Answer

```rs
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::query_fns::{jump_ring_check, minimum_sapience};
use crate::execute_fns::{
    initiate_jump_ring_travel, mint_passport, set_minimum_sapience, set_passport_contract, 
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
        ExecuteMsg::MintPassport { msg } => mint_passport(msg, deps, env, info),
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
