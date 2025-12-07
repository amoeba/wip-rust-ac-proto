# wip-rust-ac-proto

## Repository Structure

### Crates

### `acprotocol` crate

```text
 generated/
  ├── mod.rs                    # Root module file
  ├── enums/
  │   └── mod.rs               # Enums module
  ├── types/
  │   ├── mod.rs               # Types module (NEW)
  │   └── common.rs            # Common types
  └── messages/
      ├── mod.rs               # Messages module
      ├── c2s/
      │   ├── mod.rs           # C2S module
      │   └── TypeName.rs      # Individual message files (171 files)
      └── s2c/
          ├── mod.rs           # S2C module
          └── TypeName.rs      # Individual message files (192 files)
```
