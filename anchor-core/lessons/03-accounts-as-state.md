# 03 Accounts as State

## Analogy

An account is not a function. It is stored state.

Think of it as:

- a locker with an address
- a data blob inside it
- an owner who is allowed to change it

## Official facts

Anchor account types help ensure the account provided by the client matches what the program expects.
`Account<'info, T>` checks ownership on deserialization.
`Signer<'info>` checks the transaction signer.

- [Anchor Account Types](https://www.anchor-lang.com/docs/references/account-types)

## Exercise

Classify these as “state”, “authority”, or “program boundary”:

1. `Account<'info, T>`
2. `Signer<'info>`
3. `SystemAccount<'info>`
4. `UncheckedAccount<'info>`

## Hint

If the type checks ownership or signature, it is guarding the boundary.

