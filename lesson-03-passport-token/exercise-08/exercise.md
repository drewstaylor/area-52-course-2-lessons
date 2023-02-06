<!---
Course: 2
Lesson: 3
Exercise: 8

Title: Writing Unit Tests Part 2
Filename: integration_tests.rs
-->

> Glancing at SECTION 31's finished Ring, what a sight to behold. The **STATUS** screen now reads "Preparing to perform diagnostic..." (in nano-digital yellow). Well, that's a huge improvement from what it said before: "Offline" (in bio-neon crimson).

Now that we have our mock blockchain environment setup inside a unit test, we can move on to the testing. The goal of our test is to verify setting the on-chain metadata works as expected for our NFT collection.

In `lib.rs` our metadata schema looks like this:

```rs
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
```

The `MintMsg<T>` struct comes from `cw721-soulbound`, where `T` is a generic to allow for different types and formats of metadata. 

```rs
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MintMsg<T> {
    pub token_id: String,
    pub owner: String,
    pub token_uri: Option<String>,
    pub extension: T,
}
```

In `passport-token`, `lib.rs` publicly exports as a type alias called `Extension` which is what `passport-token` uses for the `T` of `MintMsg<T>`:

```rs
// From `src/lib.rs`
pub type Extension = Option<Metadata>;
```

# Exercise

1. Create a variable called `mint_msg`. Assign it a `MintMsg` struct and write each of the four members on their own line. Assign the `token_id` struct field the value of the `token_id` string variable. `owner` will be the string version of the `MINTER` constant (created with `to_string`). Set `token_uri` to `None` (since we're using on-chain metadata), and set the `extension` field to `metadata_extension`.
2. Create a variable called `execute_msg` and assign it the `Mint` function from `ExecuteMsg`. Don't forget to pass it `mint_msg` but since we need to use `mint_msg` again you'll have to [clone](https://doc.rust-lang.org/std/clone/trait.Clone.html) it.
3. Call the `execute` function of the `contract` variable. The arguments you need to pass it, and its call format are similar to what you wrote for `contract.instantiate`. It's the last time we're using `info` so you won't need to `clone` it, and use the `execute_msg` you created in step 2 for it's `msg` parameter.
4. Write the `execute` call on a single line and [unwrap](https://docs.rs/unwrap/latest/unwrap/) it at the end.
5. Lastly, create a variable called `res` to store the query result. Assign it a query to `nft_info` (from `contract`) and pass in the parameters `deps` and `token_id`. Since this is just a query we won't be `DepsMut` this time, you can get a reference to `Deps` by called `deps.as_ref`. For the `token_id` argument use `token_id.into()` so the query auto-magically inserts the correct type. (Don't forget to [unwrap](https://docs.rs/unwrap/latest/unwrap/) the query result.)

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
            name: "Cyborg".to_string(),
            sapience_level: SapienceScale::High,
        };

        let metadata_extension = Some(Metadata {
            name: Some("Traveler Name".into()),
            description: Some("Ever since you became a cyborg, you've been feeling pretty weird...".into()),
            image: Some("ipfs://QmZdPdZzZum2jQ7jg1ekfeE3LSz1avAaa42G6mfimw9TEn".into()),
            dna: Some("Example DNA String".into()),
            species: Some(species.name),
            sapience_level: Some(species.sapience_level),
            issuer: Some(Addr::unchecked("archway1yvnw8xj5elngcq95e2n2p8f80zl7shfwyxk88858pl6cgzveeqtqy7xtf7")),
            origin: Some("earth".into()),
            identity: Some(Addr::unchecked("archway1f395p0gg67mmfd5zcqvpnp9cxnu0hg6r9hfczq")),
        });

        let token_id = "1".to_string();
        // Create `mint_msg` here
        // Create `execute_msg` here
        // Execute the `execute_msg` here

        // Create `res` here

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
