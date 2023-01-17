# Lesson 1: Introduction to CosmWasm NFTs

**Topics:** `cw721`, `cw721-base`

**Path:** [cw721-base](https://github.com/CosmWasm/cw-nfts/tree/main/contracts/cw721-base)

### Exercises

- **Excerise 1:** _CW what?_
  - Content
    - Overview of the `cw721` specification
    - Relationship of `cw721` `cw721-base`
    - Other officially supported types (`cw2981-royalties`, `cw721-fixed-price`, `cw721-metadata-onchain`, `cw721-non-transferable`)
  - Exercise
    - Quiz format, modified from [prework1](https://github.com/phi-labs-ltd/area-52-course2-prework/blob/main/course-02-cw721/lesson-01_cw721-base_contract-structure/exercise-01/exercise-01.md)
    - Re-phrase central question from [prework1](https://github.com/phi-labs-ltd/area-52-course2-prework/blob/main/course-02-cw721/lesson-01_cw721-base_contract-structure/exercise-01/exercise-01.md) in terminology that hinges on the relationship of `cw721` to `cw721-base`

- **Exercise 2:** _A Basic Overview of cw721_
  - Content
    - Overview of all entry points (`Instantiate`, `Query`, `Execute`)
  - Exercise
    - Quiz format
    - "Which answer is not a message type that can be imported from `cw721`?"
  

- **Exercise 3:** _Not all NFT metadata is created equal_
  - Content 
    - Creating `cw721` with on-chain vs off-chain metadata
  - Exercise
    - Create an off-chain metadata NFT by setting `token_uri` to an IPFS value, and setting `extension` to `None`

- **Exercise 4:** _When to use `cw721`_
  - Content 
    - Clear up distinction of `cw721` meaning "an NFT", vs the more specific meaning (e.g. what the lesson is referring to) of the source code package and crate
    - Queries and query responses
  - Exercise
    - Finish writing the query function

- **Exercise 5:** _When to use `cw721-base`_
  - Content 
    - Creating the token collection contract (e.g. `passport-token`)
    - Introduces the idea of what it means to change `cw721-base` into our own custom NFT code
  - Exercise
    - Elaborates on exercise 2's code
    - Start creating the token collection contract for an NFT with off-chain metadata (Instantiate)

- **Exercise 6:** _Creating the `Execute` and `Query` entry points_
  - Content
  - Exercise
    - Elaborates on exercise 4's code
    - Continue creating the token collection contract for an NFT with off-chain metadata (Execute, Query)

- **Exercise 7:** _Finishing the collection contract_
  - Content
    - Using Reply
    - Remind them they've done this before in Course 1
  - Exercise
    - Elaborates on exercise 5's code
    - Finish creating the token collection contract for an NFT with off-chain metadata (Reply)
  
- **Exercise 8:** _Summary_