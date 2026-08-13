# Retium Quest and Reputation System (RQS)

A decentralized Quest, XP, and on-chain Reputation system built in Rust under the unified **RCP-1** contract standard for the upcoming **Retium** blockchain ecosystem.

## Features
- **User Registration:** Allows unique users to register their profiles securely.
- **Dynamic Quests:** Associates quest completions with cryptographic validation, rewarding users with non-transferable XP.
- **Soulbound Reputation Badges:** Automatically awards unique, non-transferable RCP-1 achievement badges based on the user's XP threshold:
  - **100+ XP:** "Bronze Explorer"
  - **350+ XP:** "Silver Builder"
  - **850+ XP:** "Gold ZK-Master"
- **Duplicate Claim Prevention:** Validates state transitions to prevent double-claiming of quests natively.

## Structure
```text
retium-quest-app/
├── src/
│   └── lib.rs                  # RCP-1 contract logic & native unit tests
├── Cargo.toml                  # Cargo package manifest
└── README.md                   # Project documentation

