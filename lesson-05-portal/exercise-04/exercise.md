<!---
Course: 2
Lesson: 5
Exercise: 4

Title: Modifying the JumpRingTravel Execute Arguments
Filename: contract.rs
-->

> Ever since you became a cyborg you've been feeling super weird, and you've been having the strangest dreams. Have you _really_ just implemented your own decentralized identity system using `cw721` NFTs? The Jump Ring looks so snazzy now, and booting it up in test mode you see your NFT avatar and `passport-token` appearing on the loading screen. You realize that next time you work on the Jump Ring you'll get to test traveling through the Ring! Oh, but before all that, the Portal's compiler is still throwing an error. Fulfill this final task and the NFT identity integration will be completed.

The `initiate_jump_ring_travel` entry point function is already imported in `contract.rs`, but since its message type has been modified in `msg.rs`, the Portal project doesn't [compile](https://doc.rust-lang.org/cargo/commands/cargo-build.html) yet.

If we try to compile the project as is, the [rustc](https://doc.rust-lang.org/rustc/what-is-rustc.html) compiler throws two errors.


Error #1

```rs
ExecuteMsg::JumpRingTravel { to } => initiate_jump_ring_travel(to, deps, env, info)
                            ^^^^ missing field `traveler`
```

Error #2

```rs
ExecuteMsg::JumpRingTravel { to } => initiate_jump_ring_travel(to, deps, env, info),
                                                                ^^^^^^^^^^^^^^^^^ ---- an argument of type `Addr` is missing
```

Rust's compiler sure is helpful, even if it's a bit opinionated. Like a code whisperer that doesn't have [ChatGPT](https://en.wikipedia.org/wiki/ChatGPT)'s pitfalls 😏

### Rustc

[rustc](https://doc.rust-lang.org/rustc/what-is-rustc.html) is the compiler for Rust provided by [Cargo](https://doc.rust-lang.org/cargo/getting-started/index.html) (and the Rust project itself).

Normally, Rust programmers don't invoke the [rustc](https://doc.rust-lang.org/rustc/what-is-rustc.html) directly, but do it through [Cargo](https://doc.rust-lang.org/cargo/getting-started/index.html) with dependencies implemented and managed by the `Cargo.toml` file located at the root of the project.

Rust programs can be built using the command `cargo build`. The following are bash commands that can be executed from any folder inside your Rust project.

Building an unoptimized Rust binary:

```bash
cargo build
```

Building an optimized Rust binary:

```bash
cargo build --release
```

#### (_Note: that building a Rust binary is not the same as building the [wasm](https://webassembly.org/) that will be uploaded to and instantiated on the blockchain._)

Running a project's unit tests:

```bash
cargo test
```

Running a project's unit tests with debugging output enabled (if you need to use `println!`, `dbg!`, etc.):

```bash
cargo test -- --nocapture
```

# Exercise 

1. Add the missing message parameter to `ExecuteMsg::JumpRingTravel`. If you're confused about what do, have a look at `ExecuteMsg::JumpRingTravel` in `msg.rs` to determine what field is missing.

# Starter

```rs
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::query_fns::{jump_ring_check, minimum_sapience};
use crate::execute_fns::{
    initiate_jump_ring_travel, mint_passport, set_minimum_sapience, set_passport_contract, 
    set_planet_name, set_potion_contract, set_sapient_names,
};
use crate::state::{Config, CONFIG};

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::JumpRingPreCheck { traveler } => jump_ring_check(traveler),
        QueryMsg::MinimumSapience {} => minimum_sapience(deps),
    }
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SetPlanetName { to } => set_planet_name(to, deps, info),
        ExecuteMsg::SetSapientNames { to } => set_sapient_names(to, deps, info),
        ExecuteMsg::SetMinimumSapience { to } => set_minimum_sapience(to, deps, info),
        ExecuteMsg::SetPassportContract { contract } => set_passport_contract(contract, deps, info),
        ExecuteMsg::SetPotionContract { contract } => set_potion_contract(contract, deps, info),
        ExecuteMsg::MintPassport { msg } => mint_passport(msg, deps, env, info),
        ExecuteMsg::JumpRingTravel { to } => initiate_jump_ring_travel(to, deps, env, info), // Modify `ExecuteMsg::JumpRingTravel` here
    }
}

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let config = Config {
        owner: info.sender,
        planet_name: msg.planet_name,
        planet_sapients: msg.planet_sapients,
        minimum_sapience: msg.minimum_sapience,
        passport_contract: msg.passport_contract,
        potion_contract: msg.potion_contract,
    };
    CONFIG.save(deps.storage, &config)?;
    Ok(Response::new()
        .add_attribute("owner", config.owner)
        .add_attribute("minimum_sapience", config.minimum_sapience.as_str()))
}
```

# Answer

```rs
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::query_fns::{jump_ring_check, minimum_sapience};
use crate::execute_fns::{
    initiate_jump_ring_travel, mint_passport, set_minimum_sapience, set_passport_contract, 
    set_planet_name, set_potion_contract, set_sapient_names,
};
use crate::state::{Config, CONFIG};

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::JumpRingPreCheck { traveler } => jump_ring_check(traveler),
        QueryMsg::MinimumSapience {} => minimum_sapience(deps),
    }
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SetPlanetName { to } => set_planet_name(to, deps, info),
        ExecuteMsg::SetSapientNames { to } => set_sapient_names(to, deps, info),
        ExecuteMsg::SetMinimumSapience { to } => set_minimum_sapience(to, deps, info),
        ExecuteMsg::SetPassportContract { contract } => set_passport_contract(contract, deps, info),
        ExecuteMsg::SetPotionContract { contract } => set_potion_contract(contract, deps, info),
        ExecuteMsg::MintPassport { msg } => mint_passport(msg, deps, env, info),
        ExecuteMsg::JumpRingTravel { to, traveler } => initiate_jump_ring_travel(to, traveler, deps, env, info),
    }
}

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let config = Config {
        owner: info.sender,
        planet_name: msg.planet_name,
        planet_sapients: msg.planet_sapients,
        minimum_sapience: msg.minimum_sapience,
        passport_contract: msg.passport_contract,
        potion_contract: msg.potion_contract,
    };
    CONFIG.save(deps.storage, &config)?;
    Ok(Response::new()
        .add_attribute("owner", config.owner)
        .add_attribute("minimum_sapience", config.minimum_sapience.as_str()))
}
```
