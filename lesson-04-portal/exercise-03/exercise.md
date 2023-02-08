<!---
Course: 2
Lesson: 4
Exercise: 3

Title: Minting NFTs From a Contract Part 1
Filename: execute_fns.rs
-->

> Buckle up travelers, your intergalactic learning curve is about to ramp up really quickly.

Previously, we devised our identity tokens (`passport-token`), and now we've got to use the `passport-token` in Portal and design our identity system.

Portal functions like an intergalactic embassy and passport control combined. Most pre-flight security checks occur in [Potion](https://github.com/phi-labs-ltd/area-52-courses/tree/main/01_Starting_with_CosmWasm/potion/src/execute_fns), but identity verification happens in the `JumpRing` itself (which can only be called by Potion). 

An Earthling might say that Potion operates like airport security, while Portal handles boarding passengers and flights. If you don't have a passport, Portal can create one for you but you must apply for the passport through Potion.

### Querying Cw721 Tokens

`WasmQuery::Smart` will send our queries to the token collection contract (`passport-token`). If you need a refresher on `WasmQuery` check it out [here](https://area-52.io/starting-with-cosm-wasm/4/check_sapience_level-function-part-1) and [here](https://area-52.io/starting-with-cosm-wasm/4/check_sapience_level-function-part-2).

We want to know if a specific user has any passports. The `Tokens` query entry point (read more [here](https://docs.rs/cw721/latest/cw721/enum.Cw721QueryMsg.html) and [here](https://docs.rs/cw721/latest/cw721/trait.Cw721Query.html#tymethod.tokens)) of `cw721` tokens accepts an `owner` address as a parameter, so we'll use that to perform our check.

```rs
let query_msg = QueryMsg::Tokens {
    owner: "archway1f395p0gg67mmfd5zcqvpnp9cxnu0hg6r9hfczq".to_string(),
    start_after: None,
    limit: None,
};
```

`Tokens` returns a [JSON](https://en.wikipedia.org/wiki/JSON) array. If `owner` has 0 NFTs, the response array will be empty. If they own 1 or more tokens, it returns an array of the owned `token_id`s.

### Enummerability in Cw721

`cw721` tokens are [enumerable](https://github.com/CosmWasm/cw-nfts/blob/main/packages/cw721/README.md#enumerable), which is implemented using [pagination](https://github.com/CosmWasm/cw-nfts/blob/main/packages/cw721/README.md#enumerable). 

Use the `limit` and `start_after` parameters of the `Tokens` query for pulling data for those greedy whales holding _a lot_ of NFTs. 

By default (e.g. `limit` and `start_after` set to `None`), enumerable queries return the first 100 records. 

If an `owner` holds more than 100 tokens, the second page of results can be queried by setting `start_after` to `100`:

```rs
let query_msg = QueryMsg::Tokens {
    owner: "archway1f395p0gg67mmfd5zcqvpnp9cxnu0hg6r9hfczq".to_string(),
    start_after: 100,
    limit: None,
};
```

# Exercise

Let's begin working on a new entry point function called `mint_passport`. Before minting tokens, we'll add some restrictions (so owners cannot hold more than one passport at a time).

1. `QueryMsg` has been provided, but you'll need to handle things from there. Create a variable called `query_req` and assign it to `QueryRequest::Wasm`. The function argument to pass to the `QueryRequest` is a `WasmQuery::Smart` enum, whose members (`contract_addr` and `msg`) can be written on separate lines.
2. For `WasmQuery`'s first member (`contract_addr`), assign the passport contract's address (accessed from `config.passport_contract`), but we'll need it again later so you'll have to [clone](https://doc.rust-lang.org/std/clone/trait.Clone.html) it and use [into](https://doc.rust-lang.org/std/convert/trait.Into.html) to get the right type.
3. For `WasmQuery`'s second member (`msg`), assign a reference to `query_msg`, but don't forget to convert it to binary and [unwrap](https://docs.rs/unwrap/latest/unwrap/) it.
4. The final line of the `query_req` declaration closes both the `WasmQuery` enum and the `QueryRequest`.
5. Create a variable called `query_resp` that explicitly enforces a `TokensResponse` type (imported from [cw721](https://docs.rs/cw721/latest/cw721/struct.TokensResponse.html)).
6. For its value, call the `query` function from `querier.query` which can be accessed from `deps`. The argument to be sent to `query` is a reference to the `query_req` created in steps 1 through 4, and don't forget to capture any errors that might occur.
7. If the address already owns a passport, fail with an `IllegalAlien` contract error (see `error.rs` for more information).

# Starter

```rs
use cosmwasm_std::{
    Addr, DepsMut, Env, MessageInfo, QueryRequest, 
    to_binary, Response, WasmQuery,
};

use cw721::TokensResponse;
use passport_token::{
    Extension, QueryMsg as Cw721QueryMsg,
};

use crate::error::ContractError;
use crate::state::config;

use universe::species::{SapienceScale, Sapient};

pub fn mint_passport(
    msg: MintMsg,
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    
    // Only potion contract can call this function
    let potion_contract = config.potion_contract;
    if info.sender != potion_contract {
        return Err(ContractError::Unauthorized {});
    }

    // Minting fails if user already owns a passport
    let query_msg: passport_token::QueryMsg<Extension> = Cw721QueryMsg::Tokens {
        owner: msg.identity.clone().into(),
        start_after: None,
        limit: None,
    };
    // Create the `query_req` variable here
    // Create the `query_resp` variable here
    if !query_resp.tokens.is_empty() {
        // Throw the `IllegalAlien` error here
    }

    Ok(Response::default())
}

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

# Answer

```rs
use cosmwasm_std::{
    Addr, DepsMut, Env, MessageInfo, QueryRequest, 
    to_binary, Response, WasmQuery,
};

use cw721::TokensResponse;
use passport_token::{
    Extension, QueryMsg as Cw721QueryMsg,
};

use crate::error::ContractError;
use crate::state::config;

use universe::species::{SapienceScale, Sapient};

pub fn mint_passport(
    msg: MintMsg,
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    
    // Only potion contract can call this function
    let potion_contract = config.potion_contract;
    if info.sender != potion_contract {
        return Err(ContractError::Unauthorized {});
    }

    // Minting fails if user already owns a passport
    let query_msg: passport_token::QueryMsg<Extension> = Cw721QueryMsg::Tokens {
        owner: msg.identity.clone().into(),
        start_after: None,
        limit: None,
    };
    let query_req = QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: config.passport_contract.clone().into(),
        msg: to_binary(&query_msg).unwrap(),
    });
    let query_resp: TokensResponse = deps.querier.query(&query_req)?;
    if !query_resp.tokens.is_empty() {
        return Err(ContractError::IllegalAlien {});
    }

    Ok(Response::default())
}

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
