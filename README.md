# Q1_26_accel_parth

A collection of Solana smart contracts and Rust projects exploring various blockchain development patterns and technologies.

## Projects

| Project | Description | Technologies |
|---------|-------------|--------------|
| [whitelist-transfer-hook](./whitelist-transfer-hook/) | SPL Token 2022 transfer hook implementation that enforces whitelist restrictions on token transfers. Only whitelisted addresses can transfer tokens with this hook enabled. | Anchor 0.32.1, SPL Token 2022, TypeScript |
| [tuktuk-escrow](./tuktuk-escrow/) | Escrow program using TukTuk SDK for scheduled transactions and Cron SDK for time-based execution | Anchor 0.31.0, TukTuk SDK, Helium Cron SDK |
| [magicblock-er-example](./magicblock-er-example/) | Example implementation using MagicBlock's ephemeral rollups SDK for high-performance state management | Anchor 0.32.1, MagicBlock ER SDK |
| [escrow-litesvm](./escrow_litesvm/) | Basic escrow contract tested with LiteSVM for fast local Solana program testing | Anchor 0.30.1, LiteSVM |
| [transfer-hook-vault](./week1_challenge/) | Token transfer hook with vault functionality for securing tokens during transfers | Anchor 0.32.1, SPL Token 2022 |
| [gpt-oracle-tuktuk](./gpt-oracle-tuktuk/) | GPT-powered pricing oracle with TukTuk scheduling for AI-based price feeds | Anchor 0.32.1, TukTuk SDK, SPL Token |
| [generic-storage](./generic_storage) | Rust learning project: format-agnostic storage system using traits, generics, and PhantomData with Borsh, Wincode, and JSON serializers. |
| [scheduler](./scheduler/) | Task Scheduler is a small, time‑ and priority‑based job schedule, it lets you queue jobs for future execution, pick which function to run, and watch execution logs in real time. | Rust, Ratatui, Crossterm |
| [accel-pinocchio-escrow](./accel-pinocchio-escrow/) | High-performance escrow implementation using Pinocchio framework for optimized Solana programs with reduced compute cost | Pinocchio 0.10.2, LiteSVM, Wincode, Borsh |
| [pinocchio-fundraiser](./pinocchio-fundraiser/) | Crowdfunding/fundraiser program built with Pinocchio framework for minimal compute unit usage | Pinocchio 0.10.2, LiteSVM, SPL Token 2022 |
| [nft-staking-core](./nft-staking-core/) | NFT staking implementation using Metaplex Core (MPL Core) for next-gen NFT standards | Anchor 0.32.1, MPL Core |