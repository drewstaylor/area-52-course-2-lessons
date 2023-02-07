<!---
Course: 2
Lesson: 3
Exercise: 4

Title: Writing the Instantiate Entry Point Part 2
Filename: lib.rs
-->

> The librarians are watching, and in the `passport-token` blueprint, they've written something rather long about the `minter` and something called "address validation".

CosmWasm dependencies (`Deps`, `DepsMut`) have a handy helper function which allows us to validate if an `Addr` string is a valid Cosmos address. It can be accessed via the [api](https://docs.rs/cosmwasm-std/latest/cosmwasm_std/trait.Api.html) trait of `Deps` and `DepsMut`.

```rs
let unvalidated_addr: Addr = "archway1f395p0gg67mmfd5zcqvpnp9cxnu0hg6r9hfczq";
let validated_addr = deps.api.addr_validate(&unvalidated_addr)?; // addr_validate always takes a reference (`&`)
```

In the last lesson we created `cw721-soulbound`, at that time our `InstantiateMsg` looked like this:

```rs
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    /// Name of the NFT contract
    pub name: String,
    /// Symbol of the NFT contract
    pub symbol: String,

    /// The minter is the only one who can create new NFTs.
    /// This is designed for a base NFT that is controlled by an external program
    /// or contract. You will likely replace this with custom logic in custom NFTs
    pub minter: String,
}
```

Being the only account that can mint NFTs, `minter` has an admin-like privilege in `cw721-base` (and by extension `cw721-soulbound`). Since it can never be updated after deploying, it's probably a good idea if we validate it before saving it to the contract state.

# Exercise

Validate `minter` and save it, along with some other parameters of the token collection, to the contract state.

1. Create a variable called `info` and assign it a [cw721 ContractInfoResponse](https://docs.rs/cw721/0.9.2/cw721/struct.ContractInfoResponse.html).
2. Populate the two `ContractInfoResponse` members with their corresponding values from `InstantiateMsg`, each on their own (indented) line.
3. Save a reference to `info` to the `contract_info` key of the contract's storage. Access the `save` method via the `cw721-soulbound` type alias `Cw721MetadataContract` (e.g. `Cw721MetadataContract::default().contract_info`). The reference to `info` is the second parameter to pass to the `save` function, the first is `deps.storage`. Write everything on one line, but don't forget to capture any errors that might occur.
4. Create a variable called `minter` and assign it a validated minter address using `deps.api.addr_validate`. The argument to pass `addr_validate` comes from `InstantiateMsg`.
5. Finally, save the validated minter address using `minter.save` which can be accessed from `Cw721MetadataContract::default()`. The arguments to pass to `save` are `deps.storage` and a reference to `minter`. Write everything on one line, but don't forget to capture any errors that might occur.

# Starter

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

const CONTRACT_NAME: &str = "crates.io:passport-token";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub mod entry {
    use super::*;

    #[cfg(not(feature = "library"))]
    use cosmwasm_std::entry_point;
    use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, StdResult};

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn instantiate(
        deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        msg: InstantiateMsg,
    ) -> StdResult<Response> {
        set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

        // Create `info` here
        // Save `info` to `contract_info` here

        // Create `minter` here and validate it
        // Save `minter` to storage here

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

const CONTRACT_NAME: &str = "crates.io:passport-token";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub mod entry {
    use super::*;

    #[cfg(not(feature = "library"))]
    use cosmwasm_std::entry_point;
    use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, StdResult};

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
}
```
