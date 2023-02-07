<!---
Course: 2
Lesson: 3
Exercise: 5

Title: Moving on to Query and Execute
Filename: lib.rs
-->

> Excellent work so far. Just a few more tweaks and (hopefully) you can teleport home, but I guess you missed lunch after all. Will you make it home in time for dinner?

It's time to set up `query` and `execute` for our token collection contract. After that, the `passport-token` package will be mostly complete. `query` and `execute` will use the defaults from `cw721-soulbound` that we created in [Lesson 2](). 

Consider how we used the [type alias](https://doc.rust-lang.org/reference/items/type-aliases.html) `Cw721MetadataContract`, in `instantiate`, to access storage methods for the keys `contract_info` and `minter`:

```rs
// Saving to the `contract_info` key
Cw721MetadataContract::default().contract_info.save(deps.storage, &info)?;
```

```rs
// Saving to the `minter` key
Cw721MetadataContract::default().minter.save(deps.storage, &minter)?;
```

The default entry point functions from `cw721-soulbound` can be accessed in the same way (`Cw721MetadataContract::default().query`, `Cw721MetadataContract::default().execute`, etc.).

# Exercise

You might recall from [Course 1](https://area-52.io/starting-with-cosm-wasm/2/execute-vs-query) that `query` uses `Deps` which is read only, while `execute` and `instantiate` use `DepsMut` which is mutable.

1. Write the execute entry point. Its arguments are `deps`, `env`, `info` and `msg`, each of which must be passed to the `execute` default from `Cw721MetadataContract`. Since`cw721_soulbound::ExecuteMsg<Extension, Empty>` uses multiple [generics](https://doc.rust-lang.org/rust-by-example/generics.html), we've provided you with a [type alias](https://doc.rust-lang.org/reference/items/type-aliases.html) (`ExecuteMsg`) for convenience. You'll find it below the metadata schema and other type aliases.
2. Write the query entry point. Its arguments are `deps`, `env` and `msg`. `QueryMsg` from `cw721-soulbound` implements a [generic](https://doc.rust-lang.org/rust-by-example/generics.html) but we won't be needing it, so set it to an `Empty` value (e.g. `QueryMsg<Empty>`).

# Starter

```rs
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Empty, Addr};
use cw2::set_contract_version;

pub use cw721_soulbound::{ContractError, InstantiateMsg, QueryMsg};
pub use universe::species::SapienceScale;

pub use cw721::{ContractInfoResponse};

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
pub type ExecuteMsg = cw721_soulbound::ExecuteMsg<Extension, Empty>;

const CONTRACT_NAME: &str = "crates.io:passport-token";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub mod entry {
    use super::*;

    #[cfg(not(feature = "library"))]
    use cosmwasm_std::entry_point;
    use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn instantiate(
        deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        msg: InstantiateMsg,
    ) -> StdResult<Response> {
        set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

        let info = ContractInfoResponse {
            name: msg.name,
            symbol: msg.symbol,
        };
        Cw721MetadataContract::default().contract_info.save(deps.storage, &info)?;

        let minter = deps.api.addr_validate(&msg.minter)?;
        Cw721MetadataContract::default().minter.save(deps.storage, &minter)?;

        Ok(Response::default())
    }

    #[cfg_attr(not(feature = "library"), entry_point)]
    // Write the `execute` entry point here

    #[cfg_attr(not(feature = "library"), entry_point)]
    // Write the `query` entry point here
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

pub use cw721::{ContractInfoResponse};

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
pub type ExecuteMsg = cw721_soulbound::ExecuteMsg<Extension, Empty>;

const CONTRACT_NAME: &str = "crates.io:passport-token";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub mod entry {
    use super::*;

    #[cfg(not(feature = "library"))]
    use cosmwasm_std::entry_point;
    use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn instantiate(
        deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        msg: InstantiateMsg,
    ) -> StdResult<Response> {
        set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

        let info = ContractInfoResponse {
            name: msg.name,
            symbol: msg.symbol,
        };
        Cw721MetadataContract::default().contract_info.save(deps.storage, &info)?;

        let minter = deps.api.addr_validate(&msg.minter)?;
        Cw721MetadataContract::default().minter.save(deps.storage, &minter)?;

        Ok(Response::default())
    }

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, ContractError> {
        Cw721MetadataContract::default().execute(deps, env, info, msg)
    }

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn query(
        deps: Deps,
        env: Env,
        msg: QueryMsg<Empty>
    ) -> StdResult<Binary> {
        Cw721MetadataContract::default().query(deps, env, msg)
    }
}
```
