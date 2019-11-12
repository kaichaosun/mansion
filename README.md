# substrate-real-estate-node

A new SRML-based Substrate node, ready for hacking.

## Tasks
DPM: Department of Property Management

* [x] Owner - create a property record
* [x] DPM - authenticate the new created property
* [x] Owner - put a property in market
* [x] Buyer - make an offer and lock a property in market
* [x] DPM - authenticate the purchase
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

## FAQ

Q: What's the private key of Alice?
A: The key seed of prebuild accout Alice is `//Alice`, and bob's seed is `//Bob`, [source code](https://github.com/paritytech/substrate/blob/master/core/cli/src/lib.rs#L727).

Q: Why system::Trait do not need to import?
A: The use of `system::Trait` is a full path to system crate, no need to import with `use` keyword.

Q: Can all the user run a validator?
A: For `dev` network, Alice is the only one and default validator. For `local` network, Alice and Bob are the available validators. If you are playing on `kusama` network with Polkadot, you need enough ksm to be selected as a validator.

Q: What's the serialised json format for Option<Balance>?

Q: `cargo build` is slow, how to improve it?
A: If it's related to network, try use a mirrored crates registry. If it's hardware related, invest on yourself with a powerful machine. :) 

