# Hasher

Package `hasher` provides a Hasher trait.

```rs
pub trait Hasher {
    fn hash(&self, data: &[u8]) -> Vec<u8>;
}
```

# Test

```
$ cargo test --all-features
```
