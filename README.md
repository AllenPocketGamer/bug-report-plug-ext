### Subject of the issue

The project which depends on `ic-agent` failed to parse the pem file exported from plug extension;

AND

The pem file exported from plug extension is rejected by dfx.

### Environment

- macOS Monterey
  - MacBook Pro(14-inch, 2021)
  - Chip Apple M1 Pro
  - Memory 16GB

- plug extension
  - Alpha Release - 0.5.1

- dfx
  - v0.10.0

### Step to reproduce

```rust
cargo run
```

```bash
dfx identity import plug-bug ./identity.pem
```

### Expected behaviour

no errors print

### Actual behaviour

```
failed to parse ./identity.pem from plug wallet to basic identity, A key was rejected by Ring: WrongAlgorithm
failed to parse ./identity.pem from plug wallet to secp256k1 identity, An error occurred while reading the file: missing data
```

```bash
Error: Invalid Ed25519 private key in PEM file
Caused by:
  A key was rejected by Ring: WrongAlgorithm
    WrongAlgorithm
```