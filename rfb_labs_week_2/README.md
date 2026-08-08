# Rust for Bitcoin 2.0 — Week 2

Build a simplified Bitcoin transaction model while practising structs, enums,
traits, ownership, borrowing, collections, and `Result`-based error handling.

The crate is intentionally incomplete. Search for `TODO` and implement each part;
do not change the public type names or function signatures.

## Recommended workflow

1. Read [ASSIGNMENT.md](ASSIGNMENT.md).
2. Complete Parts 3–5 in `transaction.rs` and `error.rs`.
3. Remove `#[ignore]` from the relevant test and run it.
4. Complete the traits and borrowing functions in Parts 6–7.
5. Build the payment example in `main.rs`.
6. Complete UTXO selection and its tests.
7. Add the remaining required tests yourself.

```bash
cargo test
cargo test -- --ignored
cargo run
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
```

`cargo test` checks the starter project. Ignored tests intentionally exercise
unfinished code; enable them progressively rather than leaving them ignored in the
submission.

## Written answers

Answer in your own words. Add the ownership compiler error from Part 7 as a fenced
text block, then explain what caused it.

1. What is a Bitcoin transaction input?

   A bitcoin transaction input holds a reference to a UTXO in a senders wallet. A transaction can hold multiple inputs to be able to provide enough value for the recipient's payment, and transaction fees.

2. What is a Bitcoin transaction output?
3. What is a UTXO?
4. What does an outpoint identify?
5. How is a transaction fee calculated?
6. Why use integers rather than floating-point numbers for bitcoin amounts?
7. Why does `total_input_value()` borrow `self`?
8. Why does `add_input()` take `&mut self`?
9. What happens when an input is moved into a transaction?
10. Why is `Result` preferable to `panic!` for validation failures?
11. How do enums help model regular and coinbase inputs?
12. How does the `BitcoinValue` trait reduce duplication?

## Ownership and borrowing experiment

I initially iterated over the transaction outputs by value:

```rust
for txn in transaction.outputs {
    if txn.recipient == recipient {
        recipients_transactions.push(txn);
    }
}
```

This produced:

```text
error[E0308]: mismatched types
expected struct `Vec<&TxOutput>`
   found struct `Vec<TxOutput>`
```

Iterating by value made each `txn` an owned `TxOutput`, so Rust inferred the
result as `Vec<TxOutput>`. The function was required to return references borrowed
from the transaction. I fixed it by iterating over `&transaction.outputs`, making
each item an `&TxOutput` and allowing the returned references to remain tied to
the transaction's lifetime.

## Design notes

Describe any choices you made, including your UTXO-selection trade-offs and (if
attempted) the optional transaction-state extension.

## Example output

Paste the output of `cargo run` here once Part 8 is complete.

```text
Transaction(version=2, locktime=0, inputs=2, outputs=2, total input=120000 sats, total output=118000 sats, fee=2000 sats)
Transaction(version=2, locktime=0, inputs=2, outputs=2, total input=120000 sats, total output=118000 sats, fee=2000 sats)
```
