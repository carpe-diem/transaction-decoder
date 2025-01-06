# Bitcoin Transaction Decoder



```
$ cargo run

Transaction: {
  "transaction_id": "3c1804567a336c3944e30b3c2593970bfcbf5b15a40f4fc6b626a360ee0507f2",
  "version": 1,
  "inputs": [
    {
      "txin": "8073cdf947ac97c23b77b055217da78d3ad71d30e1f6c095be8b30f7d6c1d542",
      "output_index": 1,
      "script_sig": "4730440220771361aae55e84496b9e7b06e0a53dd122a1425f85840af7a52b20fa329816070220221dd92132e82ef9c133cb1a106b64893892a11acf2cfa1adb7698dcdc02f01b0121030077be25dc482e7f4abad60115416881fe4ef98af33c924cd8b20ca4e57e8bd5",
      "sequence": 4294967294
    },
    {
      "txin": "9cb414caf4a633b3446c22d6174be670b3e0e746024cc0c1ef0e15f3c57cc875",
      "output_index": 0,
      "script_sig": "483045022100e0d85fece671d367c8d442a96230954cdda4b9cf95e9edc763616d05d93e944302202330d520408d909575c5f6976cc405b3042673b601f4f2140b2e4d447e671c47012103c43afccd37aae7107f5a43f5b7b223d034e7583b77c8cd1084d86895a7341abf",
      "sequence": 4294967294
    }
  ],
  "outputs": [
    {
      "amount": 0.01028587,
      "script_pubkey": "76a9144ef88a0b04e3ad6d1888da4be260d6735e0d308488ac"
    },
    {
      "amount": 0.02002,
      "script_pubkey": "a91476c0c8f2fc403c5edaea365f6a284317b9cdf72587"
    }
  ],
  "lock_time": 0
}
 ~/src/training-code/transaction-decoder  main !1 ?2  cargo run                                                                                                   ✔ 
   Compiling transaction-decoder v0.1.0 (/Users/albertopaparelli/src/training-code/transaction-decoder)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.76s
     Running `target/debug/transaction-decoder`
Error: Hex decode error: Odd number of digits
 ~/src/training-code/transaction-decoder  main !1 ?2  cargo run                                                                                                   ✔ 
   Compiling transaction-decoder v0.1.0 (/Users/albertopaparelli/src/training-code/transaction-decoder)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.19s
     Running `target/debug/transaction-decoder`
Transaction: {
  "transaction_id": "3c1804567a336c3944e30b3c2593970bfcbf5b15a40f4fc6b626a360ee0507f2",
  "version": 1,
  "inputs": [
    {
      "txin": "8073cdf947ac97c23b77b055217da78d3ad71d30e1f6c095be8b30f7d6c1d542",
      "output_index": 1,
      "script_sig": "4730440220771361aae55e84496b9e7b06e0a53dd122a1425f85840af7a52b20fa329816070220221dd92132e82ef9c133cb1a106b64893892a11acf2cfa1adb7698dcdc02f01b0121030077be25dc482e7f4abad60115416881fe4ef98af33c924cd8b20ca4e57e8bd5",
      "sequence": 4294967294
    },
    {
      "txin": "9cb414caf4a633b3446c22d6174be670b3e0e746024cc0c1ef0e15f3c57cc875",
      "output_index": 0,
      "script_sig": "483045022100e0d85fece671d367c8d442a96230954cdda4b9cf95e9edc763616d05d93e944302202330d520408d909575c5f6976cc405b3042673b601f4f2140b2e4d447e671c47012103c43afccd37aae7107f5a43f5b7b223d034e7583b77c8cd1084d86895a7341abf",
      "sequence": 4294967294
    }
  ],
  "outputs": [
    {
      "amount": 0.01028587,
      "script_pubkey": "76a9144ef88a0b04e3ad6d1888da4be260d6735e0d308488ac"
    },
    {
      "amount": 0.02002,
      "script_pubkey": "a91476c0c8f2fc403c5edaea365f6a284317b9cdf72587"
    }
  ],
  "lock_time": 0
}
```

```
$ cargo test
running 5 tests
test tests::test_read_compact_size_count_1 ... ok
test tests::test_read_compact_size_count_20000 ... ok
test tests::test_read_compact_size_count_255 ... ok
test tests::test_read_compact_size_count_254 ... ok
test tests::test_read_compact_size_count_253 ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```