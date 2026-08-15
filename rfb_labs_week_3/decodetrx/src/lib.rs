use std::io::{Error, Read};

use error::DecodeError;
use sha2::{Digest, Sha256};
use transaction::{Amount, Input, Output, Transaction, Txid};

pub mod error;
mod transaction;

#[allow(unused_variables)]
fn read_version(transaction_hex: &str) -> u32 {
    todo!("read the transaction version from hexadecimal text")
}

pub fn read_u64(transaction_bytes: &mut &[u8]) -> Result<u64, DecodeError> {
  let mut buffer = [0_u8; 8];
  transaction_bytes.read_exact(&mut buffer)?; // []
  Ok(u64::from_le_bytes(buffer))
}

fn read_amount(transaction_bytes: &mut &[u8]) -> Result<Amount, Error> {
    todo!("read a transaction amount")
}

pub fn read_u32(bytes_slice: &mut &[u8]) -> Result<u32, DecodeError> {
    let mut buffer = [0_u8; 4];
    bytes_slice.read_exact(&mut buffer)?;
    Ok(u32::from_le_bytes(buffer))
}

fn read_compact_size(transaction_bytes: &mut &[u8]) -> Result<u64, Error> {
    todo!("read a CompactSize integer")
}

fn read_txid(transaction_bytes: &mut &[u8]) -> Result<Txid, Error> {
    todo!("read a transaction ID")
}

fn read_script_size(transaction_bytes: &mut &[u8]) -> Result<String, Error> {
    todo!("read a length-prefixed script")
}

fn read_version_byte(transaction_bytes: &mut &[u8]) -> Result<u32, Error> {
    todo!("read the version from decoded bytes")
}
// Bitcoin uses little-endian encoding for most of its numeric fields, meaning the least significant byte comes first.

fn hash_row_transaction(row_transaction_bytes: &[u8]) -> Result<Txid, Error> {
    todo!("hash the raw transaction")
}

pub fn decode_transaction(transaction_hex: String) -> Result<String, Box<dyn std::error::Error>> {
    todo!("decode the complete transaction")
}
