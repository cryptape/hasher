# Hasher

Package `hasher` provides a Hasher trait.

```rs
pub trait Hasher {
    const LENGTH: usize;

    fn digest(&self, data: &[u8]) -> Vec<u8>;
}
```

Add this to your `Cargo.toml`:

```toml
[dependencies]
hasher = "0.1"
```

# Supported algorithms

- blake2b
- keccak
- sm3

# Test

```
$ cargo test --all-features
```
