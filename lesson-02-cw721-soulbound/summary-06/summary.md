<!---
Course: 2 
Lesson: 2
Exercise: 6 (Summary)

Filename: Cargo.toml
-->

# Overview

In this lesson we learned how to customize the behavior of CosmWasm NFTs in a modular fashion.

In general we learned about:

- Using dependencies hosted on [Crates.io](https://crates.io/)
- Using local dependencies 
- Compiling Rust programs as libraries vs binaries

From CosmWasm we touched on:

- Modifying `cw721-base` with custom logic
- The composition of `cw721-non-transferable`

But there's one more thing...
**Don't forget to update your package name in `Cargo.toml`!**

### **SOURCE CODE**
- <ExternalLink href="https://github.com/phi-labs-ltd/area-52-courses/">Building with NFTs repo</ExternalLink>

<!--- NEXT UP: -->
# Exercise

Creating the token collection contract for `cw721-soulbound`

# Starter

```yaml
name = "cw721-soulbound"
version = "0.13.4"
edition = "2018"
description = "Area-52 implementation of soulbound cw721 NFTs"
authors = ["Richard Mervin Bissell Jr. <rbissel@cia.gov>"]
license = "NOFORN"

exclude = [
  # Those files are rust-optimizer artifacts. You might want to commit them for convenience but they should not be part of the source code publication.
  "artifacts/*",
]

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
[lib]
crate-type = ["cdylib", "rlib"]

[features]
# for more explicit tests, cargo test --features=backtraces
backtraces = ["cosmwasm-std/backtraces"]
# use library feature to disable all instantiate/execute/query exports
library = []

[dependencies]
cw-utils = "0.13.4"
cw2 = "0.13.4"
cw721 = { path = "./packages/cw721", version = "0.13.4" }
cw-storage-plus = "0.13.4"
cosmwasm-std = { version = "1.0.0" }
schemars = "0.8.10"
serde = { version = "1.0.140", default-features = false, features = ["derive"] }
thiserror = { version = "1.0.31" }

[dev-dependencies]
cosmwasm-schema = { version = "1.0.0" }
```

# Answer

```yaml
name = "cw721-soulbound"
version = "0.13.4"
edition = "2018"
description = "Area-52 implementation of soulbound cw721 NFTs"
authors = ["Richard Mervin Bissell Jr. <rbissel@cia.gov>"]
license = "NOFORN"

exclude = [
  # Those files are rust-optimizer artifacts. You might want to commit them for convenience but they should not be part of the source code publication.
  "artifacts/*",
]

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
[lib]
crate-type = ["cdylib", "rlib"]

[features]
# for more explicit tests, cargo test --features=backtraces
backtraces = ["cosmwasm-std/backtraces"]
# use library feature to disable all instantiate/execute/query exports
library = []

[dependencies]
cw-utils = "0.13.4"
cw2 = "0.13.4"
cw721 = { path = "./packages/cw721", version = "0.13.4" }
cw-storage-plus = "0.13.4"
cosmwasm-std = { version = "1.0.0" }
schemars = "0.8.10"
serde = { version = "1.0.140", default-features = false, features = ["derive"] }
thiserror = { version = "1.0.31" }

[dev-dependencies]
cosmwasm-schema = { version = "1.0.0" }
```
