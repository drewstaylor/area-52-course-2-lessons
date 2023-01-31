# Hint

It's important we don't get confused between our `MintMsg` messages, of which there are two.

1. `MintMsg` of the Portal contract, is used by the `mint_passport` entry point function

```rs
// portal/src/msg.rs
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MintMsg {
    pub name: String,
    pub description: String,
    pub image: String,
    pub dna: String,
    pub species: String,
    pub sapience_level: SapienceScale,
    pub identity: Addr,
}
```

2. `MintMsg<T>` of the `cw721-soulbound` package, is used by the `mint` entry point function in the `passport-token` collection contract

```rs
// nft/cw721-soulbound/src/msg.rs
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MintMsg<T> {
    /// Unique ID of the NFT
    pub token_id: String,
    /// The owner of the newly minter NFT
    pub owner: String,
    /// Universal resource identifier for this NFT
    /// Should point to a JSON file that conforms to the ERC721
    /// Metadata JSON Schema
    pub token_uri: Option<String>,
    /// Any custom extension used by this contract
    pub extension: T,
}
```
