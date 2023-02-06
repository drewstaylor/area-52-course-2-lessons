<!---
Course: 2 
Lesson: 1
Exercise: 1

Title: Cw-What?
Filename: quiz.txt
-->

> Do cyborgs dream of electric sheep? You hate to admit it, but ever since you became a cyborg you've been feeling super weird. You've been having the strangest dreams–which is also perplexing, since you have almost no desire anymore to fulfill your biological needs like nourishment and sleep. Then one morning, the strangest thing of all happened. Waking up, you look around and you have no idea where you are!

# Cw-What?

[Cw721](https://github.com/CosmWasm/cw-nfts/blob/main/packages/cw721/README.md) is a specification for [non-fungible tokens](https://en.wikipedia.org/wiki/Non-fungible_token) created with [CosmWasm](https://cosmwasm.com/). The name and design are based on Ethereum's [ERC721 standard](https://eips.ethereum.org/EIPS/eip-721), with some enhancements. 

`cw721` types can be imported by contracts that wish to implement non-fungible tokens according to the `cw721` specification, or by contracts that call some contract that implements something from the `cw721` specification (e.g. `Response` types).

### Overview of the cw721 specification

To conform to the `cw721` spec, the following _must_ be implemented in the token contract.

**Transactions:**

1. `Approve`
2. `Revoke`
3. `ApproveAll`
4. `RevokeAll`

**Queries:**

1. `OwnerOf`
2. `Approval`
3. `Approvals`
4. `AllOperators`
5. `NumTokens`
6. `Tokens`
7. `AllTokens`

**Tip:** _Use the 'Help' button to view detailed information about the above, such as what they can do, their calling arguments and their responses._

Additionally, `cw721` tokens which do not implement the following transfer types, are "[soulbound](https://vitalik.ca/general/2022/01/26/soulbound.html)" or "non-transferable" tokens; additionally, tokens implementing _one_ but not _both_ of the following will not properly conform to the `cw721` standard.

**Transfer Transactions:**

1. `TransferNft`
2. `SendNft`

### Relationship of cw721 to cw721-base

There are several supported token templates can developers can use to build their NFT projects. The most common one is called [cw721-base](https://crates.io/crates/cw721-base).

You may have noticed that `Mint` was not included in our list of `cw721` requirements. This is by design, since it gives developers full control over how their tokens will be created and distributed to collectors. 

`cw721-base` is an implementation of the `cw721` specification that _does_ include source code and message types for minting tokens. It  does so in a very specific way, since in `cw721-base` only one Cosmos address is given permission to mint tokens. This address, called the `minter`, and has an admin like privilege to create and distribute NFTs of the contract. Regular users *cannot* call `Mint` directly, but `minter` *can* mint NFTs directly to any user (e.g. without requiring a second transaction to send the minted NFT from the contract to the user).

### Other supported (and related) cw721 standards

While most NFT projects will extend from [cw721-base](https://github.com/CosmWasm/cw-nfts/tree/main/contracts/cw721-base), there are some other supported implementations you'll want to be aware of.

- [cw2981-royalties](https://github.com/CosmWasm/cw-nfts/tree/main/contracts/cw2981-royalties)
- [cw721-fixed-price](https://github.com/CosmWasm/cw-nfts/tree/main/contracts/cw721-fixed-price)
- [cw721-metadata-onchain](https://github.com/CosmWasm/cw-nfts/tree/main/contracts/cw721-metadata-onchain)
- [cw721-non-transferable](https://github.com/CosmWasm/cw-nfts/tree/main/contracts/cw721-non-transferable)

We'll be learning more about `cw721-metadata-onchain` and `cw721-non-transferable` later on in this course. For the other two, suffice  to say that `cw2981-royalties` is based on [Ethereum's standard for NFT royalties](https://eips.ethereum.org/EIPS/eip-2981) and `cw721-fixed-price` is for NFTs which enforce a specific and constant sale price.

# Exercise
Quiz time! Simply place an `x` in the checkbox that is your answer.

# Starter
```markdown
Which of these is *not* in the `cw721` token specification?
[] `SendNft`
[] `OwnerOf`
[] `Mint`
[] `ReceiveNft`

```

# Answer
```markdown
Which of these is *not* in the `cw721` token specification?
[] `SendNft`
[] `OwnerOf`
[x] `Mint`
[] `ReceiveNft`
```
