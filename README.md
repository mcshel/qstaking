# Qstaking

*Qstaking* is a custom implementation of NFT staking program. It is partially based on [Gemworks](https://github.com/gemworks).

## Requirements

1. Node v16.17.0
2. Yarn v1.22.19
3. Rust 1.63.0
4. Anchor 0.24.2

## Installation

1. Install dependencies using `yarn`
2. Build using `anchor build`
3. Update the generated program address in `programs/qstaking/lib.rs` and `Anchor.toml`
4. Re-build using `anchor build`
5. Deploy on-chain using `solana program deploy target/deploy/qstaking.so`
6. (Optional) Test using `anchor test --skip-local-validator --skip-deploy --skip-build`


## Usage

For usage examples see the tests.
