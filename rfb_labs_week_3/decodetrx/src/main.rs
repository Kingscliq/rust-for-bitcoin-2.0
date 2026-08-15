use std::println;

use clap::Parser;
use decodetrx::decode_transaction;
use decodetrx::read_u64;

/// Decode a raw Bitcoin transaction supplied as hexadecimal text.
#[derive(Debug, Parser)]
#[command(name = "decodetrx", version, about)]
struct Cli {
    /// Complete raw transaction hex
    transaction_hex: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let cli = Cli::parse();


    let raw = "02000000000101af032baef1838ce072e0541a29a700127edf80298c4136e99371ac4a6f2231ec0000000000fdffffff02813e0000000000001600145598af355466d36e6a671374abbb2b75ef4779560000000000000000156a5d1214011400ff7f818cec82d08bc0a88281d21502473044022007ecae7dfd412adebece20f97dba8fbcd9bc1c495ef2213604a320dd5371f587022058d218c56d5acf4a0e39a53eda36e1d8b2b9967c8a4bb1cde6e0f7c1f2a6cc090121037b72b3c3f3d7e4107fa7a08ecffd74dc16ee332abe6eafabd89acea8bdc66f4200000000";

    let transaction_bytes = hex::decode(raw)?;

    let mut cursor = transaction_bytes.as_slice();
let final_value = read_u64(&mut cursor)?;

println!("{:?}", final_value);
    
    // let decoded_transaction = decode_transaction(cli.transaction_hex)?;

    // println!("{decoded_transaction}");
    Ok(())
}

// // https://mempool.space/testnet/tx/3c1804567a336c3944e30b3c2593970bfcbf5b15a40f4fc6b626a360ee0507f2
