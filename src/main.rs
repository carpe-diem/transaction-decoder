use std::io::{Error as IoError, Read};
use std::error::Error;
use sha2::{Sha256, Digest};
use transaction::{Amount, Input, Output, Transaction, Txid};
mod transaction;

fn read_compact_size(transaction_bytes: &mut &[u8]) -> Result<u64, IoError> {
    let mut compact_size = [0_u8; 1];
    transaction_bytes.read(&mut compact_size)?;

    match compact_size[0] {
        0..=252 => Ok(compact_size[0] as u64),
        253 => {
            let mut buffer = [0; 2];
            transaction_bytes.read(&mut buffer)?;
            Ok(u16::from_le_bytes(buffer) as u64)
        },
        254 => {
            let mut buffer = [0; 4];
            transaction_bytes.read(&mut buffer)?;
            Ok(u32::from_le_bytes(buffer) as u64)
        },
        255 => {
            let mut buffer = [0; 8];
            transaction_bytes.read(&mut buffer)?;
            Ok(u64::from_le_bytes(buffer))
        },
    }
}

#[allow(unused)]
fn read_u32(transaction_bytes: &mut &[u8]) -> Result<u32, IoError> {
    let mut buffer = [0; 4];
    transaction_bytes.read(&mut buffer)?;
    Ok(u32::from_le_bytes(buffer))
}

fn read_amount(transaction_bytes: &mut &[u8]) -> Result<Amount, IoError> {
    let mut buffer = [0; 8];
    transaction_bytes.read(&mut buffer)?;
    Ok(Amount::from_sat(u64::from_le_bytes(buffer)))
}

fn read_txin(transaction_bytes: &mut &[u8]) -> Result<Txid, IoError> {
    let mut buffer = [0; 32];
    transaction_bytes.read(&mut buffer)?;
    Ok(Txid::from_bytes(buffer))
}

fn read_script(transaction_bytes: &mut &[u8]) -> Result<String, IoError> {
    let script_size = read_compact_size(transaction_bytes)? as usize;
    let mut buffer = vec![0; script_size as usize];
    transaction_bytes.read(&mut buffer)?;
    Ok(hex::encode(buffer))
}

fn hash_raw_transaction(transaction_bytes: &[u8]) -> Txid {
    let mut hasher = Sha256::new();
    hasher.update(transaction_bytes);
    let hash1 = hasher.finalize();

    let mut hasher = Sha256::new();
    hasher.update(hash1);
    let hash2 = hasher.finalize();
    
    Txid::from_bytes(hash2.into())

}

fn decode(transaction_hex: String) -> Result<String, Box<dyn Error>> {
    let transaction_bytes = hex::decode(transaction_hex).map_err(|e| format!("Hex decode error: {}", e))?;

    let mut bytes_slice = transaction_bytes.as_slice();
    let version = read_u32(&mut bytes_slice)?;
    let input_length = read_compact_size(&mut bytes_slice)?;
    let mut inputs = Vec::new();

    for _ in 0..input_length {
        let txin = read_txin(&mut bytes_slice)?;
        let output_index = read_u32(&mut bytes_slice)?;
        let script_sig = read_script(&mut bytes_slice)?;
        let sequence = read_u32(&mut bytes_slice)?;

        inputs.push(Input {
            txin,
            output_index,
            script_sig,
            sequence,
        });
    }

    let output_count = read_compact_size(&mut bytes_slice)?;
    let mut outputs = Vec::new();

    for _ in 0..output_count {
        let amount = read_amount(&mut bytes_slice)?;
        let script_pubkey = read_script(&mut bytes_slice)?;
        outputs.push(Output {
            amount,
            script_pubkey,
        });
    }

    let lock_time = read_u32(&mut bytes_slice)?;
    let transaction_id = hash_raw_transaction(&transaction_bytes);

    let transaction = Transaction {
        version,
        inputs,
        outputs,
        lock_time,
        transaction_id,
    };
    Ok(serde_json::to_string_pretty(&transaction)?)

}

fn main() { 
    let transaction_hex = "010000000242d5c1d6f7308bbe95c0f6e1301dd73a8da77d2155b0773bc297ac47f9cd7380010000006a4730440220771361aae55e84496b9e7b06e0a53dd122a1425f85840af7a52b20fa329816070220221dd92132e82ef9c133cb1a106b64893892a11acf2cfa1adb7698dcdc02f01b0121030077be25dc482e7f4abad60115416881fe4ef98af33c924cd8b20ca4e57e8bd5feffffff75c87cc5f3150eefc1c04c0246e7e0b370e64b17d6226c44b333a6f4ca14b49c000000006b483045022100e0d85fece671d367c8d442a96230954cdda4b9cf95e9edc763616d05d93e944302202330d520408d909575c5f6976cc405b3042673b601f4f2140b2e4d447e671c47012103c43afccd37aae7107f5a43f5b7b223d034e7583b77c8cd1084d86895a7341abffeffffff02ebb10f00000000001976a9144ef88a0b04e3ad6d1888da4be260d6735e0d308488ac508c1e000000000017a91476c0c8f2fc403c5edaea365f6a284317b9cdf7258700000000";
    
    match decode(transaction_hex.to_string()) {
        Ok(json) => println!("Transaction: {}", json),
        Err(e) => eprintln!("Error: {}", e),
    }
}


#[cfg(test)]
mod tests {
    use super::read_compact_size;
    use super::Error;

    #[test]
    fn test_read_compact_size_count_1() -> Result<(), Box<dyn Error>> {
        let mut bytes = [1_u8].as_slice();
        let count = read_compact_size(&mut bytes)?;
        assert_eq!(count, 1u64);
        Ok(())
    }

    #[test]
    fn test_read_compact_size_count_253() -> Result<(), Box<dyn Error>> {
        let mut bytes = [253_u8, 0, 1].as_slice();
        let count = read_compact_size(&mut bytes)?;
        assert_eq!(count, 256u64);
        Ok(())
    }

    #[test]
    fn test_read_compact_size_count_254() -> Result<(), Box<dyn Error>> {
        let mut bytes = [254_u8, 0, 0, 0, 1].as_slice();
        let count = read_compact_size(&mut bytes)?;
        assert_eq!(count, 256_u64.pow(3));
        Ok(())
    }

    #[test]
    fn test_read_compact_size_count_255() -> Result<(), Box<dyn Error>> {
        let mut bytes = [255_u8, 0, 0, 0, 0, 0, 0, 0, 1].as_slice();
        let count = read_compact_size(&mut bytes)?;
        assert_eq!(count, 256_u64.pow(7));
        Ok(())
    }

    #[test]
    fn test_read_compact_size_count_20000() -> Result<(), Box<dyn Error>> {
        let hex = "fd204e";
        let decoded = hex::decode(hex)?;
        let mut bytes = decoded.as_slice();
        let count = read_compact_size(&mut bytes)?;
        let expected_count = 20_000_u64;
        assert_eq!(count, expected_count);
        assert_eq!(bytes.len(), 0);
        Ok(())
    }
}