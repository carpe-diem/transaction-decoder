use clap::Parser;
// use clap::{Arg, Command};


#[derive(Parser)]
#[command(name = "Transaction Decoder")]
#[command(version = "0.1.0")]
#[command(about = "Bitcoin transaction decoder")]
struct Cli {
    #[arg(
        required = true,
        help = "(string, required) Raw transaction hex"
    )]
    transaction_hex: String,
}



fn main() { 
    // let transaction_hex = "010000000242d5c1d6f7308bbe95c0f6e1301dd73a8da77d2155b0773bc297ac47f9cd7380010000006a4730440220771361aae55e84496b9e7b06e0a53dd122a1425f85840af7a52b20fa329816070220221dd92132e82ef9c133cb1a106b64893892a11acf2cfa1adb7698dcdc02f01b0121030077be25dc482e7f4abad60115416881fe4ef98af33c924cd8b20ca4e57e8bd5feffffff75c87cc5f3150eefc1c04c0246e7e0b370e64b17d6226c44b333a6f4ca14b49c000000006b483045022100e0d85fece671d367c8d442a96230954cdda4b9cf95e9edc763616d05d93e944302202330d520408d909575c5f6976cc405b3042673b601f4f2140b2e4d447e671c47012103c43afccd37aae7107f5a43f5b7b223d034e7583b77c8cd1084d86895a7341abffeffffff02ebb10f00000000001976a9144ef88a0b04e3ad6d1888da4be260d6735e0d308488ac508c1e000000000017a91476c0c8f2fc403c5edaea365f6a284317b9cdf7258700000000";
    let cli = Cli::parse();

    match transaction_decoder::decode(cli.transaction_hex) {
        Ok(json) => println!("Transaction: {}", json),
        Err(e) => eprintln!("Error: {}", e),
    }
    // Use the builder method instead of the derive attributes method.
    // let matches = Command::new("Transaction Decoder")
    // .version("1.0")
    // .about("Bitcoin Transaction Decoder")
    // .arg(Arg::new("transaction_hex")
    //         .required(true)
    //         .help("(string, required) Raw transaction hex"))
    // .get_matches();

    // let transaction_hex = matches
    //     .get_one::<String>("transaction_hex")
    //     .expect("required")
    //     .to_string();

    // match decode(transaction_hex) {
    //     Ok(json) => println!("{}", json),
    //     Err(e) => eprintln!("{}", e)
    // }
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