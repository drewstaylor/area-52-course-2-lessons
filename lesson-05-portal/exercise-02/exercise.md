<!---
Course: 2
Lesson: 5
Exercise: 2

Title: Verifying Identity With NFTs Part 2
Filename: execute_fns.rs
-->

> Err...(the onomatopoeia not the Rust kind), well actually, maybe you should write all this down so you don't forget again how everything is supposed to work. You chuckle to yourself, and cringe, knowing if those Librarian scribes at SECTION 31 saw you scribbling out your system design so furiously they'd be grinning.

Before teleporting through the `JumpRing` portal, we need to verify the `Traveler` is holding a valid passport. 

To send our query, we'll again be using the `query` function from `deps.querier`. The syntax of our query will resemble what we did earlier, when we used `deps.querier.query` in `mint_passport`:

```rs
// From `mint_passport`
let query_resp: TokensResponse = deps.querier.query(&query_req)?;
```

### The Passport Identity System So far

Some things to keep in mind:

- We're using soulbound NFTs
- Only the Portal contract (`JumpRing`) can mint
- Each NFT owner can only hold one `passport-token` at a time
- Token IDs are keyed by holder address
- Only the Potion contract (`Imbiber`) can call `mint_passport`

Identity theft already isn't possible because of `cw721-soulbound. Each [cw721](https://github.com/CosmWasm/cw-nfts/blob/main/packages/cw721/README.md) `token_id` must be unique, and the `token_id` of each `passport-token` is the wallet address of its holder. That means the query to `NftInfo` will fail if we try to teleport someone not holding a `passport-token` that matches their Cosmos address.

### Using NFT Metadata Returned by `NftInfo`

The `NftInfo` query returns a `Response` type called `NftInfoResponse` (which can be imported from `cw721`).

Since `passport-token` is using on-chain metadata, we can read the metadata of its NFTs using the `extension` field like this:

```rs
let query_req = QueryRequest::Wasm(WasmQuery::Smart {
    contract_addr: "address".to_string(),
    msg: to_binary(&query_msg).unwrap(),
});

let some_extension = query_req.extension;
```

Had we been using the off-chain metadata version of [cw721](https://github.com/CosmWasm/cw-nfts/blob/main/packages/cw721/README.md), we'd have to get the NFT's metadata from an external link like:

```rs
let query_req = QueryRequest::Wasm(WasmQuery::Smart {
    contract_addr: "address".to_string(),
    msg: to_binary(&query_msg).unwrap(),
});

let some_token_uri = query_req.token_uri;
```

Since the blockchain can't access data from [IPFS](https://ipfs.tech/), or hosted on some website. A smart contract querying the NFT's metadata, won't be able to parse or interpret the json stored at an external link (e.g. `"ipfs://bafybeigxa4ifta32fjl7yejgr6sddanwcgex5m2xxhatjzpms4iwh5bcvm/ascended.json"`) because the blockchain is an isolated universe, and doesn't know about the Internet or [IPFS](https://ipfs.tech/).

A powerful feature of [cw721](https://github.com/CosmWasm/cw-nfts/blob/main/packages/cw721/README.md) tokens with on-chain metadata, is that other smart contracts have access to, and can utilize, the rare and unique properties of NFTs. 

An example of that could look like this:

```rs
let query_req = QueryRequest::Wasm(WasmQuery::Smart {
    contract_addr: config.passport_contract.into(),
    msg: to_binary(&query_msg).unwrap(),
});

let some_extension = query_req.extension;

if some_extension.name.unwrap() == "Richard Bissell Jr".to_string() {
    // Do something ...
}
```

# Exercise 

Here's how we can make sure `Traveler`s teleporting through the `JumpRing` possess a valid `passport-token`.

1. From `cw721` import the `NftInfoResponse`.
2. In ` initiate_jump_ring_travel`, create a variable called `query_resp` that explicitly enforces `NftInfoResponse<Metadata>` as its type.
3. Assign `query_resp` a call to the `query` function, which is an attribute of the `querier` dependency (`DepsMut`). For its function argument, pass `query` a reference to `query_req`, and don't forget to capture any errors that could occur.
4. Write an `if` condition to verify the NFT's `identity` field matches the `traveler`'s Cosmos address that was forwarded to ` initiate_jump_ring_travel` by the Potion contract. You can the `identity` metadata field from `query_resp.extension`, but you'll have to [unwrap]() it. If `identity` and `traveler` are _not_ equal, return an [Err](https://doc.rust-lang.org/std/result/enum.Result.html) of type `ContractError::Unauthorized {}`.

# Starter

```rs
use cosmwasm_std::{
    Addr, CosmosMsg, DepsMut, Env, MessageInfo, QueryRequest, 
    to_binary, Response, WasmMsg, WasmQuery,
};

use cw721::{/* import the `Response` type for `NftInfo` here */, TokensResponse};
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

    let messages = vec![mint_resp];
    Ok(Response::new().add_messages(messages))
}

pub fn  initiate_jump_ring_travel(
    _to: Addr,
    traveler: Addr,
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    // Only potion contract can call this function
    let potion_contract = config.potion_contract;
    if info.sender != potion_contract {
        return Err(ContractError::Unauthorized {});
    }

    let query_msg: passport_token::QueryMsg<Extension> = Cw721QueryMsg::NftInfo {
        token_id: traveler.clone().into(),
    };
    let query_req = QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: config.passport_contract.into(),
        msg: to_binary(&query_msg).unwrap(),
    });

    // Create a variable called `query_resp` here
    
    // Create an if condition here to verify the NFT's `identity` matches the `traveler` address sent by `Imbiber`

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

use cw721::{NftInfoResponse, TokensResponse};
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

    let messages = vec![mint_resp];
    Ok(Response::new().add_messages(messages))
}

pub fn  initiate_jump_ring_travel(
    _to: Addr,
    traveler: Addr,
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    // Only potion contract can call this function
    let potion_contract = config.potion_contract;
    if info.sender != potion_contract {
        return Err(ContractError::Unauthorized {});
    }

    let query_msg: passport_token::QueryMsg<Extension> = Cw721QueryMsg::NftInfo {
        token_id: traveler.clone().into(),
    };
    let query_req = QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: config.passport_contract.into(),
        msg: to_binary(&query_msg).unwrap(),
    });

    let query_resp: NftInfoResponse<Metadata> = deps.querier.query(&query_req)?;
    
    if query_resp.extension.identity.unwrap() != traveler {
        return Err(ContractError::Unauthorized {});
    }

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
