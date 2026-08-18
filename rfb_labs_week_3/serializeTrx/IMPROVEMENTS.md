# Transaction Serializer Improvement Plan

## Goal

Extend the transaction serializer from a hard-coded demonstration into a reusable command-line program that accepts structured transaction data, validates it, serializes it according to Bitcoin's transaction format, and returns raw transaction hex.

The intended flow is:

```text
CLI arguments or JSON file
            ↓
   transaction input model
            ↓
        validation
            ↓
 internal Transaction types
            ↓
 Bitcoin byte serialization
            ↓
    raw transaction hex
```

## 1. Create a Complete Cargo Project

Add a `Cargo.toml` and separate the program into focused modules:

```text
serializeTrx/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── transaction.rs
│   └── error.rs
└── tests/
```

Responsibilities:

- `main.rs`: parse command-line arguments and display the result.
- `lib.rs`: validation and serialization logic.
- `transaction.rs`: transaction, input, output, amount, and TXID types.
- `error.rs`: custom serializer errors.
- `tests/`: unit and integration tests.

Suggested dependencies:

- `clap` for command-line parsing.
- `serde` and `serde_json` for JSON transaction input.
- `hex` for converting between hexadecimal text and bytes.
- `thiserror` for readable custom errors.
- `sha2` for TXID and WTXID calculation.

## 2. Accept Transaction Data Through Clap

Support two ways of supplying transaction values.

### JSON file

```bash
cargo run -- --file transaction.json
```

This is the more convenient option for transactions containing multiple inputs, outputs, and witness items.

### Direct command-line values

```bash
cargo run -- \
  --version 2 \
  --segwit \
  --locktime 0 \
  --input '{"prev_txid":"8fb0...","vout":1,"script_sig":"","sequence":4294967295,"witness":["3045...","029c..."]}' \
  --output '{"value":69886,"script_pubkey":"0014a632..."}' \
  --output '{"value":29442,"script_pubkey":"00149831..."}'
```

The `--input` and `--output` arguments should be repeatable so the user can provide multiple inputs and outputs.

The JSON-file and direct-argument paths must both create the same internal `Transaction` value. There should be only one validation and serialization implementation.

## 3. Add JSON Input Types

Derive `serde::Deserialize` for the types used to read transaction data.

Human-facing types may contain hexadecimal strings, while the internal transaction types should contain validated bytes. This separation prevents invalid hexadecimal values from reaching the serializer.

Example JSON:

```json
{
  "version": 2,
  "segwit": true,
  "inputs": [
    {
      "prev_txid": "8fb0...",
      "vout": 1,
      "script_sig": "",
      "sequence": 4294967295,
      "witness": ["3045...", "029c..."]
    }
  ],
  "outputs": [
    {
      "value": 69886,
      "script_pubkey": "0014a632..."
    }
  ],
  "locktime": 0
}
```

## 4. Add Custom Error Handling

Replace `Box<dyn Error>`, plain string errors, and `unwrap()` with a `SerializeError` enum created using `thiserror`.

The error variants should cover cases such as:

- Invalid hexadecimal input.
- Incorrect TXID length.
- Missing inputs.
- Missing outputs.
- Invalid output amount.
- Witness data supplied for a legacy transaction.
- Script or witness length that cannot be represented safely.
- Integer overflow while calculating total values.
- File-reading failure.
- Invalid JSON.

The CLI should catch the error and print its readable `Display` message to standard error before returning a failure exit code.

## 5. Validate Before Serializing

Before producing transaction bytes, validate that:

- The transaction contains at least one input.
- The transaction contains at least one output.
- Every previous TXID represents exactly 32 bytes.
- Every hexadecimal field is valid.
- Output amounts are within Bitcoin's valid monetary range.
- Adding output values does not overflow.
- Legacy transactions do not contain witness data.
- Script and witness lengths can be encoded safely.
- The chosen transaction format is consistent with its witness data.

Serialization success means more than accepting an even-length hex string. The supplied structured values must also form a coherent transaction representation.

## 6. Define TXID Byte-Order Behaviour

The CLI and JSON interface should accept previous TXIDs in the normal display order used by Bitcoin Core and block explorers.

Before writing a previous TXID into an input, the serializer should:

1. Decode the 64 hexadecimal characters into 32 bytes.
2. Validate the byte length.
3. Reverse the bytes into Bitcoin's serialized wire order.
4. Append them to the serialized input.

This rule should be documented so users do not need to supply an already-reversed TXID.

## 7. Refactor the Serialization Logic

Split the current large `serialize_transaction()` function into smaller responsibilities:

```text
encode_compact_size
serialize_input
serialize_output
serialize_witnesses
serialize_transaction
```

The top-level order must remain visible:

```text
Version
Marker and flag for SegWit
Input count
Inputs
Output count
Outputs
Witnesses for SegWit
Locktime
```

Each helper appends its bytes in Bitcoin's required order and returns errors instead of panicking.

## 8. Use the `hex` Crate

Replace the manual `hex_to_bytes()` and `bytes_to_hex()` implementations with:

```rust
hex::decode(value)
hex::encode(bytes)
```

The manual functions are useful for learning, but the crate provides reusable conversion logic and structured errors.

## 9. Improve Program Output

By default, print the raw transaction hex so it can be copied or piped into another command.

Optional output could include:

- Serialized transaction bytes.
- Transaction size in bytes.
- Transaction format: legacy or SegWit.
- TXID.
- WTXID.
- Output file support.

Possible CLI flags:

```text
--show-bytes
--show-size
--show-txid
--output <PATH>
```

## 10. Calculate TXID and WTXID

After serialization, calculate the transaction identifiers using double SHA-256.

- A legacy TXID hashes the complete legacy serialization.
- A SegWit TXID excludes the marker, flag, and witness data.
- A WTXID hashes the complete SegWit serialization, including witness data.
- For a legacy transaction, the TXID and WTXID are identical.

The serializer should display the hashes in the conventional reversed byte order used by Bitcoin Core and block explorers.

## 11. Add Round-Trip Verification

Use the decoder to verify the serializer:

```text
Transaction values
       ↓ serialize
raw transaction hex
       ↓ decode
decoded transaction values
```

Compare the important fields:

- Version.
- Inputs and outpoints.
- Outputs and amounts.
- Scripts.
- Witnesses.
- Locktime.

This confirms that the decoder and serializer agree on the transaction byte format.

## 12. Add Tests

Add tests for:

- Legacy transaction serialization.
- SegWit transaction serialization.
- Known transaction values producing the expected raw transaction hex.
- CompactSize boundary values: `252`, `253`, `65,535`, and `65,536`.
- Multiple inputs and outputs.
- Invalid hexadecimal fields.
- Incorrect TXID length.
- Missing inputs or outputs.
- Witness data in a legacy transaction.
- Amount and total-output overflow.
- TXID and WTXID calculation.
- JSON-file input.
- Direct command-line input.
- Readable user-facing error messages.
- Serializer-to-decoder round trips.

## Implementation Order

1. Create the Cargo project and module structure.
2. Add `clap`, `serde`, `serde_json`, `hex`, `thiserror`, and `sha2`.
3. Define the CLI and support JSON-file and direct command-line input.
4. Define human-facing and internal transaction types.
5. Add custom errors and transaction validation.
6. Refactor the existing serializer into smaller helpers.
7. Produce raw transaction hex.
8. Calculate TXID and WTXID.
9. Add unit, integration, CLI, and round-trip tests.

