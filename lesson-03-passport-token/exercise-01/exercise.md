<!---
Course: 2
Lesson: 3
Exercise: 1

Title: 
Filename: ../Cargo.toml

Storyline placeholder:
>
-->

In this lesson we'll be making the token collection contract, but let's take a moment to review what we've done and why.

### Custom NFT Logic

Changing the behavior of `cw721` is undesirable (not to mention dangerous), since it's the rule book functionality CosmWasm NFTs _must_ possess. It was a neccessary evil in our case, because `cw721` was developed with asset transfers in mind. Most times when you need to implement custom behavior, it can be achieved by adding the logic in the token collection, or by modifying `cw721-base` with a new implementation and then changing the package name (e.g. `cw721-my-custom-nft`). Notice when we tried to do it that way, the NFT contract was left with dangling entry points that were always failing, that would charge the users gas fees but were effectively useless.

```rs
// An entry point function that can never succeed is never a good idea
fn transfer_nft(
    &self,
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _recipient: String,
    _token_id: String,
) -> Result<Response<C>, ContractError> {
    return Err(ContractError::Unauthorized {});
}
```

### Modular Package Design

Additionally, we learned about creating and inheriting our own packages, and how it helps keeps our code clean and modular. Separating code this way lets us compile and work on portions of our project independently. 

For packages that can either be deployed as a smart contract or inherited as a dependency, when we define them as a in `Cargo.toml` we must explicitly enable library [features](https://doc.rust-lang.org/cargo/reference/features.html), using the syntax `features = ["library"]`.

```yaml
[dependencies]
some-dependency = { path = "some-path/some-dependency", version = "0.1.0", features = ["library"] }
```

# Exercise

In the previous lesson we created a new package called `cw721-soulbound`, but before it can be used in the token collection (a new package we'll call `passport-token`), Rust library [features](https://doc.rust-lang.org/cargo/reference/features.html) need to be enabled.

1. Locate `cw721-soulbound` in the dependency list and enable the `library` feature.

# Starter

```yaml
[package]
name = "passport-token"
version = "0.1.0"
authors = ["SECTION 31 <section31@area-52.io>"]
edition = "2018"

exclude = [
  # Those files are rust-optimizer artifacts. You might want to commit them for convenience but they should not be part of the source code publication.
  "artifacts/*",
]

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[lib]
crate-type = ["cdylib", "rlib"]

[profile.release]
opt-level = 3
debug = false
rpath = false
lto = true
debug-assertions = false
codegen-units = 1
panic = 'abort'
incremental = false
overflow-checks = true

[features]
# for more explicit tests, cargo test --features=backtraces
backtraces = ["cosmwasm-std/backtraces"]
# use library feature to disable all instantiate/execute/query exports
library = []

[package.metadata.scripts]
optimize = """docker run --rm -v "$(pwd)":/code \
  -e CARGO_TERM_COLOR=always \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.12.5
"""

[dependencies]
cosmwasm-std = "~1.0.0-beta"
cw2 = "0.11"
cw721 = { path = "../cw721-soulbound/packages/cw721", version = "0.13.4" }
cw721-soulbound = { path = "../cw721-soulbound", version = "0.13.4" }
universe = { path = "../../universe" }
schemars = "0.8"
serde = { version = "1.0", default-features = false, features = ["derive"] }
thiserror = "1.0.23"

[dev-dependencies]
cosmwasm-schema = "~1.0.0-beta"
```

# Answer

```yaml
[package]
name = "passport-token"
version = "0.1.0"
authors = ["SECTION 31 <section31@area-52.io>"]
edition = "2018"

exclude = [
  # Those files are rust-optimizer artifacts. You might want to commit them for convenience but they should not be part of the source code publication.
  "artifacts/*",
]

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[lib]
crate-type = ["cdylib", "rlib"]

[profile.release]
opt-level = 3
debug = false
rpath = false
lto = true
debug-assertions = false
codegen-units = 1
panic = 'abort'
incremental = false
overflow-checks = true

[features]
# for more explicit tests, cargo test --features=backtraces
backtraces = ["cosmwasm-std/backtraces"]
# use library feature to disable all instantiate/execute/query exports
library = []

[package.metadata.scripts]
optimize = """docker run --rm -v "$(pwd)":/code \
  -e CARGO_TERM_COLOR=always \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.12.5
"""

[dependencies]
cosmwasm-std = "~1.0.0-beta"
cw2 = "0.11"
cw721 = { path = "../cw721-soulbound/packages/cw721", version = "0.13.4" }
cw721-soulbound = { path = "../cw721-soulbound", version = "0.13.4", features = ["library"] }
universe = { path = "../../universe" }
schemars = "0.8"
serde = { version = "1.0", default-features = false, features = ["derive"] }
thiserror = "1.0.23"

[dev-dependencies]
cosmwasm-schema = "~1.0.0-beta"
```
