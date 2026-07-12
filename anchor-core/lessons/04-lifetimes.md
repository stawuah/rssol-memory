# 04 Lifetimes

## Analogy

A lifetime is a promise that a borrowed reference will stay valid long enough.

If a reference is a library loan card, the lifetime is the due date rules.

## Official facts

Rust lifetimes ensure references are valid as long as needed, and they prevent dangling references.
Anchor uses lifetimes in forms like `Context<'info, T>` and `Account<'info, T>`.

- [Rust Lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)
- [Anchor Program Structure](https://www.anchor-lang.com/docs/basics/program-structure)
- [Anchor Account Types](https://www.anchor-lang.com/docs/references/account-types)

## Exercise

Answer:

1. Why does Rust need lifetimes for references?
2. In Anchor, what does `'info` represent in the account context?
3. Why are these references safer than raw pointers?

## Hint

Lifetimes describe relationships. They do not extend object life.

