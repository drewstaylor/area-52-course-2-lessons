<!---
Course: 2
Lesson: 3
Exercise: 9 (Summary)

Filename: integration_tests.rs

Storyline placeholder:
>
-->

# Overview

In this lesson we developed the token contract for an NFT collection that implements the `cw721-soulbound` package from [Lesson 2]().

In general we learned about:

- Declaring custom packages as dependencies
- Enabling libarary features in `Cargo.toml`
- Serializing and deserializing NFT metadata with [Serde](https://serde.rs/)
- Using [type aliases](https://doc.rust-lang.org/reference/items/type-aliases.html)
- Writing unit tests for smart contracts

From the CosmWasm libraries we touched on:

- `cw2` and contract migrations
- Validating Cosmos `Addr` types with `addr_validate` from `Deps.api`
- The `Reply` type used in the `reply` entry point
- Using `mock_dependencies`, `mock_env`, `mock_info` from `cosmwasm_std::testing` to simulate blockchain queries and transactions

### **SOURCE CODE**
Check out the full code we've covered throughout this lesson. Fork it, tweak it, and make it your own :)
- <ExternalLink href="https://github.com/phi-labs-ltd/area-52-courses/">Building with NFTs repo</ExternalLink>

<!--- NEXT UP: -->
# Exercise

Creating the token collection contract for `cw721-soulbound`!

# Starter

```rs
#![cfg(test)]

mod tests {
    use crate::{Cw721MetadataContract, ExecuteMsg, Metadata};

    use cw721::Cw721Query;
    use cw721_soulbound::{InstantiateMsg, MintMsg};
    use universe::species::{Species, SapienceScale};

    use cosmwasm_std::Addr;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};

    const MINTER: &str = "jumpring";    // Each JumpRing mints passports and handles passport validation;
                                        // (Like airport security and an intergalactic embassy combined)

    #[test]
    fn use_metadata_extension() {
        let mut deps = mock_dependencies();
        let contract = Cw721MetadataContract::default();
        let info = mock_info(MINTER, &[]);

        let instantiate_msg = InstantiateMsg {
            name: "passport token".to_string(),
            symbol: "PASS".to_string(),
            minter: MINTER.to_string(),
        };

        contract.instantiate(deps.as_mut(), mock_env(), info.clone(), instantiate_msg).unwrap();

        let species = Species {
            name: "Cyborg type 3 (Human)".to_string(),
            sapience_level: SapienceScale::High,
        };

        let metadata_extension = Some(Metadata {
            name: Some("Traveler Name".into()),
            description: Some("Ever since you became a Cyborg, you've been feeling pretty weird...".into()),
            image: Some("ipfs://QmZdPdZzZum2jQ7jg1ekfeE3LSz1avAaa42G6mfimw9TEn".into()),
            dna: Some("Example DNA String".into()),
            species: Some(species.name),
            sapience_level: Some(species.sapience_level),
            issuer: Some(Addr::unchecked("archway1yvnw8xj5elngcq95e2n2p8f80zl7shfwyxk88858pl6cgzveeqtqy7xtf7")),
            origin: Some("earth".into()),
            identity: Some(Addr::unchecked("archway1f395p0gg67mmfd5zcqvpnp9cxnu0hg6r9hfczq")),
        });

        let token_id = "1".to_string();
        let mint_msg = MintMsg {
            token_id: token_id,
            owner: MINTER.to_string(),
            token_uri: None,
            extension: metadata_extension,
        };
        let execute_msg = ExecuteMsg::Mint(mint_msg.clone());
        contract.execute(deps.as_mut(), mock_env(), info, execute_msg).unwrap();

        let res = contract.nft_info(deps.as_ref(), token_id.into()).unwrap();

        assert_eq!(res.token_uri, mint_msg.token_uri); // off-chain metadata should be `None`
        assert_eq!(res.extension, mint_msg.extension); // on-chain metadata should be equal to `metadata_extension`
    }
}
```

# Answer

```rs
#![cfg(test)]

mod tests {
    use crate::{Cw721MetadataContract, ExecuteMsg, Metadata};

    use cw721::Cw721Query;
    use cw721_soulbound::{InstantiateMsg, MintMsg};
    use universe::species::{Species, SapienceScale};

    use cosmwasm_std::Addr;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};

    const MINTER: &str = "jumpring";    // Each JumpRing mints passports and handles passport validation;
                                        // (Like airport security and an intergalactic embassy combined)

    #[test]
    fn use_metadata_extension() {
        let mut deps = mock_dependencies();
        let contract = Cw721MetadataContract::default();
        let info = mock_info(MINTER, &[]);

        let instantiate_msg = InstantiateMsg {
            name: "passport token".to_string(),
            symbol: "PASS".to_string(),
            minter: MINTER.to_string(),
        };

        contract.instantiate(deps.as_mut(), mock_env(), info.clone(), instantiate_msg).unwrap();

        let species = Species {
            name: "Cyborg type 3 (Human)".to_string(),
            sapience_level: SapienceScale::High,
        };

        let metadata_extension = Some(Metadata {
            name: Some("Traveler Name".into()),
            description: Some("Ever since you became a Cyborg, you've been feeling pretty weird...".into()),
            image: Some("ipfs://QmZdPdZzZum2jQ7jg1ekfeE3LSz1avAaa42G6mfimw9TEn".into()),
            dna: Some("Example DNA String".into()),
            species: Some(species.name),
            sapience_level: Some(species.sapience_level),
            issuer: Some(Addr::unchecked("archway1yvnw8xj5elngcq95e2n2p8f80zl7shfwyxk88858pl6cgzveeqtqy7xtf7")),
            origin: Some("earth".into()),
            identity: Some(Addr::unchecked("archway1f395p0gg67mmfd5zcqvpnp9cxnu0hg6r9hfczq")),
        });

        let token_id = "1".to_string();
        let mint_msg = MintMsg {
            token_id: token_id,
            owner: MINTER.to_string(),
            token_uri: None,
            extension: metadata_extension,
        };
        let execute_msg = ExecuteMsg::Mint(mint_msg.clone());
        contract.execute(deps.as_mut(), mock_env(), info, execute_msg).unwrap();

        let res = contract.nft_info(deps.as_ref(), token_id.into()).unwrap();

        assert_eq!(res.token_uri, mint_msg.token_uri); // off-chain metadata should be `None`
        assert_eq!(res.extension, mint_msg.extension); // on-chain metadata should be equal to `metadata_extension`
    }
}
```
