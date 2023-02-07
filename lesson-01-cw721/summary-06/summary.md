<!---
Course: 2 
Lesson: 1
Exercise: 6 (Summary)

Filename: lib.rs
-->

# Overview

In this lesson we examined the [cw721](https://github.com/CosmWasm/cw-nfts/blob/main/packages/cw721/README.md) standard for CosmWasm NFTs, and its relationship to [cw721-base](https://crates.io/crates/cw721-base) which is a basic implementation of a `cw721` NFT contract.

In general, we learned about:

- The entry points and functions provided by `cw721` as implemented in `cw721-base`
- How to mint NFTs using `cw721-base`
- Using `cw721` with off-chain metadata
- Creating a token collection contract

### **SOURCE CODE**
- <ExternalLink href="https://github.com/phi-labs-ltd/area-52-courses/">Building with NFTs repo</ExternalLink>

<!--- NEXT UP: -->
# Exercise

Modifying `cw721-base` with custom logic

# Starter

```rs
use cosmwasm_std::Empty;
use cw721_base::{
    ContractError, InstantiateMsg, MintMsg, 
    MinterResponse, QueryMsg
};
pub type Extension = Option<Empty>;
pub type Cw721Contract<'a> = cw721_base::Cw721Contract<'a, Extension, Empty, Empty, Empty>;
pub type ExecuteMsg = cw721_base::ExecuteMsg<Extension, Empty>;

pub mod entry {
    use super::*;

    #[cfg(not(feature = "library"))]
    use cosmwasm_std::entry_point;
    use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

    #[entry_point]
    pub fn instantiate(
        deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        msg: InstantiateMsg,
    ) -> StdResult<Response> {
        let minter = deps.api.addr_validate(&msg.minter)?;
        Cw721Contract::default().minter.save(deps.storage, &minter)?;
        Ok(Response::default())
    }

    #[entry_point]
    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, ContractError> {
        Cw721Contract::default().execute(deps, env, info, msg)
    }

    #[entry_point]
    pub fn query(deps: Deps, env: Env, msg: QueryMsg<Empty>) -> StdResult<Binary> {
        Cw721Contract::default().query(deps, env, msg)
    }
}
```

# Answer

```rs
use cosmwasm_std::Empty;
use cw721_base::{
    ContractError, InstantiateMsg, MintMsg, 
    MinterResponse, QueryMsg
};
pub type Extension = Option<Empty>;
pub type Cw721Contract<'a> = cw721_base::Cw721Contract<'a, Extension, Empty, Empty, Empty>;
pub type ExecuteMsg = cw721_base::ExecuteMsg<Extension, Empty>;

pub mod entry {
    use super::*;

    #[cfg(not(feature = "library"))]
    use cosmwasm_std::entry_point;
    use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

    #[entry_point]
    pub fn instantiate(
        deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        msg: InstantiateMsg,
    ) -> StdResult<Response> {
        let minter = deps.api.addr_validate(&msg.minter)?;
        Cw721Contract::default().minter.save(deps.storage, &minter)?;
        Ok(Response::default())
    }

    #[entry_point]
    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, ContractError> {
        Cw721Contract::default().execute(deps, env, info, msg)
    }

    #[entry_point]
    pub fn query(deps: Deps, env: Env, msg: QueryMsg<Empty>) -> StdResult<Binary> {
        Cw721Contract::default().query(deps, env, msg)
    }
}
```
