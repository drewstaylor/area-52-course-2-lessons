<!---
Course: 2
Lesson: 3
Exercise: 2

Title: Cw721 with On-chain Metadata
Filename: lib.rs

Storyline placeholder:
>
-->

### On-Chain vs Off-Chain NFT Metadata

So far we've only seen off-chain metadata in action. For `passport-token` we'll use the very awesome on-chain metadata instead. 

Rather than uploading [JSON](https://developer.mozilla.org/en-US/docs/Learn/JavaScript/Objects/JSON) files to [IPFS](https://ipfs.tech/), unique NFT attributes will be stored in the contract itself. Translating between contract storage and [JSON](https://developer.mozilla.org/en-US/docs/Learn/JavaScript/Objects/JSON) will be handled by [Serde](https://serde.rs/), so anyone querying token metadata still gets a response in [JSON](https://developer.mozilla.org/en-US/docs/Learn/JavaScript/Objects/JSON) format.

### Serde Package in Rust

[Serde](https://serde.rs/) is a framework for serializing and deserializing Rust data structures efficiently and generically. It consists of data structures that know how to serialize and deserialize themselves along with data formats that know how to serialize and deserialize other things. [Serde](https://serde.rs/) allows any supported data structure to be serialized and deserialized using any supported data format.

### MintMsg (again)

Previously, we wrote code like

```rs
let mint_msg = MintMsg {
    token_id: "token 1".to_string(),
    owner: "archway1f395p0gg67mmfd5zcqvpnp9cxnu0hg6r9hfczq".to_string(),
    token_uri: "ipfs://bafybeigxa4ifta32fjl7yejgr6sddanwcgex5m2xxhatjzpms4iwh5bcvm/ascended.json".to_string(),
    extension: None, // `extension` field is for on-chain metadata!
};
```

In the on-chain version, it's `token_uri` that will be `None` and `extension` that will have data. 

In Rust `std::option` is a built in type that represents optional data. Every `Option` is either `Some` (and contains a value), or `None` (does not contain a value). `Option` types are common in Rust code and have a number of uses, read more about them [here](https://doc.rust-lang.org/std/option/).

```rs
// Having at least these three member fields ensures 
// interoperability with most NFT dapps and marketplaces
let metadata_extension = Some(Metadata {
    name: Option<String>,
    description: Option<String>,
    image: Option<String>,
});

let mint_msg = MintMsg {
    token_id: "token 1".to_string(),
    owner: "archway1f395p0gg67mmfd5zcqvpnp9cxnu0hg6r9hfczq".to_string(),
    token_uri: None, // `token_uri` field is for off-chain metadata
    extension: metadata_extension,
};
```

Using `Option` for NFT metadata fields is considered a best practice. That's so developers can have flexibility within their NFT collections. For example, you might want to make certain attributes rare.

# Exercise

1. Create a public structure called `Metadata` to create a metadata schema for the NFTs. We can use it to ensure each NFT conforms to the schema
2. `Metadata` will have 9 public members, `name`, `description`, `image`, `dna`, `species`, `sapience_level`, `issuer`, `origin` and `identity`
3. Most of the members will have the `String` type; but, `sapience_level`'s type will be `SapienceScale`, and both `issuer` and `identity` are Cosmos addresses (`Addr` type)
4. Each of the members can be indented and written on a separate line

# Starter

```rs
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Empty, Addr};

pub use cw721::Expiration;
pub use universe::species::SapienceScale;

pub use cw721::{ContractInfoResponse};

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)] // Derived from `serde` and `schemars`
// Add the `Metadata` struct here

pub type Extension = Option<Metadata>;
pub type Cw721MetadataContract<'a> = cw721_soulbound::Cw721Contract<'a, Extension, Empty, Empty, Empty>;
```

# Answer

```rs
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Empty, Addr};

pub use cw721::Expiration;
pub use universe::species::SapienceScale;

pub use cw721::{ContractInfoResponse};

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)] // Derived from `serde` and `schemars`
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
```
