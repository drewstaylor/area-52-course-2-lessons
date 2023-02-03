<!---
Course: 2
Lesson: 4
Exercise: 5

Title: Minting NFTs From a Contract Part 3
Filename: execute_fns.rs
-->

> Hey, maybe it was all just a dream. Just another cyborg fever dream, a biological quirk of your electro-chemical psyche. These Portal modifications, wouldn't it be so great to take the credit for that idea? Of course it was all you. You're so clever, cyborg.

In a moment we'll finish the minting logic, but first let's think about `passport-token`'s `ExecuteMsg::Mint`. Its message type looks like this:

```rs
pub struct MintMsg<T> {
    /// Unique ID of the NFT
    pub token_id: String,
    /// The owner of the newly minter NFT
    pub owner: String,
    /// Universal resource identifier for this NFT
    /// Should point to a JSON file that conforms to the ERC721
    /// Metadata JSON Schema
    pub token_uri: Option<String>,
    /// Any custom extension used by this contract
    pub extension: T,
}
```

The [generic](https://doc.rust-lang.org/rust-by-example/generics.html) `T`, of `MintMsg<T>` represents an abstract type which could be any valid on-chain NFT metadata enforced by a token collection contract.

In the `passport-token` collection contract we enforced, and exported, a type called `Metadata`, and we also exported a [type alias](https://doc.rust-lang.org/reference/items/type-aliases.html) called `Extension`:

```rs
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct Metadata {
    pub name: Option<String>,
    pub description: Option<String>,
    pub image: Option<String>,
    pub dna: Option<String>,
    pub species: Option<String>,
    pub sapience_level: Option<SapienceScale>,
    pub issuer: Option<Addr>,
    pub origin: Option<String>,
    pub identity: Option<Addr>,
}

pub type Extension = Option<Metadata>;
```

That means for the purposes of minting a `passport-token`, the `Extension` type will be enforced as `T` in the `MintMsg<T>` of `cw721-soulbound`, which looks something like this:

```rs
let metadata_extension = Some(Metadata {
    name: Some("some traveler".to_string()),
    description: Some("some description".into()),
    image: Some("ipfs://QmZdPdZzZum2jQ7jg1ekfeE3LSz1avAaa42G6mfimw9TEn".into()),
    dna: Some("some dna".to_string()),
    species: Some("some cyborg".to_string()),
    sapience_level: Some(SapienceScale::High),
    issuer: Some(Addr::unchecked("archway1yvnw8xj5elngcq95e2n2p8f80zl7shfwyxk88858pl6cgzveeqtqy7xtf7")),
    origin: Some("some planet".),
    identity: Some(Addr::unchecked("archway1f395p0gg67mmfd5zcqvpnp9cxnu0hg6r9hfczq")),
});
```

We mentioned the `T` of `MintMsg<T>` is a Rust [generic](https://doc.rust-lang.org/rust-by-example/generics.html), lets think about why it allows token collection contracts extending from `cw721-soulbound` to enforce any valid on-chain metadata.

### Rust Generics

[Generics](https://doc.rust-lang.org/rust-by-example/generics.html) in Rust is a way of generalizing types and functionalities to broader cases. This is useful for making things like NFTs which require a lot diversity, but using [generics](https://doc.rust-lang.org/rust-by-example/generics.html) calls for rather involved syntax. The simplest and most common use of generics is for type parameters. 

For example, defining a generic function named foo that takes an argument `T` of any type:
```rs
fn foo<T>(arg: T) { ... }
```

In `passport-token` the type parameter that we're interested in (`MintMsg<T>` and `extension: T`) is represented by the `Extension` type.

### Option in Rust

The `Option` type represents an optional value. Every `Option` is either `Some` (contains a value), or `None` (does not contain a value). For NFTs created using `cw721-base` (or `cw721-soulbound`) using `Option` is a best practice, since it allows for greater diversity and rarer NFTs to occur within the collection.

# Exercise

1. Create a variable called `mint_msg` that explictly enforces the `ExecuteMsg` type from `passport_token`. Its value will be a call to `Cw721ExecuteMsg::Mint` which takes a `Cw721MintMsg` struct as its argument.
2. `Cw721MintMsg` will have four struct fields: `token_id`, `owner`, `token_uri` and `extension`.
3. `token_id`'s value can be cloned from `msg.identity`, this way we can use a holder's address to query if they have an NFT. It also helps ensure travelers can't hold multiple passports. Use [into](https://doc.rust-lang.org/std/convert/trait.Into.html) to make sure it gets the correct type.
4. `owner`'s value can also be set from `msg.identity`, but since it's the last time we need to use it you won't need to clone it. Use [into](https://doc.rust-lang.org/std/convert/trait.Into.html) to make sure it gets the correct type.
5. Since we're using `cw721` with on-chain metadata, `token_uri` can be set to `None`.
6. Set `extension` to the `metadata_extension` variable created in the previous exercise.
7. Create a variable called `mint_resp` that explicitly enforces the `CosmosMsg` type. Set its value to the `Execute` variant of the `WasmMsg` enum. Write each of its members on their own line. `contract_addr` needs to be set to the `passport_contract` which is a state attribute of `config`, and use [into](https://doc.rust-lang.org/std/convert/trait.Into.html) to make sure it gets the correct type. Set `msg` to a reference to `mint_msg`, wrapped in a `to_binary` call, and don't forget to capture any errors that might occur using the `?` operator. Since we're already charging our service fee in the Potion contract, we don't require any funds for this contract execution, so the `funds` attribute can be set to an empty vector macro (`vec![]`).
8. To end the `mint_resp` declaration, put a closing brace on its own line then a call to [into](https://doc.rust-lang.org/std/convert/trait.Into.html) (also on its own line).

# Starter

```rs
use cosmwasm_std::{
    Addr, CosmosMsg, DepsMut, Env, MessageInfo, QueryRequest, 
    to_binary, Response, WasmMsg, WasmQuery,
};

use cw721::TokensResponse;
use passport_token::{
    ExecuteMsg as Cw721ExecuteMsg, Extension, Metadata, 
    MintMsg as Cw721MintMsg, QueryMsg as Cw721QueryMsg,
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

    let metadata_extension: Extension = Some(Metadata {
        name: Some(msg.name),
        description: Some(msg.description),
        image: Some(msg.image),
        dna: Some(msg.dna),
        species: Some(msg.species),
        sapience_level: Some(msg.sapience_level),
        issuer: Some(env.contract.address.clone()),
        origin: Some(config.planet_name),
        identity: Some(msg.identity.clone()),
    });

    // Create the `mint_msg` variable here

    // Create the `mint_resp` variable here

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
    Addr, CosmosMsg, DepsMut, Env, MessageInfo, QueryRequest, 
    to_binary, Response, WasmMsg, WasmQuery,
};

use cw721::TokensResponse;
use passport_token::{
    ExecuteMsg as Cw721ExecuteMsg, Extension, Metadata, 
    MintMsg as Cw721MintMsg, QueryMsg as Cw721QueryMsg,
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

    let metadata_extension: Extension = Some(Metadata {
        name: Some(msg.name),
        description: Some(msg.description),
        image: Some(msg.image),
        dna: Some(msg.dna),
        species: Some(msg.species),
        sapience_level: Some(msg.sapience_level),
        issuer: Some(env.contract.address.clone()),
        origin: Some(config.planet_name),
        identity: Some(msg.identity.clone()),
    });

    let mint_msg: passport_token::ExecuteMsg = Cw721ExecuteMsg::Mint(Cw721MintMsg {
        token_id: msg.identity.clone().into(),
        owner: msg.identity.into(),
        token_uri: None,
        extension: metadata_extension,
    });

    let mint_resp: CosmosMsg = WasmMsg::Execute {
        contract_addr: config.passport_contract.into(),
        msg: to_binary(&mint_msg)?,
        funds: vec![],
    }
    .into();

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
