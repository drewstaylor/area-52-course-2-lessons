# Lesson 4: Using NFTs in Other Contracts Part 1

**Topics:** `cw721-non-transferable`, Custom `cw721` token collections, `WasmQuery`, `WasmMsg::Execute`, `vec!()` macro

**Path:** [passport-token](https://github.com/drewstaylor/area-52-course-2/tree/main/nft/passport-token)

### Exercises

- **Exercise 1:** _Preparing Portal to Become the Minter Part 1_
  - Content
    - Saving and updating the NFT contract address in JumpRing
  - Exercise
    - Finish writing the `set_passport_contract` and `set_potion_contract` functions in `execute_fn.rs` of the Portal contract

- **Exercise 2:** _Preparing Portal to Become the Minter Part 2_
  - Content
    - Exposing the two new execute functions, created in the previous exercise, as entry points to `Execute` of the Portal contract
  - Exercise
    - Add the two new entrypoints to `execute` in `contract.rs` (look at `msg.rs` and `state.rs` for help regarding their types)

- **Exercise 3:** _Minting NFTs From a Contract Part 1_
  - Content
    - Querying NFTs from another contract
    - `WasmQuery::Smart`
  - Exercise
    - Write a query to the token collection (entry point: `Cw721QueryMsg::Tokens`) that checks if a specific user address is holding any passport tokens
    - If the already holding a passport token, fail with an `IllegalAlien` contract error (e.g. `ContractError`; see `error.rs`)

- **Exercise 4:** _Minting NFTs From a Contract Part 2_
  - Content
    - Serde requirements for NFT's with on-chain metadata
    - `MintMsg`
  - Exercise
    - Create a variable called `metadata_extension`, of type `Extension`, and assign it the correct members required by `Extension` as enforced by the token collection contract (See: `passport_token`). Most of it's values will come from the `msg` sent directly to the `mint_passport` function; however, `issuer`'s value will come from the environment (`env.contract.address.clone()`) and `origin` will come from the Portal contract's state (`config.planet_name`).
    - Create a variable called `mint_msg`, of type `passport_token::ExecuteMsg`, and assign it the correct members required

- **Exercise 5:** _Minting NFTs From a Contract Part 3_
  - Content
  - Exercise

- **Exercise 6:** _Using Vectors for Contract Responses_
  - Content
    - Vector responses from contracts
  - Exercise
    - Using the `vec` macro, create a variable called `messages` and add `mint_resp` to it
    - Modify `mint_passport` so that it sends the `messages` vector as it's response

- **Exercise 7:** _Creating a New Message Type_
  - Content
    - Reinforcing Cosmos message types
  - Exercise
    - Create and add the new message type in `msg.rs`

- **Exercise 8:** _Finalizing the Mint Passport Entry Point_
  - Content
    - Reinforcing how to enable execute entry points
  - Exercise
    - Add the new entry point for `mint_passport` to `execute` in `contract.rs`

- **Exercise 9:** _Summary_