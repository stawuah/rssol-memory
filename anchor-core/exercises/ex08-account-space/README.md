# ex08 — Account serialization space

## Rust idea

Calculate encoded size from fixed fields, length prefixes, and maximum contents.

## Solana model

Anchor account data normally begins with an 8-byte discriminator. A `String` or
`Vec` uses a 4-byte length plus contents. Allocation must cover future maxima.

## How to solve it

1. Start with 8 discriminator bytes.
2. Add 8 bytes for the score.
3. Add `4 + name_max` for the string.
4. Add `4 + tags_max` for the byte vector.
5. Test zero sizes and a realistic maximum.

Production code should bound dynamic fields and consider checked size arithmetic.
