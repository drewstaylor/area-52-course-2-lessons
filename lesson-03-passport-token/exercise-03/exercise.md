<!---
Course: 2
Lesson: 3
Exercise: 3

Title: Writing the Instantiate Entry Point Part 1
Filename: lib.rs

Storyline placeholder:
>
-->

What if you wanted to change a contract that's already been deployed? Cosmos blockchains have a convenient way for smart contracts to migrate their code. Code migrations allow you to update the code, while keeping the same contract address. For obvious reasons, only the contract's admin (usually the Cosmos address who deployed the contract initially) can do this. 

Additionally, contracts undergoing migration can choose to retain all, or some, of their state. This provides developers with a safe way of resolving bugs, and adding new features, without impacting their users.

Conveniently, CosmWasm provides the [cw2](https://docs.rs/cw2/0.13.4/cw2/) library for helping manage code versions for contracts that have been, or may become, migrated.

`cw2` compliant contracts store the following data

- key: `contract_info`
- data: Json-serialized `ContractVersion`

```rs
pub struct ContractVersion {
    pub contract: String,
    pub version: String,
}
```

Once serialized (e.g. by [Serde]()) that data might look like this

```json
{
    "contract": "crates.io:my-cosmwasm-project",
    "version": "v0.1.0"
}
```

Instantiating a contract is the first migration

```rs
#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let contract_name: &str = "crates.io:my-cosmwasm-project";
    let contract_version: &str = env!("CARGO_PKG_VERSION");
    set_contract_version(deps.storage, contract_name, contract_version)?;
    
    // More code ...
    
    Ok(Response::default())
}
```

# Exercise

1. Create two Rust [constants](https://doc.rust-lang.org/std/keyword.const.html) called `CONTRACT_NAME` and `CONTRACT_VERSION`, and explictly enforce their types as `&str`
2. Set the value of `CONTRACT_NAME` to `"crates.io:passport-token"`
3. For the value of `CONTRACT_VERSION`, use CosmWasm's `env` macro and pass it the `"CARGO_PKG_VERSION"` flag
4. In `instantiate`, make a call to `cw2`'s [set_contract_version](https://docs.rs/cw2/0.13.4/cw2/fn.set_contract_version.html). The parameters to send to the function are the `storage` attribute from `deps`, followed by `CONTRACT_NAME` and then `CONTRACT_VERSION`

# Starter
```rs
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Empty, Addr};
use cw2::set_contract_version;

pub use cw721_soulbound::{ContractError, InstantiateMsg};
pub use universe::species::SapienceScale;

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
pub type Cw721MetadataContract<'a> = cw721_soulbound::Cw721Contract<'a, Extension, Empty, Empty, Empty>;

// `CONTRACT_NAME` goes here
// `CONTRACT_VERSION` goes here

pub mod entry {
    use super::*;

    #[cfg(not(feature = "library"))]
    use cosmwasm_std::entry_point;
    use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, StdResult};

    #[cfg_attr(not(feature = "library"), entry_point)] // This allows other developers to use our project a libarary
    pub fn instantiate(
        deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        _msg: InstantiateMsg,
    ) -> StdResult<Response> {
        // Set contract version here

        Ok(Response::default())
    }
}
```

# Answer

```rs
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Empty, Addr};
use cw2::set_contract_version;

pub use cw721_soulbound::{ContractError, InstantiateMsg};
pub use universe::species::SapienceScale;

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
pub type Cw721MetadataContract<'a> = cw721_soulbound::Cw721Contract<'a, Extension, Empty, Empty, Empty>;

const CONTRACT_NAME: &str = "crates.io:passport-token";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub mod entry {
    use super::*;

    #[cfg(not(feature = "library"))]
    use cosmwasm_std::entry_point;
    use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, StdResult};

    #[cfg_attr(not(feature = "library"), entry_point)] // This allows other developers to use our project a libarary
    pub fn instantiate(
        deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        _msg: InstantiateMsg,
    ) -> StdResult<Response> {
        set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

        Ok(Response::default())
    }
}
```