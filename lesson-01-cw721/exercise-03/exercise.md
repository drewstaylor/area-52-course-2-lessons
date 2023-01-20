<!---
Course: 2 
Lesson: 1
Exercise: 3

Title: Not all NFT metadata is created equal

Storyline placeholder:
>
-->

A main feature of [non-fungible](https://en.wikipedia.org/wiki/Non-fungible_token) tokens, is their unique properties, or _metadata_, where no two tokens of the collection are exactly alike. Historically, this metadata has been stored in a [JSON](https://www.w3schools.com/js/js_json_syntax.asp) file uploaded to a storage protocol somewhere, such as [IPFS (the Interplanetary File System)](https://ipfs.tech/). 

A common standard for NFT metadata is the [OpenSea metadata standard](https://docs.opensea.io/docs/metadata-standards), but other format standards do exist.

### On-Chain vs. Off-Chain Metadata in CosmWasm

NFTs were originally created on the Ethereum blockchain where there's some resource limitations regarding memory and data storage. This is why most NFT metadata is stored on IPFS instead of in the memory of the token smart contract.

In CosmWasm, there are fewer resource limitations and it's totally feasible to store metadata in the NFT smart contract itself, but off-chain metadata is still supported in CosmWasm. It's up to the token developer to decide which metadata storage strategy they will use.

### Off-Chain Metadata

Later in the course we'll work directly with on-chain metadata NFTs, but for now it's helpful to understand the older way (off-chain metadata) of doing things.

The below `MintMsg` example comes from a project extending `cw721-base`, but note that `Extension` and `token_uri` are part of `cw721`. The string prefix `ipfs://` is common for signifying [IPFS](https://ipfs.tech/) uploads.

```rs
let token_uri = "ipfs://bafybeigxa4ifta32fjl7yejgr6sddanwcgex5m2xxhatjzpms4iwh5bcvm/ascended.json";

let mint_msg = MintMsg {
    token_id: "token 1".to_string(),
    owner: "archway1f395p0gg67mmfd5zcqvpnp9cxnu0hg6r9hfczq".to_string(), // After mint, this address will own the NFT
    token_uri: token_uri.to_string(), // `token_uri` is for external urls containing metadata in JSON format
    extension: None, // `extension` field is for on-chain metadata!
};
```

# Exercise

Using the provided code, finish the `mint_handler` function for executing a minting transaction. Imagine the minter will be a contract and will send a transaction of type `WasmMsg::Execute` to an NFT contract (which we'll refer to as `some_token`). 

1. Finish `mint_msg` by adding the values for `token_uri` and `extension` for an NFT with off-chain metadata
2. Create a variable called `mint_resp` and explictly enforce the`CosmosMsg` type
3. For the value of `mint_resp` use the `ExecuteMsg` variant of the `WasmMsg` enum from `CosmosMsg`
4. `WasmMsg::Execute` has three member attributes, `contract_addr`, `msg` and `funds`, each can be entered on separate lines
5. The value of `contract_addr` will be the string version of `some_token_address`
6. For `msg` use a reference (`&`) to `mint_msg`, and don't forget to convert it to binary
7. We won't send extra funds with this transaction so set `funds` to an empty vector created using the `vec` macro
8. The `mint_resp` declaration ends with a call to [into](https://doc.rust-lang.org/std/convert/trait.Into.html), written on its own line

If you need a refresher on using `WasmMsg::Execute` you can check out [this exercise](https://area-52.io/starting-with-cosm-wasm/3/imbibe_potion-function-part-4) from an earlier course.

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
        token_uri: , // Add the `token_uri` value here
        extension: , // Add the `extension` value here
    });

    // Write the mint_resp variable here

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