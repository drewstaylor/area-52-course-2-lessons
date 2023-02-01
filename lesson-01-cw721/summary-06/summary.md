<!---
Course: 2 
Lesson: 1
Exercise: 6 (Summary)

Filename: lib.rs
-->

# Overview

In this lesson we looked at the [cw721](https://github.com/CosmWasm/cw-nfts/blob/main/packages/cw721/README.md) standard for CosmWasm NFTs, and its relationship to [cw721-base](https://crates.io/crates/cw721-base) which is a basic implementation of a `cw721` NFT contract.

In general, we learned about:

- The entry points and functions provided by `cw721` as implemented in `cw721-base`
- How to mint NFTs with `cw721-base`
- `cw721` with off-chain metadata
- Creating a token collection contract

### **SOURCE CODE**
- <ExternalLink href="https://github.com/phi-labs-ltd/area-52-courses/">Building with NFTs repo</ExternalLink>

<!--- NEXT UP: -->
# Exercise

Modifying `cw721-base` with custom logic

# Starter

```rs
use cosmwasm_std::{
    CosmosMsg, DepsMut, Env, MessageInfo, to_binary, 
    Response, WasmMsg,
};
use some_token::{
    ExecuteMsg as Cw721ExecuteMsg, MintMsg as Cw721MintMsg,
};
use crate::error::ContractError;
use crate::state::CONFIG;
use crate::msg::MintMsg;

pub fn mint_handler(
    msg: MintMsg,
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let some_token_address = config.some_token_address;
    
    let token_uri = "ipfs://bafybeigxa4ifta32fjl7yejgr6sddanwcgex5m2xxhatjzpms4iwh5bcvm/ascended.json";
    let owner = "archway1f395p0gg67mmfd5zcqvpnp9cxnu0hg6r9hfczq";

    let mint_msg: some_token::ExecuteMsg = Cw721ExecuteMsg::Mint(Cw721MintMsg {
        token_id: "token 1".to_string(),
        owner: owner.to_string(),
        token_uri: token_uri.to_string(),
        extension: None,
    });

    let mint_resp: CosmosMsg = WasmMsg::Execute {
        contract_addr: some_token_address.to_string(),
        msg: to_binary(&mint_msg)?,
        funds: vec![],
    }
    .into();

    let messages = vec![mint_resp];
    Ok(Response::new().add_messages(messages))
}
```

# Answer

```rs
use cosmwasm_std::{
    CosmosMsg, DepsMut, Env, MessageInfo, to_binary, 
    Response, WasmMsg,
};
use some_token::{
    ExecuteMsg as Cw721ExecuteMsg, MintMsg as Cw721MintMsg,
};
use crate::error::ContractError;
use crate::state::CONFIG;
use crate::msg::MintMsg;

pub fn mint_handler(
    msg: MintMsg,
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let some_token_address = config.some_token_address;
    
    let token_uri = "ipfs://bafybeigxa4ifta32fjl7yejgr6sddanwcgex5m2xxhatjzpms4iwh5bcvm/ascended.json";
    let owner = "archway1f395p0gg67mmfd5zcqvpnp9cxnu0hg6r9hfczq";

    let mint_msg: some_token::ExecuteMsg = Cw721ExecuteMsg::Mint(Cw721MintMsg {
        token_id: "token 1".to_string(),
        owner: owner.to_string(),
        token_uri: token_uri.to_string(),
        extension: None,
    });

    let mint_resp: CosmosMsg = WasmMsg::Execute {
        contract_addr: some_token_address.to_string(),
        msg: to_binary(&mint_msg)?,
        funds: vec![],
    }
    .into();

    let messages = vec![mint_resp];
    Ok(Response::new().add_messages(messages))
}
```
