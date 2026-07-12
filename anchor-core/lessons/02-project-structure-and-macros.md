# 02 Project Structure and Macros

## Analogy

Anchor macros are forms that tell the compiler and runtime what to check.

- `declare_id!` sets the program identity
- `#[program]` marks instruction handlers
- `#[derive(Accounts)]` defines required accounts
- `#[account]` defines on-chain state layout

## Official facts

Anchor says these are the main macros in a program, and that the framework uses Rust macros to reduce boilerplate and simplify common security checks.

- [Anchor Program Structure](https://www.anchor-lang.com/docs/basics/program-structure)

## Exercise

Take this pseudo-program:

```rust
declare_id!("11111111111111111111111111111111");
#[program]
mod demo {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, data: u64) -> Result<()> {
        ctx.accounts.new_account.data = data;
        Ok(())
    }
}
```

Write, in words:

1. what the program ID is
2. what the instruction handler is
3. what the `Accounts` struct is doing

## Hint

The macros are not magic. They are declarations of intent plus generated boilerplate.

