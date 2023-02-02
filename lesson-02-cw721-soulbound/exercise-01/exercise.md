<!---
Course: 2 
Lesson: 2
Exercise: 1

Title: Contracts and Libraries
Filename: lib.rs
-->

> If Section 31 didn't like what you were doing, they could have just vapourized the teleporter (not to mention your entire DNA). So why'd you end up in their cyber jail? It's got to have something to do with this missing `passport-token`.

It occurs to you that you may need to build an NFT just get out of here!

When it comes to making NFT collections with custom logic, there are a few strategies that could be followed.

For example:

1. Add customizations in the token collection contract
2. Bootstrap a version of `cw721-base` locally within our project and modify it
3. Import modified versions of `cw721-base` as package dependencies

We'll be reviewing and comparing these approaches, but before we do that lets talk a bit about [Rust's dependency model](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html).

### Dependencies in Rust

Before we go about changing core NFT dependencies, it will be helpful to review how dependencies work. 

Dependencies, or [packages](https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html), are bundles of code that can be imported into your project inside the `Cargo.toml` file in the root folder of a Rust project. 

If the package is not published to [crates.io](https://crates.io/), we declare it as a dependency like:

```yaml
[dependencies]
cosmwasm-std = "~1.0.0-beta" # E.g. {package-name} = "{package_version}"
```

If the package hasn't been published to [crates.io](https://crates.io/), it can be declared as a dependency if you instruct the compiler where to look for it. We can do that by adding a `path` parameter to a folder where that package's code exists. If no package, of the specified name and version, exists at that path, your Rust project won't compile.

```yaml
[dependencies]
example-local-package = { path = "../example-local-package", version = "0.1.0" }
```

### Compilation in Rust

The project you're working on can also be a package. If your code is public, other people could include it in the dependency tree of their projects.

Rust projects can either be compiled as binaries or libraries. Binaries refer to executable projects with a `main()` method. Libraries are components that can be reused in other projects. Unlike a binary program, a library does not have `main()` method as its entry point.

Below are two different project structures, where project `a` represents the convention for programs (`main.rs`), and project `b` the convention for libraries (`lib.rs`). There's more to it than that, but we'll learn as we go.

```yaml
├── a
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── b
    ├── Cargo.toml
    └── src
        └── lib.rs
```

### Compilation in CosmWasm

CosmWasm projects are always compiled as libraries. This allows for multiple entry points into the program (`instantiate`, `query`, `execute`, `reply`, etc.), none of which are `main()`. That doesn't mean all CosmWasm contracts are intended to be inherited as packages by other developers. 

If you do want to support other developers using your contracts, it's important to publicly export all types that could be useful for projects inheriting your package. 

```rs
// Publicly exported
pub use cw721_base::{
    ContractError, InstantiateMsg, MintMsg, 
    MinterResponse, QueryMsg
};

// Not publicly exported
use cw721::ContractInfoResponse;
```

In the above example [cw721](https://crates.io/crates/cw721) isn't publicly exported. Other developers can still import it into their projects since it's a publicly available [crate](https://doc.rust-lang.org/rust-by-example/crates.html) hosted on [crates.io](https://crates.io/).

# Exercise
There's been a complaint lodged against the creators of `some_token`. Other developers trying to use it in marketplace contracts are struggling to execute transactions. Do you know why?

Review and adjust the imports and type aliases. Ensure anything custom has public visibility, and think carefully about which items _should_ be public and which _don't_ need to be.

# Starter
```rs
pub use cosmwasm_std::Empty;
pub use cw721_base::{
    ContractError, InstantiateMsg, MintMsg, 
    MinterResponse, QueryMsg
};
type Extension = Option<Empty>;
type Cw721Contract<'a> = cw721_base::Cw721Contract<'a, Extension, Empty, Empty, Empty>;
type ExecuteMsg = cw721_base::ExecuteMsg<Extension, Empty>;

pub mod entry {
    use super::*;

    #[cfg(not(feature = "library"))]
    pub use cosmwasm_std::entry_point;
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