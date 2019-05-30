# substrate-real-estate-node

A new SRML-based Substrate node, ready for hacking.

## Tasks
DPM: Department of Property Management

* [x] Owner - create a property record
* [x] DPM - authenticate the new created property
* [x] Owner - put a property in market
* [x] Buyer - make an offer and lock a property in market
* [ ] DPM - authenticate the purchase
* [ ] Buyer/Owner - cancel the offer, unlock the property (nice to have)
* [ ] Owner - confirm the lock (nice to have)

## Run
```shell
./init.sh

./build.sh

cargo build --release

./target/release/template-node purge-chain --dev

./target/release/template-node --dev
```

## JSON serialization for property data struct
```json
{
  "Property": {
    "id": "H256",
    "size": "u64",
    "certificate_no": "u64",
    "is_authenticated": "bool"
  }
}
```

## Questions
* what's the private key of Alice
* why system::Trait do not need to import
* all the user can run a validator?
* option<Balance> -> Balance in the serialization json format
* `cargo build` is slow, how to improve it
* new created account is not allowed to execute extrinsics.
* purge can't delete created account.
