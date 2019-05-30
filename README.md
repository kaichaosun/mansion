# substrate-real-estate-node

A new SRML-based Substrate node, ready for hacking.

## Tasks
DPM: Department of Property Management

* Owner - create a property record
* DPM - authenticate the new created property
* Owner - put a property in market
* Buyer - buy a property in market
* Owner - confirm the trade for the purchase
* DPM - authenticate the purchase
* Block - finalize the purchase

## Run
TODO

## JSON serialization
```json
{
  "Property": {
    "id": "H256",
    "size": "u64",
    "certificate_no": "u64",
    "price": "Balance"
  }
}
```

## Questions
* What's the private key of Alice
* Why system::Trait do not need to import
* All the user can run a validator?
* Option<Balance> -> Balance in the serialization json format