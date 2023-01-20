<!---
Course: 2 
Lesson: 1
Exercise: 5

Title: When to use `cw721-base`

Storyline placeholder:
>
-->

In the last two exercises we've been talking about a "token collection contract", which we've been referring to as `some_token`. What is this contract and how does it relate to `cw721` and `cw721-base`?

- `cw721` is a package for enforcing your token adheres to the [cw721](https://github.com/CosmWasm/cw-nfts/blob/main/packages/cw721/README.md) standard
- `cw721-base` is a package that implements the `cw721` standard and provides minting functionality, but where minting is limited to a single admin (or "creator") called the `minter` (an address that must be declared during contract instantiation and can never be updated)
- `some_token` is our example token collection contract. It extends from `cw721-base` and is the NFT smart contract that will be deployed to the blockchain

### When to import cw721-base

A common use case for importing [cw721-base](https://crates.io/crates/cw721-base) into your project is for making the token collection contract.

### Do I really need a token collection contract? Can't I just deploy `cw721-base`?

You could just deploy `cw721-base`. In fact, the [unit tests](https://github.com/CosmWasm/cw-nfts/blob/main/contracts/cw721-base/src/contract_tests.rs) of `cw721-base` are doing just that. However, it's preferable to separate `cw721-base` (a template for all NFT projects) from the logic and code of some specific NFT project.

1. If you deploy `cw721-base` as your collection contract, on-chain metadata won't work (but off-chain metadata will). This happens because [lib.rs](https://github.com/CosmWasm/cw-nfts/blob/main/contracts/cw721-base/src/lib.rs#L14-L15) of `cw721-base` exports a default `extension` with a value of `None`

```rs
pub type Extension = Option<Empty>;
```

2. Adding custom logic means changing `cw721-base` (a public library) so it's not really `cw721-base` anymore.

# Exercise

Now we'll create the collection contract for `some_token`. This is the only code needed for creating the collection contract (along with the `Cargo.toml` which imports the dependencies). Your task is to finish writing the entry points.

1. Save the `minter` address in `instantiate` so that once the contract is deployed, tokens can be minted by the admin address. To save `minter` you'll need the `save` function of `Cw721MetadataContract::default().minter` and pass two arguments to it, which are `deps.storage` and a reference to `minter` (a variable defined and validated just above)
2. Finish the `execute` entry point by calling the `execute` function of `Cw721MetadataContract::default()`. The arguments to pass to it are the same as what's being sent to `some_token`'s `execute`
3. Finish the `query` entry point with a call to `query` from `Cw721MetadataContract::default()`. The arguments to pass it are the same as those being sent to the `query` entry point of `some_token`

# Starter

```rs
use cosmwasm_std::Empty;
pub use cw721_base::{
    ContractError, InstantiateMsg, MintMsg, 
    MinterResponse, QueryMsg
};
pub type Extension = Option<Metadata>;
pub type Cw721MetadataContract<'a> = cw721_base::Cw721Contract<'a, Extension, Empty, Empty, Empty>;
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
        // Save the minter address here
        Ok(Response::default())
    }

    #[entry_point]
    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, ContractError> {
        // Finish the `execute` entry point here
    }

    #[entry_point]
    pub fn query(deps: Deps, env: Env, msg: QueryMsg<Empty>) -> StdResult<Binary> {
        // Finish the `query` entry point here
    }
}
```

# Answer

```rs
use cosmwasm_std::Empty;
pub use cw721_base::{
    ContractError, InstantiateMsg, MintMsg, 
    MinterResponse, QueryMsg
};
pub type Extension = Option<Metadata>;
pub type Cw721MetadataContract<'a> = cw721_base::Cw721Contract<'a, Extension, Empty, Empty, Empty>;
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
        Cw721MetadataContract::default().minter.save(deps.storage, &minter)?;
        Ok(Response::default())
    }

    #[entry_point]
    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, ContractError> {
        Cw721MetadataContract::default().execute(deps, env, info, msg)
    }

    #[entry_point]
    pub fn query(deps: Deps, env: Env, msg: QueryMsg<Empty>) -> StdResult<Binary> {
        Cw721MetadataContract::default().query(deps, env, msg)
    }
}
```