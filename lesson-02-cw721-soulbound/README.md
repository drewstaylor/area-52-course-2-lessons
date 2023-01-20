# Lesson 2: Customizing your NFTs

**Topics:** `cw721`, `cw721-base`, `cw721-non-transferable`,

**Path:** [cw721-soulbound](https://github.com/drewstaylor/area-52-course-2/tree/main/nft/cw721-soulbound)

### Exercises

- **Exercise 1:** _Contracts and Libraries_
  - Content
    - Contractors vs Libarians
    - Changing `cw721-base` into something custom, for specific use cases
  - Exercise
    - Make the correct types public

- **Exercise 2:** _Contractors and Librarians_
  - Content
    - Changing `cw721-base` into something custom, for your own specific use cases
  - Exercise
    - Change the code in `execute.rs` for the functions `transfer_nft` and `send_nft`
    - Since we can't remove this code without modifying `cw721` (impl. err), just return an `Unauthorized ` error

- **Exercise 3:** _Librarians Part 1_
  - Content
    - Changing `package/cw721` into something custom, for your own specific use cases
  - Exercise
    - Remove `transfer_nft`, and `send_nft` from `trait.rs`

- **Exercise 3:** _Librarians Part 2_
  - Content
    - Changing `cw721-base` into something custom, for your own specific use cases
  - Exercise
    - Remove `TransferNft` and `SendNFT` from `msg.rs`

- **Exercise 4**: _Librarians Part 3_
  - Content
    - Changing `cw721-base` into something custom, for your own specific use cases
    - Don't forget to update both `Cargo.toml` files now that you changed the behaviour of these packages!
  - Exercise 
    - Update `Cargo.toml` with the new package name (e.g. `cw721-soulbound`)

- **Exercise 5:** _Summary_
