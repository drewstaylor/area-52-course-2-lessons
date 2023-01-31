# Lesson 3: Creating the NFT Collection Contract

**Topics:** `cw721-non-transferable`, Custom `cw721` token collections

**Path:** [passport-token](https://github.com/drewstaylor/area-52-course-2/tree/main/nft/passport-token)

### Exercises

- **Exercise 2:** _The Librarians are Watching_
  - Content
    - Enabling library features
  - Exercise
    - Enable library features for `cw721-soulbound` in `Cargo.toml`
    

- **Exercise 2:** _Cw721 with On-chain Metadata_
  - Content
    - Creating a metadata schema for the token collection
    - Using `serde` for your metadata
  - Exercise
    - Create a public struct called `Metadata` and assign it the correct member fields and their types

<!-- - **Exercise 3:** _Helpful Types, Aliases and Visibilty_
  - Content
    - Modify from [prework](https://github.com/phi-labs-ltd/area-52-course2-prework/blob/main/course-02-cw721/lesson-03_cw721-visa/exercise-07/exercise.md)
  - Create the following helper types and give them public visibility
    - pub type Extension = Option<Metadata>;
    - pub type Cw721MetadataContract<'a> = cw721_soulbound::Cw721Contract<'a, Extension, Empty, Empty, Empty>;
    - pub type ExecuteMsg = cw721_soulbound::ExecuteMsg<Extension, Empty>; -->

- **Exercise 3:** _Writing the Instantiate Entry Point Part 1_
  - Content
    - Setting the version information and crate declaration for a contract
  - Exercise
    - Set the contract version in the `instantiate` entry point

- **Exercise 4:** _Writing the Instantiate Entry Point Part 2_
  - Content
    - Validating an `addr` using `deps.api.addr_validate`
    - Saving to storage (remind them they've done this before in Course 1)
  - Exercise
    - Remind them they've done this before in Lesson 1
    - Finish writing the `instantiate` entry point
      - Set token `name` and `symbol`
      - Set `minter`

- **Exercise 5:** _Moving on to Query and Execute_
  - Content
    - Query and Execute entry points will use the default parameters of the `cw721-soulbound` token specification created in Lesson 2
  - Exercise
    - Remind them they've done this before in Lesson 1
    - Finish writing the `query` and `execute` endpoints

- **Exercise 6:** _Don't Forget to Reply_
  - Content
    - Reinforcing the utility of contracts that can Reply
  - Exercise
    - Remind them they've done this before in Lesson 1 and in Course 1.

- **Exercise 7:** _Writing Unit Tests Part 1_
  - Content
    - How to create unit tests in the same file
    - How to create a separate file just for unit tests
    - Using the `Extension` attribute for NFTs with on-chain metadata
    - Setting up a mock test environment to simulate a blockchain transaction
  - Exercise
    - Set up a mock test environment to simulate a blockchain transaction in a test
    - Create an `Extension` with the correct `Metadata` and types that match the schema created earlier in the lesson (e.g. in exercise 1)

- **Exercise 8:** _Writing Unit Tests Part 2_
  - Content
    - Calling a mock test environment from a test to simulate a blockchain transaction
    - Using the assertion macros (`assert` and `assert_eq`)
  - Exercise
    - Finish writing the unit test by adding the following:
      - Mint an NFT on the test blockchain
      - Query the `nft_info` entry point to get back the metadata of the NFT that was minted
      - Use `assert_eq` to make two integrity checks:
        - `token_uri` should be `None`
        - `extension` should be equal to the `Metadata` struct completed in the previous exercise

- **Exercise 9:** _Summary_
