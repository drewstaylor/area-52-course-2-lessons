<!---
Course: 2 
Lesson: 1
Exercise: 2

Title: Cw721 Entry Points
Filename: quiz.txt

Storyline placeholder:
>
-->

### Overview of Cw721 entry points

Forgotten what entry points are? Check out [this](https://area-52.io/starting-with-cosm-wasm/1/cosmwasm-entry-points) lesson if you need to refresh your memory.

**Instantiate**

Since `cw721` is a specification, it requires an implementation (such as `cw721-base`) in order to be used on the blockchain. Messages and types for `Instantiate` are not included in `cw271`, since it's just a specification. You will however, find an implementation of [Instantiate](https://github.com/CosmWasm/cw-nfts/blob/main/contracts/cw721-base/src/lib.rs#L24-L34) included in [cw721-base](https://github.com/CosmWasm/cw-nfts/tree/main/contracts/cw721-base).

**Query**

In the previous exercise, we listed the `cw721` queries that must be implemented to conform to the `cw721` standard. For more information about their message types, you can look at the `query.rs` file in your coding window.

**Execute**

In the previous exercise, we listed the `cw721` transactions to be implemented to conform to the `cw721` standard. For more information about their message types, you can look at the `msg.rs` file in your coding window.

# Exercise
Quiz time! Place an `x` in any checkbox that is your answer.

# Starter
```markdown
Which of the following answers are **not** types that can be imported from `cw721`?
[] `InstantiateMsg`
[] `ExecuteMsg`
[] `Cw721QueryMsg`
[] `Cw721ExecuteMsg`
[] `NftInfo`

```

# Answer
```markdown
Which of the following answers are **not** types that can be imported from `cw721`?
[x] `InstantiateMsg`
[x] `ExecuteMsg`
[] `Cw721QueryMsg`
[] `Cw721ExecuteMsg`
[] `NftInfo`
```
