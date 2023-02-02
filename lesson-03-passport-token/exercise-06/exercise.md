<!---
Course: 2
Lesson: 3
Exercise: 6

Title: Don't Forget to Reply
Filename: lib.rs

Storyline placeholder:
>
-->

Now `passport-token`, our token collection contract, just needs one more thing. It already works, but it's methods can only be called by users. If we want to make it so other contracts can interact with it, we need to add the `reply` entry point.

The message struct for `Reply` can be imported from `cosmwasm_std`, and has the following struct fields:
```rs
pub struct Reply {
    pub id: u64,
    pub result: ContractResult<SubcallResponse>,
}
```

For a refresher on implementing `reply`, you can check out [this exercise](https://area-52.io/starting-with-cosm-wasm/4/reply:-a-new-entry-point) from [Course 1](https://area-52.io/starting-with-cosm-wasm). We'll be using the same implementation, except for the `Err` CosmWasm `Response`, which will be the `ContractError::Unauthorized {}` variant from `error.rs`.

# Exercise

Add the `reply` entry point, this is the last entry point our collection contract requires.

# Starter

```rs
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Empty, Addr, Reply, SubMsgResult};
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

    #[cfg_attr(not(feature = "library"), entry_point)]
    // Add the `reply` entry point here
}
```

# Answer

```rs
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Empty, Addr, Reply, SubMsgResult};
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

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn reply(_deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
        match msg.result {
            SubMsgResult::Ok(_) => Ok(Response::default()),
            SubMsgResult::Err(_) => Err(ContractError::Unauthorized {}),
        }
    }
}
```
