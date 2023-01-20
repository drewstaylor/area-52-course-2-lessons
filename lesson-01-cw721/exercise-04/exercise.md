<!---
Course: 2 
Lesson: 1
Exercise: 4

Title: When to use `cw721`

Storyline placeholder:
>
-->

Technically speaking, `cw721` refers to the NFT standard (not the NFTs themselves). This standard is also a public Rust library that can be imported into a CosmWasm project.

Let's think about what the use cases are for importing the `cw721` package into your project.

### When to import cw721

- Querying an NFT collection from inside a contract and enforcing the correct response type
- Making a new type of NFT with custom logic (e.g. "soulbound" NFTs)

**Example import:**

```rs
use cw721::{NftInfoResponse, TokensResponse};   // Imports response types for querying NFT 
                                                // metadata and getting token ids
```

### When not to import cw721 into your project

Most NFT projects _will_ import and use `cw721` for enforcing the correct types of query responses, but more often what you need should be imported from the token collection contract.

Below are common use cases where you likely _won't_ need to import `cw721`.

- Minting tokens (minting is not implemented in `cw721`, import the minting message from the token collection contract)
- Creating the message to query an NFT collection (whenever possible, use the `QueryMsg` exported by the token collection contract)
- Creating the execute message for a transaction to an NFT contract (whenever possible, use the `ExecuteMsg` exported by the token collection contract)

# Exercise

Now we'll be further extending the code we began writing in the previous exercise. We've provided you with a working `WasmQuery` and `QueryMsg`, but you'll need to write the response for it.

1. Create a variable called `query_response` that enforces the `TokensResponse` type imported from `cw721`
2. For its value you'll need to use the `query` module from `DepsMut`, which is exposed via the [querier](https://docs.rs/cosmwasm-std/latest/cosmwasm_std/struct.DepsMut.html#structfield.querier) field of the `DepsMut` struct
3. Pass `querier.query` a reference (`&`) to `query_req` as its function argument, and don't forget to capture any errors that could occur by using the `?` operator

# Starter

```rs
use cosmwasm_std::{
    CosmosMsg, DepsMut, Env, MessageInfo, QueryRequest, 
    to_binary, Response, WasmMsg, WasmQuery
};
use some_token::{
    ExecuteMsg as Cw721ExecuteMsg, MintMsg as Cw721MintMsg,
    QueryMsg as Cw721QueryMsg,
};
use cw721::TokensResponse;
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
    let owner = owner: "archway1f395p0gg67mmfd5zcqvpnp9cxnu0hg6r9hfczq";
    
    let query_msg: some_token::QueryMsg<Extension> = Cw721QueryMsg::Tokens {
        owner: owner.clone().to_string(),
        start_after: None,
        limit: None,
    };

    let query_req = QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: some_token_address.clone().to_string(),
        msg: to_binary(&query_msg).unwrap(),
    });
    
    // Create and handle `query_resp` here

    // Our `mint_handler` fails with an error if the user has already minted
    if !query_resp.tokens.is_empty() {
        return Err(ContractError::Unauthorized {});
    }

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
    CosmosMsg, DepsMut, Env, MessageInfo, QueryRequest, 
    to_binary, Response, WasmMsg, WasmQuery
};
use some_token::{
    ExecuteMsg as Cw721ExecuteMsg, MintMsg as Cw721MintMsg,
    QueryMsg as Cw721QueryMsg,
};
use cw721::TokensResponse;
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
    let owner = owner: "archway1f395p0gg67mmfd5zcqvpnp9cxnu0hg6r9hfczq";
    
    let query_msg: some_token::QueryMsg<Extension> = Cw721QueryMsg::Tokens {
        owner: owner.clone().to_string(),
        start_after: None,
        limit: None,
    };

    let query_req = QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: some_token_address.clone().to_string(),
        msg: to_binary(&query_msg).unwrap(),
    });
    
    let query_resp: TokensResponse = deps.querier.query(&query_req)?;

    // Our `mint_handler` fails with an error if the user has already minted
    if !query_resp.tokens.is_empty() {
        return Err(ContractError::Unauthorized {});
    }

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