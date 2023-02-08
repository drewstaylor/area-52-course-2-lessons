<!---
Course: 2
Lesson: 4
Exercise: 1

Title: Preparing Portal to Become the Minter Part 1
Filename: execute_fns.rs
-->

> With slight trepidation you run the diagnostic test. `test integration_tests::use_metadata_extension ... ok`, it worked! You watch as the **STATUS** screen on SECTION 31's Ring updates itself. Now it reads "Online" (in cybernetic-green).

Remember the `JumpRing` portal? Looking at SECTION 31's Ring, it seems so much more sophisticated.

Now that we have our token collection contract and non-transferable NFTs (with on-chain metadata), we're going to implement them in Portal. 

We're creating a secure (and decentralized) identity system for `Traveler` flight control. The system works like this:

1. Travel passports (`passport-token`s) are non-transferable (`cw721-soulbound`)
2. Only a Portal contract (`JumpRing`) can mint passports (`Cw721MetadataContract::default().minter`)
3. A Traveler can hold only one passport at a time
4. `token_id`'s are wallet addresses of their owner (helpful for lookups and proofs)
5. Only the Potion contract can call the `JumpRing` Portal (since Potion enforces payment and cyberdization checks)

### A State Saving Refresher

Last time we worked on Portal, we created two functions in `execute_fns.rs` to save state. Both are nearly identical, with the only difference being the storage key they're saving to (`planet_name` vs `planet_sapients`).

```rs
pub fn set_planet_name(
    to: String,
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let mut state = config(deps.storage).load()?;
    if info.sender != state.owner {
        return Err(ContractError::Unauthorized {});
    }
    state.planet_name = to.clone();
    config(deps.storage).save(&state)?;
    Ok(Response::new().add_attribute("action", "set_planet_name"))
}
```

We'll need to save a couple more items for the identity system to work. More specifically, Portal needs to know two Cosmos addresses: 

1. The address of the Potion contract allowed to call `initiate_jump_ring_travel` and `mint_passport`
2. The address of the `passport-token` to be called in order to mint the passport

# Exercise

Let's create the code to save the Cosmos addresses of `Imbiber` (Potion) and `passport-token`.

1. Create a public function called `set_passport_contract`, with function arguments `contract` (`Addr` type), `deps` and `info` (the last two types can be inferred from `set_planet_name` and `set_sapient_names`)
2. The logic in the function enclosure will resemble that of `set_planet_name` and `set_sapient_names`, but the storage key to be saved will be `contract`, and the value of `"action"` in `add_attribute` will be `set_passport_contract`
3. Create a public function called `set_potion_contract`, with function arguments `contract` (`Addr` type), `deps` and `info` (the last two types can be inferred from `set_planet_name` and `set_sapient_names`)
4. The logic in the function enclosure will resemble that of `set_planet_name` and `set_sapient_names`, but the storage key to be saved will be `contract`, and the value of `"action"` in `add_attribute` will be `set_potion_contract`

# Starter

```rs
use crate::error::ContractError;
use crate::state::config;
use cosmwasm_std::{Addr, DepsMut, MessageInfo, Response};
use universe::species::{SapienceScale, Sapient};

pub fn  initiate_jump_ring_travel(
    _to: Addr,
    _deps: DepsMut,
    _info: MessageInfo,
) -> Result<Response, ContractError> {
    Ok(Response::default())
}

pub fn set_minimum_sapience(
    to: SapienceScale,
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let mut state = config(deps.storage).load()?;
    if info.sender != state.owner {
        return Err(ContractError::Unauthorized {});
    }
    state.minimum_sapience = to;
    config(deps.storage).save(&state)?;
    Ok(Response::default())
}

pub fn set_planet_name(
    to: String,
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let mut state = config(deps.storage).load()?;
    if info.sender != state.owner {
        return Err(ContractError::Unauthorized {});
    }
    state.planet_name = to.clone();
    config(deps.storage).save(&state)?;
    Ok(Response::new().add_attribute("action", "set_planet_name"))
}

pub fn set_sapient_names(
    to: Vec<Sapient>,
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let mut state = config(deps.storage).load()?;
    if info.sender != state.owner {
        return Err(ContractError::Unauthorized {});
    }
    state.planet_sapients = to;
    config(deps.storage).save(&state)?;
    Ok(Response::new().add_attribute("action", "set_sapient_names"))
}

// Create `set_passport_contract` here

// Create `set_potion_contract` here
```

# Answer

```rs
use crate::error::ContractError;
use crate::state::config;
use cosmwasm_std::{Addr, DepsMut, MessageInfo, Response};
use universe::species::{SapienceScale, Sapient};

pub fn  initiate_jump_ring_travel(
    _to: Addr,
    _deps: DepsMut,
    _info: MessageInfo,
) -> Result<Response, ContractError> {
    Ok(Response::default())
}

pub fn set_minimum_sapience(
    to: SapienceScale,
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let mut state = config(deps.storage).load()?;
    if info.sender != state.owner {
        return Err(ContractError::Unauthorized {});
    }
    state.minimum_sapience = to;
    config(deps.storage).save(&state)?;
    Ok(Response::default())
}

pub fn set_planet_name(
    to: String,
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let mut state = config(deps.storage).load()?;
    if info.sender != state.owner {
        return Err(ContractError::Unauthorized {});
    }
    state.planet_name = to.clone();
    config(deps.storage).save(&state)?;
    Ok(Response::new().add_attribute("action", "set_planet_name"))
}

pub fn set_sapient_names(
    to: Vec<Sapient>,
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let mut state = config(deps.storage).load()?;
    if info.sender != state.owner {
        return Err(ContractError::Unauthorized {});
    }
    state.planet_sapients = to;
    config(deps.storage).save(&state)?;
    Ok(Response::new().add_attribute("action", "set_sapient_names"))
}

pub fn set_passport_contract(
    contract: Addr,
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;
    if info.sender != config.owner {
        return Err(ContractError::Unauthorized {});
    }
    config.passport_contract = contract;
    CONFIG.save(deps.storage, &config)?;
    Ok(Response::new().add_attribute("action", "set_passport_contract"))
}

pub fn set_potion_contract(
    contract: Addr,
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;
    if info.sender != config.owner {
        return Err(ContractError::Unauthorized {});
    }
    config.potion_contract = contract;
    CONFIG.save(deps.storage, &config)?;
    Ok(Response::new().add_attribute("action", "set_potion_contract"))
}
```
