<!---
Course: 2
Lesson: 3
Exercise: 7

Title: Writing Unit Tests Part 1
Filename: integration_tests.rs

Storyline placeholder:
>
-->

[Tests](https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html) are Rust functions that verify that the non-test code is functioning in the expected manner. The bodies of test functions typically perform some setup, run the code we want to test, then assert whether the results are what we expect.

Writing unit tests for CosmWasm contracts is important, because it allows you to prove your code is safe before users do financial transactions with it (such as buying, or trading NFTs).

Tests can either be written in the same file as the code being tested, or in a separate file (especially if the test coverage is large).

We care declare test like this

```rs
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn testing_something() {
        // Test code goes here
    }
}
```

In the case of writing tests in a separate file (e.g. `integration_tests.rs`), your `lib.rs` must instance them as a [module](https://doc.rust-lang.org/reference/items/modules.html) using the `mod` keyword. The `mod` for tests should _not_ be public since it does not provide utility or types for other developers to use when inheriting your package as a library.

```rs
mod integration_tests;
```

Tests can be run with the command `cargo test`, if you're inside a project folder.

```bash
$ cargo test
    Finished test [unoptimized + debuginfo] target(s) in 0.04s
     Running unittests src/integration_tests.rs (target/debug/deps/passport_token-1fa1e432c1dd51c8)

running 1 test
test integration_tests::use_metadata_extension ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

### Testing Pitfalls

When it comes to writing tests, avoid being lulled into a false sense of security. Below are some common issues developers may face when writing tests.

- Tests don't prove what they set out to (missed edge case)
- Tests don't simulate how code would behave on the blockchain (inadequate environment)
- Incomplete test coverage (test cases missing)
- Hard coded parameters not indicative of real world performance (inadequate environment)

# Exercise

In this exercise we'll set up a mock test environment to simulate a blockchain transaction in our unit test. Our end goal is to mint a `passport-token` in this test environment, and verify its on-chain metadata works as expected.

1. Create a mutable variable called `deps` and assign it a call to [mock_dependencies](https://docs.rs/cosmwasm-std/0.9.2/cosmwasm_std/testing/fn.mock_dependencies.html) without passing in any function arguments
2. Create a variable called `contract` and assign it to `Cw721MetadataContract::default()`
3. Create a variable called `info` and assign it a call to [mock_info](https://docs.rs/cosmwasm-std/0.16.0-rc1/cosmwasm_std/testing/fn.mock_info.html) and pass it two arguments. The first argument is the `MINTER` constant, the second is an empty array reference (`&[]`).
4. Instantiate the contract call the `instantiate` function, which is an attribute of `contract`, and pass it the usual `instantiate` parameters (`deps`, `env`, `info`, and `msg`). You can get a mock instance of `DepsMut` by calling `deps.as_mut`. You can get a mock instance of `env` by calling [mock_env](https://docs.rs/cosmwasm-std/0.16.0-rc1/cosmwasm_std/testing/fn.mock_env.html). For the other two (`info` and `msg`), use the `info` variable you created in step 3, and the `instantiate_msg` you've been provided with.
5. Write the contract instantiation on a single line, and [unwrap](https://docs.rs/unwrap/latest/unwrap/) it at the end.

<!-- contract.instantiate(deps.as_mut(), mock_env(), info.clone(), instantiate_msg).unwrap(); -->

# Starter

```rs
#![cfg(test)]

mod tests {
    use crate::Cw721MetadataContract;

    use cw721_soulbound::{InstantiateMsg};
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};

    const MINTER: &str = "jumpring";    // Each JumpRing mints passports and handles passport validation;
                                        // (Like airport security and an intergalactic embassy combined)

    #[test]
    fn use_metadata_extension() {
        // Create `deps` here
        // Create `contract` here
        // Create `info` here

        let instantiate_msg = InstantiateMsg {
            name: "passport token".to_string(),
            symbol: "PASS".to_string(),
            minter: MINTER.to_string(),
        };

        // Instantiate the contract here
    }
}
```

# Answer

```rs
#![cfg(test)]

mod tests {
    use crate::Cw721MetadataContract;

    use cw721_soulbound::{InstantiateMsg};
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

        contract.instantiate(deps.as_mut(), mock_env(), info, instantiate_msg).unwrap();
    }
}
```