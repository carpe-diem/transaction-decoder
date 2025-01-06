use serde::{Serialize, Serializer};


#[derive(Debug, Serialize)]
pub struct Transaction {
    pub transaction_id: Txid,
    pub version: u32,
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
    pub lock_time: u32,
}

#[derive(Debug)]
pub struct Txid([u8; 32]);

impl Txid {
    pub fn from_bytes(bytes: [u8; 32]) -> Txid {
        Txid(bytes)
    }
}

impl Serialize for Txid {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut bytes = self.0.clone();
        bytes.reverse();
        s.serialize_str(&hex::encode(bytes))
    }
}

#[derive(Debug, Serialize)]
pub struct Input {
    pub txin: Txid,
    pub output_index: u32,
    pub script_sig: String,
    pub sequence: u32,
}

#[derive(Debug)]
pub struct Amount (u64);

impl Amount {
    pub fn from_sat(satoshis: u64) -> Amount {
        Amount(satoshis)
    }
}

pub trait BitcoinValue {
    fn to_btc(&self) -> f64;
}

impl BitcoinValue for Amount {
    fn to_btc(&self) -> f64 {
        self.0 as f64 / 100_000_000.0
    }
}

#[derive(Debug, Serialize)]
pub struct Output {
    #[serde(serialize_with = "as_btc")]
    pub amount: Amount,
    pub script_pubkey: String,
}

fn as_btc<S: Serializer, T: BitcoinValue>(t: &T, s: S) -> Result<S::Ok, S::Error> {
    let btc = t.to_btc();
    s.serialize_f64(btc)
}

// this is the correct way to implement Debug for Input 
// using #[derive(Debug)] we get the same output as the test
// impl fmt::Debug for Input {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         f.debug_struct("Input")
//             .field("txid", &self.txin)
//             .field("output_index", &self.output_index)
//             .field("script", &self.script_sig)
//             .field("sequence", &self.sequence)
//             .finish()
//     }
// }
