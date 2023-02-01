<!---
Course: 2 
Lesson: 2
Exercise: 3

Title: Non-transferable NFTs Part 1
Filename: packages/cw721/traits.rs

Storyline placeholder:
>
-->

Whoa, speaking of the previous exercise, was it easy to get lost in that code? There must be a better way to implement the custom behavior for [soulbound](https://vitalik.ca/general/2022/01/26/soulbound.html) NFTs in CosmWasm. 

Taking a stern gaze from the libarians, let's try again with a modular package design.

Remember this `Cargo.toml` code?

```yaml
[dependencies]
example-local-package = { path = "../example-local-package", version = "0.1.0" }
```

It's time we used that approach. Instead of bootstrapping `cw721-base` on directly on top of our project (in the same `src` folder), this time we can start by [cloning](https://git-scm.com/docs/git-clone) the [cw721-base](https://github.com/CosmWasm/cw-nfts/tree/main/contracts/cw721-base) repository into a separate folder. After we've made it "soulbound", we'll make the token collection contract and declare `cw721-soulbound` as a dependency.

# Exercise

Proceeding should be much easier now, we can simply remove references to `TransferNft` and `SendNft` wholesale from our new package (`cw721-soulbound`). There's a few files to be changed, but we'll tackle them one at a time.

`cw721-base` has a `packages` folder, this is where the default (e.g. transferrable) `cw721` spec lives. We'll remove the transfer traits from `cw721` first, to arrive at a soulbound `cw721` spec.

1. Locate and remove `transfer_nft`, and `send_nft`

# Starter

```rs
use schemars::JsonSchema;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::query::ApprovalResponse;
use crate::{
    AllNftInfoResponse, ApprovalsResponse, ContractInfoResponse, NftInfoResponse,
    NumTokensResponse, OperatorsResponse, OwnerOfResponse, TokensResponse,
};
use cosmwasm_std::{Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult};
use cw_utils::Expiration;

pub trait CustomMsg: Clone + std::fmt::Debug + PartialEq + JsonSchema {}

impl CustomMsg for Empty {}

pub trait Cw721<T, C>: Cw721Execute<T, C> + Cw721Query<T>
where
    T: Serialize + DeserializeOwned + Clone,
    C: CustomMsg,
{
}

pub trait Cw721Execute<T, C>
where
    T: Serialize + DeserializeOwned + Clone,
    C: CustomMsg,
{
    type Err: ToString;

    fn transfer_nft(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        recipient: String,
        token_id: String,
    ) -> Result<Response<C>, Self::Err>;

    fn send_nft(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        contract: String,
        token_id: String,
        msg: Binary,
    ) -> Result<Response<C>, Self::Err>;

    fn approve(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        spender: String,
        token_id: String,
        expires: Option<Expiration>,
    ) -> Result<Response<C>, Self::Err>;

    fn revoke(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        spender: String,
        token_id: String,
    ) -> Result<Response<C>, Self::Err>;

    fn approve_all(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        operator: String,
        expires: Option<Expiration>,
    ) -> Result<Response<C>, Self::Err>;

    fn revoke_all(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        operator: String,
    ) -> Result<Response<C>, Self::Err>;

    fn burn(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        token_id: String,
    ) -> Result<Response<C>, Self::Err>;
}

pub trait Cw721Query<T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    fn contract_info(&self, deps: Deps) -> StdResult<ContractInfoResponse>;

    fn num_tokens(&self, deps: Deps) -> StdResult<NumTokensResponse>;

    fn nft_info(&self, deps: Deps, token_id: String) -> StdResult<NftInfoResponse<T>>;

    fn owner_of(
        &self,
        deps: Deps,
        env: Env,
        token_id: String,
        include_expired: bool,
    ) -> StdResult<OwnerOfResponse>;

    fn operators(
        &self,
        deps: Deps,
        env: Env,
        owner: String,
        include_expired: bool,
        start_after: Option<String>,
        limit: Option<u32>,
    ) -> StdResult<OperatorsResponse>;

    fn approval(
        &self,
        deps: Deps,
        env: Env,
        token_id: String,
        spender: String,
        include_expired: bool,
    ) -> StdResult<ApprovalResponse>;

    fn approvals(
        &self,
        deps: Deps,
        env: Env,
        token_id: String,
        include_expired: bool,
    ) -> StdResult<ApprovalsResponse>;

    fn tokens(
        &self,
        deps: Deps,
        owner: String,
        start_after: Option<String>,
        limit: Option<u32>,
    ) -> StdResult<TokensResponse>;

    fn all_tokens(
        &self,
        deps: Deps,
        start_after: Option<String>,
        limit: Option<u32>,
    ) -> StdResult<TokensResponse>;

    fn all_nft_info(
        &self,
        deps: Deps,
        env: Env,
        token_id: String,
        include_expired: bool,
    ) -> StdResult<AllNftInfoResponse<T>>;
}
```

# Answer

```rs
use schemars::JsonSchema;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::query::ApprovalResponse;
use crate::{
    AllNftInfoResponse, ApprovalsResponse, ContractInfoResponse, NftInfoResponse,
    NumTokensResponse, OperatorsResponse, OwnerOfResponse, TokensResponse,
};
use cosmwasm_std::{Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult};
use cw_utils::Expiration;

pub trait CustomMsg: Clone + std::fmt::Debug + PartialEq + JsonSchema {}

impl CustomMsg for Empty {}

pub trait Cw721<T, C>: Cw721Execute<T, C> + Cw721Query<T>
where
    T: Serialize + DeserializeOwned + Clone,
    C: CustomMsg,
{
}

pub trait Cw721Execute<T, C>
where
    T: Serialize + DeserializeOwned + Clone,
    C: CustomMsg,
{
    type Err: ToString;

    fn approve(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        spender: String,
        token_id: String,
        expires: Option<Expiration>,
    ) -> Result<Response<C>, Self::Err>;

    fn revoke(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        spender: String,
        token_id: String,
    ) -> Result<Response<C>, Self::Err>;

    fn approve_all(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        operator: String,
        expires: Option<Expiration>,
    ) -> Result<Response<C>, Self::Err>;

    fn revoke_all(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        operator: String,
    ) -> Result<Response<C>, Self::Err>;

    fn burn(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        token_id: String,
    ) -> Result<Response<C>, Self::Err>;
}

pub trait Cw721Query<T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    fn contract_info(&self, deps: Deps) -> StdResult<ContractInfoResponse>;

    fn num_tokens(&self, deps: Deps) -> StdResult<NumTokensResponse>;

    fn nft_info(&self, deps: Deps, token_id: String) -> StdResult<NftInfoResponse<T>>;

    fn owner_of(
        &self,
        deps: Deps,
        env: Env,
        token_id: String,
        include_expired: bool,
    ) -> StdResult<OwnerOfResponse>;

    fn operators(
        &self,
        deps: Deps,
        env: Env,
        owner: String,
        include_expired: bool,
        start_after: Option<String>,
        limit: Option<u32>,
    ) -> StdResult<OperatorsResponse>;

    fn approval(
        &self,
        deps: Deps,
        env: Env,
        token_id: String,
        spender: String,
        include_expired: bool,
    ) -> StdResult<ApprovalResponse>;

    fn approvals(
        &self,
        deps: Deps,
        env: Env,
        token_id: String,
        include_expired: bool,
    ) -> StdResult<ApprovalsResponse>;

    fn tokens(
        &self,
        deps: Deps,
        owner: String,
        start_after: Option<String>,
        limit: Option<u32>,
    ) -> StdResult<TokensResponse>;

    fn all_tokens(
        &self,
        deps: Deps,
        start_after: Option<String>,
        limit: Option<u32>,
    ) -> StdResult<TokensResponse>;

    fn all_nft_info(
        &self,
        deps: Deps,
        env: Env,
        token_id: String,
        include_expired: bool,
    ) -> StdResult<AllNftInfoResponse<T>>;
}
```
