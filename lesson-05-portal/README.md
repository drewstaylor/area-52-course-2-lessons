# Lesson 5: Using NFTs in Other Contracts Part 2

**Topics:** `cw721-non-transferable`, Custom `cw721` token collections, `WasmQuery`, `NftInfo`

**Path:** [passport-token](https://github.com/drewstaylor/area-52-course-2/tree/main/nft/passport-token)

### Exercises

- **Exercise 1:** _Verifying Identity With NFTs Part 1_
  - Content
    - Using `NftInfo` to get the metadata of a specific NFT
  - Exercise
    - Write the message and query request for a query to the token collection (entry point: Cw721QueryMsg::NftInfo) that checks if a specific token exists (e.g. `token_id`)

- **Exercise 2:** _Verifying Identity With NFTs Part 2_
  - Content
    - Using `NftInfo` to get the metadata of a specific NFT
  - Exercise
    - Write the code to execute the query request created in the previous lesson, remember to catch any errors using the `?` operator (`deps.querier.query(&query_req)?;`)

<!-- - **Exercise 3:** _Verifying Identity With NFTs Part 3_
  - Content
    - Reinforcing returning errors and thinking about the identity system created so far
    - Explain why this optional check is unneccesary provided we've gotten everything right so far (have we gotten everything right so far?)
  - Exercise
    - Write an additional check to enforce that we aren't sending an authorized traveler through the JumpRing

- **Exercise 4:** _Restricting the Entry Point Caller_
  - Content
    - Reinforcing that only portal can initiate jump ring travel
    - Remind them they've seen this security code already (e.g. when they worked on the minting entry point)
  - Exercise
    - Write an if statement that ensures only Potion contract can call `initiate_jump_ring` -->

- **Exercise 3:** _Verifying Identity With NFTs Part 3_
  - Content
    - Reinforcing what we know about message responses
    - Returning multiple response attributes
  - Exercise
    - Replace `Response::default()` with `Response:new()` and add two attributes to the entry point response
    - As is standard, the first attribute to be returned is the `action`; give it a value of `initiate_jump_ring_travel` (which is the `Execute` entry point being called)
    - The second attribute to be added to the `Response` is the `traveler` being sent through the Portal. It's value will be the `traveler` (as sent to the entry point as an argument)
    
- **Exercise 4:** _Modifying the JumpRingTravel Execute Arguments_
  - Content
    - Reinforcing how to enable execute entry points
  - Exercise
    - Add the `traveler` attribute as a second function argument of `JumpRingTravel`

- **Exercise 5:** _Summary_