# 07 Errors and Validation

## Analogy

`require!` is the bouncer at the door.
`#[error_code]` is the labeled failure drawer.

## Official facts

Anchor programs return a custom `Result<T>` type.
Custom errors start at code 6000, and `require!` is the concise way to reject bad inputs.

- [Anchor Custom Errors](https://www.anchor-lang.com/docs/features/errors)

## Exercise

Write one `#[error_code]` enum for:

- missing signature
- wrong owner
- too much data

Then write one `require!` check for each.

## Hint

Make the error names describe the invariant that failed.

