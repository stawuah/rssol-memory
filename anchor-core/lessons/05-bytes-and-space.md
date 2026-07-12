# 05 Bytes and Space

## Analogy

Account space is the size of the locker before you put anything in it.

- fixed-size values are predictable drawers
- `Vec<T>` is a growing box with a length prefix
- `String` is a UTF-8 byte buffer with a length prefix
- zero-copy is direct access to the locker contents without unpacking everything

## Official facts

Anchor’s account space page lists byte sizes for common Rust types and reminds you to add 8 bytes for the account discriminator.

- [Anchor Account Space](https://www.anchor-lang.com/docs/references/space)
- [Anchor Zero Copy](https://www.anchor-lang.com/docs/features/zero-copy)

## Exercise

Compute the size, in bytes, for this struct:

```rust
pub struct Profile {
    pub score: u64,
    pub name: String,
    pub tags: Vec<u8>,
}
```

Assume:

- `name` max length = 20 bytes
- `tags` max length = 10 elements

## Hint

Add:

- 8 bytes for the discriminator
- 8 for `u64`
- 4 + 20 for `String`
- 4 + 10 for `Vec<u8>`

