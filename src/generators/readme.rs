use crate::IDL;
use convert_case::{Case, Casing};

pub fn generate_readme(idl: &IDL) -> String {
    let name = idl.name.as_ref().map(|n| n.to_case(Case::Title)).unwrap_or("SDK".to_string());
    let kebab_name = idl.name.as_ref().map(|n| n.to_case(Case::Kebab)).unwrap_or("sdk".to_string());
    let snake_name = idl.name.as_ref().map(|n| n.to_case(Case::Snake)).unwrap_or("sdk".to_string());
    let default_version = "0.1.0".to_string();
    let version = idl.version.as_ref().unwrap_or(&default_version);

    format!(r#"# {name} SDK

This Rust SDK was generated from an [IDL](https://book.anchor-lang.com/anchor/idl.html) file and provides tools for interacting with the `{name}` Solana program.

## 📦 Installation

To use this crate, add the following to your `Cargo.toml`:

```toml
{snake_name}_sdk = {{ git = "https://github.com/mfg-labs/{kebab_name}-sdk", branch = "main" }}
```

Or import it from a local path if you're testing locally:

```toml
{snake_name}_sdk = {{ path = "../{kebab_name}-sdk" }}
```

## 🧩 Features

- **Instructions**: Structs and serialization logic for all program instructions.
- **Accounts**: Anchor-compatible account representations.
- **CPI**: Context builders and invocation helpers for cross-program invocation.
- **RPC**: Structs for client-side account metas and instruction construction.
- **I11n**: Introspection helpers to decode raw transactions.
- **Events**: Strongly-typed deserialization of emitted events.

## 🚀 Usage

```rust
use anchor_lang::prelude::*;
use {snake_name}_sdk::instructions::*;
use {snake_name}_sdk::rpc::*;

// Construct the arguments for the instruction
let ix = MyInstructionName {{
    field_one: 42,
    field_two: true,
}};

// Serialize the instruction
let mut data = Vec::new();
data.extend_from_slice(&MyInstructionName::DISCRIMINATOR);
ix.serialize(&mut data).unwrap();

// Build the Solana instruction
let accounts = MyInstructionNameRpc {{
    authority: Pubkey::new_unique(),
    some_account: Pubkey::new_unique(),
}};
let metas = accounts.to_account_metas(None);

let instruction = solana_program::instruction::Instruction {{
    program_id: ID, // imported from the SDK
    accounts: metas,
    data,
}};
```

## 🔒 License

MIT or Apache-2.0
"#, name=name, kebab_name=kebab_name, snake_name=snake_name)
}
