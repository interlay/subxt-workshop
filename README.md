# Subxt Workshop (Polkadot Decoded 2022)

## Getting Started

### Install Rust

Follow the [Substrate docs](https://docs.substrate.io/main-docs/install/rust-builds/).

### Download Polkadot

```shell
curl "https://github.com/paritytech/polkadot/releases/download/v0.9.18/polkadot" --output /usr/local/bin/polkadot --location
```

## Instructions

Run the Polkadot node in a separate terminal:

```shell
polkadot --dev --tmp

> 2022-06-23 21:43:57 Parity Polkadot    
> 2022-06-23 21:43:57 ✌️  version 0.9.18-99cd17ddb2-x86_64-linux-gnu    
> 2022-06-23 21:43:57 ❤️  by Parity Technologies <admin@parity.io>, 2017-2022    
> 2022-06-23 21:43:57 📋 Chain specification: Development    
> ...
```

Run the test suite:

```shell
cargo test
```

Notice the test failures, your task is to implement the helper functions.
The files in the [`tests/`](tests/) directory are ranked in order of difficulty.

1. [Exercise 01](tests/01-get-block-number.rs) - Get the block number for a given `block_hash`.
2. [Exercise 02](tests/02-get-balance.rs) - Get the `free` balance of an `account`.
3. [Exercise 03](tests/03-get-total-frozen.rs) - Sum the total `frozen` balance.
4. [Exercise 04](tests/04-get-first-n-accounts.rs) - Fetch the first `n` accounts from storage.
5. [Exercise 05](tests/05-get-version.rs) - Return the embedded `RuntimeVersion`.
6. [Exercise 06](tests/06-transfer-balance.rs) - Transfer an `amount` from the `signer` to `dest`.
7. [Exercise 07](tests/07-estimate-inclusion-fee.rs) - Estimate the fees for an extrinsic using a custom RPC.
8. [Exercise 08](tests/08-batch-transfer.rs) - Make multiple transfers in the same call.
9. [Exercise 09](tests/09-propose-spend.rs) - Create a spending proposal and check the deposit.
10. [Exercise 10](tests/10-approve-multisig.rs) - Listen and approve multisig operations.

## Other Notes

### Refresh Metadata

This is only required if you change the node / runtime.

```shell
cargo install subxt-cli
subxt metadata -f bytes > polkadot_metadata.scale
```

### Examine Metadata

Verbose but helpful expansion of the generated Rust code.

```shell
cargo install cargo-expand
cargo expand
```

## Credits

Thanks to all subxt / substrate maintainers and contributors.