#![feature(prelude_import)]
#![no_std]
//! The Substrate Node Template runtime. This can be compiled with `#[no_std]`, ready for Wasm.
#![recursion_limit = "256"]

#[prelude_import]
use ::std::prelude::v1::*;

#[macro_use]
extern crate std as std;

pub use balances::Call as BalancesCall;
use client::{
    block_builder::api::{self as block_builder_api, CheckInherentsResult, InherentData},
    impl_runtime_apis, runtime_api,
};
pub use consensus::Call as ConsensusCall;
use parity_codec::{Decode, Encode};
#[cfg(feature = "std")]
use primitives::bytes;
use primitives::{ed25519, sr25519, OpaqueMetadata};
use rstd::prelude::*;
#[cfg(any(feature = "std", test))]
pub use runtime_primitives::BuildStorage;
use runtime_primitives::{
    create_runtime_str, generic,
    traits::{self, BlakeTwo256, Block as BlockT, NumberFor, StaticLookup, Verify},
    transaction_validity::TransactionValidity,
    ApplyResult,
};
pub use runtime_primitives::{Perbill, Permill};
#[cfg(feature = "std")]
use serde_derive::{Deserialize, Serialize};
pub use support::{construct_runtime, StorageValue};
pub use timestamp::BlockPeriod;
pub use timestamp::Call as TimestampCall;
#[cfg(feature = "std")]
use version::NativeVersion;
use version::RuntimeVersion;

/// The type that is used for identifying authorities.
pub type AuthorityId = <AuthoritySignature as Verify>::Signer;
/// The type used by authorities to prove their ID.
pub type AuthoritySignature = ed25519::Signature;
/// Alias to pubkey that identifies an account on the chain.
pub type AccountId = <AccountSignature as Verify>::Signer;
/// The type used by authorities to prove their ID.
pub type AccountSignature = sr25519::Signature;
/// A hash of some data used by the chain.
pub type Hash = primitives::H256;
/// Index of a block number in the chain.
pub type BlockNumber = u64;
/// Index of an account's extrinsic in the chain.
pub type Nonce = u64;

/// Used for the module template in `./template.rs`
mod template {
    /// A runtime module template with necessary imports
    /// Feel free to remove or edit this file as needed.
    /// If you change the name of this file, make sure to update its references in runtime/src/lib.rs
    /// If you remove this file, you can remove those references
    /// For more guidance on Substrate modules, see the example module
    /// https://github.com/paritytech/substrate/blob/master/srml/example/src/lib.rs
    use support::{decl_event, decl_module, decl_storage, dispatch::Result, StorageValue};
    use system::ensure_signed;

    /// The module's configuration trait.
    pub trait Trait: system::Trait {
        /// The overarching event type.
        type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
    }

    #[doc(hidden)]
    mod sr_api_hidden_includes_decl_storage {
        pub extern crate support as hidden_include;
    }

    struct Something<T: Trait>(
        self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::PhantomData<
            (T),
        >,
    );

    impl<T: Trait>
    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageValue<
        u32,
    > for Something<T>
    {
        type Query = Option<u32>;
        /// Get the storage key.
        fn key() -> &'static [u8] {
            "TemplateModule Something".as_bytes()
        }
        /// Load the value from the provided storage instance.
        fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::GenericStorage>(
            storage: &S,
        ) -> Self::Query {
            storage.get(<Self as self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageValue<u32>>::key()).or_else(|| Default::default())
        }
        /// Take a value from storage, removing it afterwards.
        fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::GenericStorage>(
            storage: &S,
        ) -> Self::Query {
            storage.take(<Self as self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageValue<u32>>::key()).or_else(|| Default::default())
        }
        /// Mutate the value under a key.
        fn mutate<
            R,
            F: FnOnce(&mut Self::Query) -> R,
            S: self::sr_api_hidden_includes_decl_storage::hidden_include::GenericStorage,
        >(
            f: F,
            storage: &S,
        ) -> R {
            let mut val = <Self as self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageValue<u32>>::get(storage);
            let ret = f(&mut val);
            match val {
                Some(ref val) => <Self as self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageValue<u32>>::put(&val, storage),
                None => <Self as self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageValue<u32>>::kill(storage),
            };
            ret
        }
    }

    trait Store {
        type Something;
    }

    #[doc(hidden)]
    pub struct __GetByteStructSomething<T>(
        pub  self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<
            (T),
        >,
    );

    #[cfg(feature = "std")]
    #[allow(non_upper_case_globals)]
    static __CACHE_GET_BYTE_STRUCT_Something:
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<
        self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>,
    > =
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;

    #[cfg(feature = "std")]
    impl<T: Trait>
    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DefaultByte
    for __GetByteStructSomething<T>
    {
        fn default_byte(
            &self,
        ) -> self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
            __CACHE_GET_BYTE_STRUCT_Something
                .get_or_init(|| {
                    let def_val: Option<u32> = Default::default();
                    <Option<u32> as Encode>::encode(&def_val)
                })
                .clone()
        }
    }

    impl<T: Trait> Store for Module<T> {
        type Something = Something<T>;
    }

    impl<T: 'static + Trait> Module<T> {
        pub fn something() -> Option<u32> {
            <Something<T> as self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageValue<u32>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
        }
        #[doc(hidden)]
        pub fn store_metadata() -> self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMetadata {
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMetadata {
                functions: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode({
                    &[self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageFunctionMetadata {
                        name: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode("Something"),
                        modifier: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageFunctionModifier::Optional,
                        ty: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode("u32")),
                        default: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DefaultByteGetter(&__GetByteStructSomething::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                        documentation: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode(&[]),
                    }]
                }),
            }
        }
        #[doc(hidden)]
        pub fn store_metadata_functions() -> &'static [self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageFunctionMetadata] {
            {
                &[self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageFunctionMetadata {
                    name: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode("Something"),
                    modifier: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageFunctionModifier::Optional,
                    ty: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode("u32")),
                    default: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DefaultByteGetter(&__GetByteStructSomething::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                    documentation: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode(&[]),
                }]
            }
        }
        #[doc(hidden)]
        pub fn store_metadata_name() -> &'static str {
            "TemplateModule"
        }
    }

    #[cfg(feature = "std")]
    #[structural_match]
    #[rustc_copy_clone_marker]
    pub struct Module<T: Trait>(::std::marker::PhantomData<(T)>);

    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<T: ::std::clone::Clone + Trait> ::std::clone::Clone for Module<T> {
        #[inline]
        fn clone(&self) -> Module<T> {
            match *self {
                Module(ref __self_0_0) => Module(::std::clone::Clone::clone(&(*__self_0_0))),
            }
        }
    }

    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<T: ::std::marker::Copy + Trait> ::std::marker::Copy for Module<T> {}

    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<T: ::std::cmp::PartialEq + Trait> ::std::cmp::PartialEq for Module<T> {
        #[inline]
        fn eq(&self, other: &Module<T>) -> bool {
            match *other {
                Module(ref __self_1_0) => match *self {
                    Module(ref __self_0_0) => (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &Module<T>) -> bool {
            match *other {
                Module(ref __self_1_0) => match *self {
                    Module(ref __self_0_0) => (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }

    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<T: ::std::cmp::Eq + Trait> ::std::cmp::Eq for Module<T> {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::std::cmp::AssertParamIsEq<::std::marker::PhantomData<(T)>>;
            }
        }
    }

    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<T: ::std::fmt::Debug + Trait> ::std::fmt::Debug for Module<T> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                Module(ref __self_0_0) => {
                    let mut debug_trait_builder = f.debug_tuple("Module");
                    let _ = debug_trait_builder.field(&&(*__self_0_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }

    impl<T: Trait> ::srml_support::runtime_primitives::traits::OnInitialize<T::BlockNumber>
    for Module<T>
    {}

    impl<T: Trait> ::srml_support::runtime_primitives::traits::OnFinalize<T::BlockNumber>
    for Module<T>
    {}

    impl<T: Trait> ::srml_support::runtime_primitives::traits::OffchainWorker<T::BlockNumber>
    for Module<T>
    {}

    impl<T: Trait> Module<T> {
        fn deposit_event(event: Event<T>) {
            <system::Module<T>>::deposit_event(<T as Trait>::from(event).into());
        }
    }

    /// Can also be called using [`Call`].
    ///
    /// [`Call`]: enum.Call.html
    impl<T: Trait> Module<T> {
        pub fn do_something(origin: T::Origin, something: u32) -> Result {
            let who = ensure_signed(origin)?;
            <Something<T>>::put(something);
            Self::deposit_event(RawEvent::SomethingStored(something, who));
            Ok(())
        }
    }

    #[cfg(feature = "std")]
    /// The module declaration.
    pub enum Call<T: Trait> {
        #[doc(hidden)]
        __PhantomItem(
            ::std::marker::PhantomData<(T)>,
            ::srml_support::dispatch::Never,
        ),
        #[allow(non_camel_case_types)]
        do_something(u32),
    }

    impl<T: Trait> ::srml_support::dispatch::Clone for Call<T> {
        fn clone(&self) -> Self {
            match *self {
                Call::do_something(ref something) => Call::do_something((*something).clone()),
                _ => ::std::rt::begin_panic(
                    "internal error: entered unreachable code",
                    &("src/template.rs", 32u32, 1u32),
                ),
            }
        }
    }

    impl<T: Trait> ::srml_support::dispatch::PartialEq for Call<T> {
        fn eq(&self, _other: &Self) -> bool {
            match *self {
                Call::do_something(ref something) => {
                    let self_params = (something, );
                    if let Call::do_something(ref something) = *_other {
                        self_params == (something, )
                    } else {
                        match *_other {
                            Call::__PhantomItem(_, _) => ::std::rt::begin_panic(
                                "internal error: entered unreachable code",
                                &("src/template.rs", 32u32, 1u32),
                            ),
                            _ => false,
                        }
                    }
                }
                _ => ::std::rt::begin_panic(
                    "internal error: entered unreachable code",
                    &("src/template.rs", 32u32, 1u32),
                ),
            }
        }
    }

    impl<T: Trait> ::srml_support::dispatch::Eq for Call<T> {}

    #[cfg(feature = "std")]
    impl<T: Trait> ::srml_support::dispatch::fmt::Debug for Call<T> {
        fn fmt(
            &self,
            _f: &mut ::srml_support::dispatch::fmt::Formatter,
        ) -> ::srml_support::dispatch::result::Result<(), ::srml_support::dispatch::fmt::Error>
        {
            match *self {
                Call::do_something(ref something) => _f.write_fmt(::std::fmt::Arguments::new_v1(
                    &["", ""],
                    &match (&"do_something", &(something.clone(), )) {
                        (arg0, arg1) => [
                            ::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Display::fmt),
                            ::std::fmt::ArgumentV1::new(arg1, ::std::fmt::Debug::fmt),
                        ],
                    },
                )),
                _ => ::std::rt::begin_panic(
                    "internal error: entered unreachable code",
                    &("src/template.rs", 32u32, 1u32),
                ),
            }
        }
    }

    impl<T: Trait> ::srml_support::dispatch::Decode for Call<T> {
        fn decode<Input: ::srml_support::dispatch::Input>(input: &mut Input) -> Option<Self> {
            let _input_id = input.read_byte()?;
            {
                if _input_id == (0) {
                    let something = ::srml_support::dispatch::Decode::decode(input)?;
                    return Some(Call::do_something(something));
                }
                None
            }
        }
    }

    impl<T: Trait> ::srml_support::dispatch::Encode for Call<T> {
        fn encode_to<W: ::srml_support::dispatch::Output>(&self, _dest: &mut W) {
            {
                if let Call::do_something(ref something) = *self {
                    _dest.push_byte((0) as u8);
                    something.encode_to(_dest);
                }
                {}
            };
        }
    }

    impl<T: Trait> ::srml_support::dispatch::Dispatchable for Call<T> {
        type Trait = T;
        type Origin = T::Origin;
        fn dispatch(self, _origin: Self::Origin) -> ::srml_support::dispatch::Result {
            match self {
                Call::do_something(something) => <Module<T>>::do_something(_origin, something),
                Call::__PhantomItem(_, _) => ::std::rt::begin_panic_fmt(
                    &::std::fmt::Arguments::new_v1(
                        &["internal error: entered unreachable code: "],
                        &match (&"__PhantomItem should never be used.", ) {
                            (arg0, ) => {
                                [::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Display::fmt)]
                            }
                        },
                    ),
                    &("src/template.rs", 32u32, 1u32),
                ),
            }
        }
    }

    impl<T: Trait> ::srml_support::dispatch::Callable for Module<T> {
        type Call = Call<T>;
    }

    impl<T: Trait> Module<T> {
        #[doc(hidden)]
        pub fn dispatch<D: ::srml_support::dispatch::Dispatchable<Trait=T>>(
            d: D,
            origin: D::Origin,
        ) -> ::srml_support::dispatch::Result {
            d.dispatch(origin)
        }
    }

    impl<T: Trait> Module<T> {
        #[doc(hidden)]
        pub fn call_functions() -> &'static [::srml_support::dispatch::FunctionMetadata] {
            &[::srml_support::dispatch::FunctionMetadata {
                name: ::srml_support::dispatch::DecodeDifferent::Encode("do_something"),
                arguments: ::srml_support::dispatch::DecodeDifferent::Encode(&[
                    ::srml_support::dispatch::FunctionArgumentMetadata {
                        name: ::srml_support::dispatch::DecodeDifferent::Encode("something"),
                        ty: ::srml_support::dispatch::DecodeDifferent::Encode("u32"),
                    },
                ]),
                documentation: ::srml_support::dispatch::DecodeDifferent::Encode(&[]),
            }]
        }
    }

    /// [`RawEvent`] specialized for the configuration [`Trait`]
    ///
    /// [`RawEvent`]: enum.RawEvent.html
    /// [`Trait`]: trait.Trait.html
    pub type Event<T> = RawEvent<<T as system::Trait>::AccountId>;

    /// Events for this module.
    ///
    #[structural_match]
    pub enum RawEvent<AccountId> {
        SomethingStored(u32, AccountId),
    }

    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<AccountId: ::std::clone::Clone> ::std::clone::Clone for RawEvent<AccountId> {
        #[inline]
        fn clone(&self) -> RawEvent<AccountId> {
            match (&*self, ) {
                (&RawEvent::SomethingStored(ref __self_0, ref __self_1), ) => {
                    RawEvent::SomethingStored(
                        ::std::clone::Clone::clone(&(*__self_0)),
                        ::std::clone::Clone::clone(&(*__self_1)),
                    )
                }
            }
        }
    }

    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<AccountId: ::std::cmp::PartialEq> ::std::cmp::PartialEq for RawEvent<AccountId> {
        #[inline]
        fn eq(&self, other: &RawEvent<AccountId>) -> bool {
            match (&*self, &*other) {
                (
                    &RawEvent::SomethingStored(ref __self_0, ref __self_1),
                    &RawEvent::SomethingStored(ref __arg_1_0, ref __arg_1_1),
                ) => (*__self_0) == (*__arg_1_0) && (*__self_1) == (*__arg_1_1),
            }
        }
        #[inline]
        fn ne(&self, other: &RawEvent<AccountId>) -> bool {
            match (&*self, &*other) {
                (
                    &RawEvent::SomethingStored(ref __self_0, ref __self_1),
                    &RawEvent::SomethingStored(ref __arg_1_0, ref __arg_1_1),
                ) => (*__self_0) != (*__arg_1_0) || (*__self_1) != (*__arg_1_1),
            }
        }
    }

    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<AccountId: ::std::cmp::Eq> ::std::cmp::Eq for RawEvent<AccountId> {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::std::cmp::AssertParamIsEq<u32>;
                let _: ::std::cmp::AssertParamIsEq<AccountId>;
            }
        }
    }

    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_ENCODE_FOR_RawEvent: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl<AccountId> _parity_codec::Encode for RawEvent<AccountId>
            where
                AccountId: _parity_codec::Encode,
                AccountId: _parity_codec::Encode,
        {
            fn encode_to<EncOut: _parity_codec::Output>(&self, dest: &mut EncOut) {
                match *self {
                    RawEvent::SomethingStored(ref aa, ref ba) => {
                        dest.push_byte(0usize as u8);
                        dest.push(aa);
                        dest.push(ba);
                    }
                    _ => (),
                }
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DECODE_FOR_RawEvent: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl<AccountId> _parity_codec::Decode for RawEvent<AccountId>
            where
                AccountId: _parity_codec::Decode,
                AccountId: _parity_codec::Decode,
        {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn) -> Option<Self> {
                match input.read_byte()? {
                    x if x == 0usize as u8 => Some(RawEvent::SomethingStored(
                        _parity_codec::Decode::decode(input)?,
                        _parity_codec::Decode::decode(input)?,
                    )),
                    _ => None,
                }
            }
        }
    };

    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<AccountId: ::std::fmt::Debug> ::std::fmt::Debug for RawEvent<AccountId> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match (&*self, ) {
                (&RawEvent::SomethingStored(ref __self_0, ref __self_1), ) => {
                    let mut debug_trait_builder = f.debug_tuple("SomethingStored");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    let _ = debug_trait_builder.field(&&(*__self_1));
                    debug_trait_builder.finish()
                }
            }
        }
    }

    impl<AccountId> From<RawEvent<AccountId>> for () {
        fn from(_: RawEvent<AccountId>) -> () {
            ()
        }
    }

    impl<AccountId> RawEvent<AccountId> {
        #[allow(dead_code)]
        pub fn metadata() -> &'static [::srml_support::event::EventMetadata] {
            &[::srml_support::event::EventMetadata {
                name: ::srml_support::event::DecodeDifferent::Encode("SomethingStored"),
                arguments: ::srml_support::event::DecodeDifferent::Encode(&["u32", "AccountId"]),
                documentation: ::srml_support::event::DecodeDifferent::Encode(&[]),
            }]
        }
    }
}

mod realestate {
    use parity_codec::{Decode, Encode};
    use runtime_primitives::traits::Hash;
    /// Realestate runtime module
    use support::{
        decl_event, decl_module, decl_storage, dispatch::Result, ensure, traits::Currency,
        traits::ReservableCurrency, StorageMap, StorageValue,
    };
    use system::{ensure_root, ensure_signed};

    /// The module's configuration trait.
    pub trait Trait: balances::Trait {
        /// The overarching event type.
        type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
    }

    pub struct Property<Hash> {
        id: Hash,
        size: u64,
        certificate_no: u64,
        is_authenticated: bool,
    }

    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_ENCODE_FOR_Property: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl<Hash> _parity_codec::Encode for Property<Hash>
            where
                Hash: _parity_codec::Encode,
                Hash: _parity_codec::Encode,
        {
            fn encode_to<EncOut: _parity_codec::Output>(&self, dest: &mut EncOut) {
                dest.push(&self.id);
                dest.push(&self.size);
                dest.push(&self.certificate_no);
                dest.push(&self.is_authenticated);
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DECODE_FOR_Property: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl<Hash> _parity_codec::Decode for Property<Hash>
            where
                Hash: _parity_codec::Decode,
                Hash: _parity_codec::Decode,
        {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn) -> Option<Self> {
                Some(Property {
                    id: _parity_codec::Decode::decode(input)?,
                    size: _parity_codec::Decode::decode(input)?,
                    certificate_no: _parity_codec::Decode::decode(input)?,
                    is_authenticated: _parity_codec::Decode::decode(input)?,
                })
            }
        }
    };

    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<Hash: ::std::default::Default> ::std::default::Default for Property<Hash> {
        #[inline]
        fn default() -> Property<Hash> {
            Property {
                id: ::std::default::Default::default(),
                size: ::std::default::Default::default(),
                certificate_no: ::std::default::Default::default(),
                is_authenticated: ::std::default::Default::default(),
            }
        }
    }

    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<Hash: ::std::clone::Clone> ::std::clone::Clone for Property<Hash> {
        #[inline]
        fn clone(&self) -> Property<Hash> {
            match *self {
                Property {
                    id: ref __self_0_0,
                    size: ref __self_0_1,
                    certificate_no: ref __self_0_2,
                    is_authenticated: ref __self_0_3,
                } => Property {
                    id: ::std::clone::Clone::clone(&(*__self_0_0)),
                    size: ::std::clone::Clone::clone(&(*__self_0_1)),
                    certificate_no: ::std::clone::Clone::clone(&(*__self_0_2)),
                    is_authenticated: ::std::clone::Clone::clone(&(*__self_0_3)),
                },
            }
        }
    }

    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<Hash: ::std::cmp::PartialEq> ::std::cmp::PartialEq for Property<Hash> {
        #[inline]
        fn eq(&self, other: &Property<Hash>) -> bool {
            match *other {
                Property {
                    id: ref __self_1_0,
                    size: ref __self_1_1,
                    certificate_no: ref __self_1_2,
                    is_authenticated: ref __self_1_3,
                } => match *self {
                    Property {
                        id: ref __self_0_0,
                        size: ref __self_0_1,
                        certificate_no: ref __self_0_2,
                        is_authenticated: ref __self_0_3,
                    } => {
                        (*__self_0_0) == (*__self_1_0)
                            && (*__self_0_1) == (*__self_1_1)
                            && (*__self_0_2) == (*__self_1_2)
                            && (*__self_0_3) == (*__self_1_3)
                    }
                },
            }
        }
        #[inline]
        fn ne(&self, other: &Property<Hash>) -> bool {
            match *other {
                Property {
                    id: ref __self_1_0,
                    size: ref __self_1_1,
                    certificate_no: ref __self_1_2,
                    is_authenticated: ref __self_1_3,
                } => match *self {
                    Property {
                        id: ref __self_0_0,
                        size: ref __self_0_1,
                        certificate_no: ref __self_0_2,
                        is_authenticated: ref __self_0_3,
                    } => {
                        (*__self_0_0) != (*__self_1_0)
                            || (*__self_0_1) != (*__self_1_1)
                            || (*__self_0_2) != (*__self_1_2)
                            || (*__self_0_3) != (*__self_1_3)
                    }
                },
            }
        }
    }

    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<Hash: ::std::fmt::Debug> ::std::fmt::Debug for Property<Hash> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                Property {
                    id: ref __self_0_0,
                    size: ref __self_0_1,
                    certificate_no: ref __self_0_2,
                    is_authenticated: ref __self_0_3,
                } => {
                    let mut debug_trait_builder = f.debug_struct("Property");
                    let _ = debug_trait_builder.field("id", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("size", &&(*__self_0_1));
                    let _ = debug_trait_builder.field("certificate_no", &&(*__self_0_2));
                    let _ = debug_trait_builder.field("is_authenticated", &&(*__self_0_3));
                    debug_trait_builder.finish()
                }
            }
        }
    }

    #[doc(hidden)]
    mod sr_api_hidden_includes_decl_storage {
        pub extern crate support as hidden_include;
    }

    struct Something<T: Trait>(
        self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::PhantomData<
            (T),
        >,
    );

    impl<T: Trait>
    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageValue<
        u32,
    > for Something<T>
    {
        type Query = Option<u32>;
        /// Get the storage key.
        fn key() -> &'static [u8] {
            "RealEstateStorage Something".as_bytes()
        }
        /// Load the value from the provided storage instance.
        fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::GenericStorage>(
            storage: &S,
        ) -> Self::Query {
            storage.get(<Self as self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageValue<u32>>::key()).or_else(|| Default::default())
        }
        /// Take a value from storage, removing it afterwards.
        fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::GenericStorage>(
            storage: &S,
        ) -> Self::Query {
            storage.take(<Self as self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageValue<u32>>::key()).or_else(|| Default::default())
        }
        /// Mutate the value under a key.
        fn mutate<
            R,
            F: FnOnce(&mut Self::Query) -> R,
            S: self::sr_api_hidden_includes_decl_storage::hidden_include::GenericStorage,
        >(
            f: F,
            storage: &S,
        ) -> R {
            let mut val = <Self as self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageValue<u32>>::get(storage);
            let ret = f(&mut val);
            match val {
                Some(ref val) => <Self as self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageValue<u32>>::put(&val, storage),
                None => <Self as self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageValue<u32>>::kill(storage),
            };
            ret
        }
    }

    struct Nonce<T: Trait>(
        self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::PhantomData<
            (T),
        >,
    );

    impl<T: Trait>
    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageValue<
        u64,
    > for Nonce<T>
    {
        type Query = u64;
        /// Get the storage key.
        fn key() -> &'static [u8] {
            "RealEstateStorage Nonce".as_bytes()
        }
        /// Load the value from the provided storage instance.
        fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::GenericStorage>(
            storage: &S,
        ) -> Self::Query {
            storage.get(<Self as self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageValue<u64>>::key()).unwrap_or_else(|| Default::default())
        }
        /// Take a value from storage, removing it afterwards.
        fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::GenericStorage>(
            storage: &S,
        ) -> Self::Query {
            storage.take(<Self as self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageValue<u64>>::key()).unwrap_or_else(|| Default::default())
        }
        /// Mutate the value under a key.
        fn mutate<
            R,
            F: FnOnce(&mut Self::Query) -> R,
            S: self::sr_api_hidden_includes_decl_storage::hidden_include::GenericStorage,
        >(
            f: F,
            storage: &S,
        ) -> R {
            let mut val = <Self as self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageValue<u64>>::get(storage);
            let ret = f(&mut val);
            <Self as self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageValue<u64>>::put(&val, storage);
            ret
        }
    }

    struct Properties<T: Trait>(
        self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::PhantomData<
            (T),
        >,
    );

    impl<T: Trait>
    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMap<
        T::Hash,
        Property<T::Hash>,
    > for Properties<T>
    {
        type Query = Property<T::Hash>;
        /// Get the prefix key in storage.
        fn prefix() -> &'static [u8] {
            "RealEstateStorage Properties".as_bytes()
        }
        /// Get the storage key used to fetch a value corresponding to a specific key.
        fn key_for(
            x: &T::Hash,
        ) -> self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            let mut key = <Self as self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMap<T::Hash, Property<T::Hash>>>::prefix().to_vec();
            self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode::encode_to(
                x, &mut key,
            );
            key
        }
        /// Load the value associated with the given key from the map.
        fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::GenericStorage>(
            key: &T::Hash,
            storage: &S,
        ) -> Self::Query {
            let key = <Self as self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMap<T::Hash, Property<T::Hash>>>::key_for(key);
            storage.get(&key[..]).unwrap_or_else(|| Default::default())
        }
        /// Take the value, reading and removing it.
        fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::GenericStorage>(
            key: &T::Hash,
            storage: &S,
        ) -> Self::Query {
            let key = <Self as self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMap<T::Hash, Property<T::Hash>>>::key_for(key);
            storage.take(&key[..]).unwrap_or_else(|| Default::default())
        }
        /// Mutate the value under a key
        fn mutate<
            R,
            F: FnOnce(&mut Self::Query) -> R,
            S: self::sr_api_hidden_includes_decl_storage::hidden_include::GenericStorage,
        >(
            key: &T::Hash,
            f: F,
            storage: &S,
        ) -> R {
            let mut val = <Self as self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMap<T::Hash, Property<T::Hash>>>::get(key, storage);
            let ret = f(&mut val);
            <Self as self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMap<T::Hash, Property<T::Hash>>>::insert(key, &val, storage);
            ret
        }
    }

    struct AllPropertiesArray<T: Trait>(
        self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::PhantomData<
            (T),
        >,
    );

    impl<T: Trait>
    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMap<
        u64,
        T::Hash,
    > for AllPropertiesArray<T>
    {
        type Query = T::Hash;
        /// Get the prefix key in storage.
        fn prefix() -> &'static [u8] {
            "RealEstateStorage AllPropertiesArray".as_bytes()
        }
        /// Get the storage key used to fetch a value corresponding to a specific key.
        fn key_for(
            x: &u64,
        ) -> self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            let mut key = <Self as self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMap<u64, T::Hash>>::prefix().to_vec();
            self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode::encode_to(
                x, &mut key,
            );
            key
        }
        /// Load the value associated with the given key from the map.
        fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::GenericStorage>(
            key: &u64,
            storage: &S,
        ) -> Self::Query {
            let key = <Self as self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMap<u64, T::Hash>>::key_for(key);
            storage.get(&key[..]).unwrap_or_else(|| Default::default())
        }
        /// Take the value, reading and removing it.
        fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::GenericStorage>(
            key: &u64,
            storage: &S,
        ) -> Self::Query {
            let key = <Self as self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMap<u64, T::Hash>>::key_for(key);
            storage.take(&key[..]).unwrap_or_else(|| Default::default())
        }
        /// Mutate the value under a key
        fn mutate<
            R,
            F: FnOnce(&mut Self::Query) -> R,
            S: self::sr_api_hidden_includes_decl_storage::hidden_include::GenericStorage,
        >(
            key: &u64,
            f: F,
            storage: &S,
        ) -> R {
            let mut val = <Self as self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMap<u64, T::Hash>>::get(key, storage);
            let ret = f(&mut val);
            <Self as self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMap<u64, T::Hash>>::insert(key, &val, storage);
            ret
        }
    }

    struct PropertyOwner<T: Trait>(
        self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::PhantomData<
            (T),
        >,
    );

    impl<T: Trait>
    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMap<
        T::Hash,
        T::AccountId,
    > for PropertyOwner<T>
    {
        type Query = T::AccountId;
        /// Get the prefix key in storage.
        fn prefix() -> &'static [u8] {
            "RealEstateStorage PropertyOwner".as_bytes()
        }
        /// Get the storage key used to fetch a value corresponding to a specific key.
        fn key_for(
            x: &T::Hash,
        ) -> self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            let mut key = <Self as self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMap<T::Hash, T::AccountId>>::prefix().to_vec();
            self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode::encode_to(
                x, &mut key,
            );
            key
        }
        /// Load the value associated with the given key from the map.
        fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::GenericStorage>(
            key: &T::Hash,
            storage: &S,
        ) -> Self::Query {
            let key = <Self as self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMap<T::Hash, T::AccountId>>::key_for(key);
            storage.get(&key[..]).unwrap_or_else(|| Default::default())
        }
        /// Take the value, reading and removing it.
        fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::GenericStorage>(
            key: &T::Hash,
            storage: &S,
        ) -> Self::Query {
            let key = <Self as self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMap<T::Hash, T::AccountId>>::key_for(key);
            storage.take(&key[..]).unwrap_or_else(|| Default::default())
        }
        /// Mutate the value under a key
        fn mutate<
            R,
            F: FnOnce(&mut Self::Query) -> R,
            S: self::sr_api_hidden_includes_decl_storage::hidden_include::GenericStorage,
        >(
            key: &T::Hash,
            f: F,
            storage: &S,
        ) -> R {
            let mut val = <Self as self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMap<T::Hash, T::AccountId>>::get(key, storage);
            let ret = f(&mut val);
            <Self as self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMap<T::Hash, T::AccountId>>::insert(key, &val, storage);
            ret
        }
    }

    struct Managers<T: Trait>(
        self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::PhantomData<
            (T),
        >,
    );

    impl<T: Trait>
    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMap<
        u64,
        T::AccountId,
    > for Managers<T>
    {
        type Query = T::AccountId;
        /// Get the prefix key in storage.
        fn prefix() -> &'static [u8] {
            "RealEstateStorage Managers".as_bytes()
        }
        /// Get the storage key used to fetch a value corresponding to a specific key.
        fn key_for(
            x: &u64,
        ) -> self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            let mut key = <Self as self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMap<u64, T::AccountId>>::prefix().to_vec();
            self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode::encode_to(
                x, &mut key,
            );
            key
        }
        /// Load the value associated with the given key from the map.
        fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::GenericStorage>(
            key: &u64,
            storage: &S,
        ) -> Self::Query {
            let key = <Self as self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMap<u64, T::AccountId>>::key_for(key);
            storage.get(&key[..]).unwrap_or_else(|| Default::default())
        }
        /// Take the value, reading and removing it.
        fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::GenericStorage>(
            key: &u64,
            storage: &S,
        ) -> Self::Query {
            let key = <Self as self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMap<u64, T::AccountId>>::key_for(key);
            storage.take(&key[..]).unwrap_or_else(|| Default::default())
        }
        /// Mutate the value under a key
        fn mutate<
            R,
            F: FnOnce(&mut Self::Query) -> R,
            S: self::sr_api_hidden_includes_decl_storage::hidden_include::GenericStorage,
        >(
            key: &u64,
            f: F,
            storage: &S,
        ) -> R {
            let mut val = <Self as self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMap<u64, T::AccountId>>::get(key, storage);
            let ret = f(&mut val);
            <Self as self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMap<u64, T::AccountId>>::insert(key, &val, storage);
            ret
        }
    }

    struct ManagersIndex<T: Trait>(
        self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::PhantomData<
            (T),
        >,
    );

    impl<T: Trait>
    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMap<
        T::AccountId,
        u64,
    > for ManagersIndex<T>
    {
        type Query = u64;
        /// Get the prefix key in storage.
        fn prefix() -> &'static [u8] {
            "RealEstateStorage ManagersIndex".as_bytes()
        }
        /// Get the storage key used to fetch a value corresponding to a specific key.
        fn key_for(
            x: &T::AccountId,
        ) -> self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            let mut key = <Self as self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMap<T::AccountId, u64>>::prefix().to_vec();
            self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode::encode_to(
                x, &mut key,
            );
            key
        }
        /// Load the value associated with the given key from the map.
        fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::GenericStorage>(
            key: &T::AccountId,
            storage: &S,
        ) -> Self::Query {
            let key = <Self as self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMap<T::AccountId, u64>>::key_for(key);
            storage.get(&key[..]).unwrap_or_else(|| Default::default())
        }
        /// Take the value, reading and removing it.
        fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::GenericStorage>(
            key: &T::AccountId,
            storage: &S,
        ) -> Self::Query {
            let key = <Self as self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMap<T::AccountId, u64>>::key_for(key);
            storage.take(&key[..]).unwrap_or_else(|| Default::default())
        }
        /// Mutate the value under a key
        fn mutate<
            R,
            F: FnOnce(&mut Self::Query) -> R,
            S: self::sr_api_hidden_includes_decl_storage::hidden_include::GenericStorage,
        >(
            key: &T::AccountId,
            f: F,
            storage: &S,
        ) -> R {
            let mut val = <Self as self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMap<T::AccountId, u64>>::get(key, storage);
            let ret = f(&mut val);
            <Self as self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMap<T::AccountId, u64>>::insert(key, &val, storage);
            ret
        }
    }

    struct ManagerNonce<T: Trait>(
        self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::PhantomData<
            (T),
        >,
    );

    impl<T: Trait>
    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageValue<
        u64,
    > for ManagerNonce<T>
    {
        type Query = u64;
        /// Get the storage key.
        fn key() -> &'static [u8] {
            "RealEstateStorage ManagerNonce".as_bytes()
        }
        /// Load the value from the provided storage instance.
        fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::GenericStorage>(
            storage: &S,
        ) -> Self::Query {
            storage.get(<Self as self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageValue<u64>>::key()).unwrap_or_else(|| Default::default())
        }
        /// Take a value from storage, removing it afterwards.
        fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::GenericStorage>(
            storage: &S,
        ) -> Self::Query {
            storage.take(<Self as self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageValue<u64>>::key()).unwrap_or_else(|| Default::default())
        }
        /// Mutate the value under a key.
        fn mutate<
            R,
            F: FnOnce(&mut Self::Query) -> R,
            S: self::sr_api_hidden_includes_decl_storage::hidden_include::GenericStorage,
        >(
            f: F,
            storage: &S,
        ) -> R {
            let mut val = <Self as self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageValue<u64>>::get(storage);
            let ret = f(&mut val);
            <Self as self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageValue<u64>>::put(&val, storage);
            ret
        }
    }

    struct PropertiesForsale<T: Trait>(
        self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::PhantomData<
            (T),
        >,
    );

    impl<T: Trait>
    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMap<
        u64,
        (T::Hash, T::Balance, bool, Option<T::AccountId>),
    > for PropertiesForsale<T>
    {
        type Query = (T::Hash, T::Balance, bool, Option<T::AccountId>);
        /// Get the prefix key in storage.
        fn prefix() -> &'static [u8] {
            "RealEstateStorage PropertiesForsale".as_bytes()
        }
        /// Get the storage key used to fetch a value corresponding to a specific key.
        fn key_for(
            x: &u64,
        ) -> self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            let mut key = <Self as self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMap<u64, (T::Hash, T::Balance, bool, Option<T::AccountId>)>>::prefix().to_vec();
            self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode::encode_to(
                x, &mut key,
            );
            key
        }
        /// Load the value associated with the given key from the map.
        fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::GenericStorage>(
            key: &u64,
            storage: &S,
        ) -> Self::Query {
            let key = <Self as self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMap<u64, (T::Hash, T::Balance, bool, Option<T::AccountId>)>>::key_for(key);
            storage.get(&key[..]).unwrap_or_else(|| Default::default())
        }
        /// Take the value, reading and removing it.
        fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::GenericStorage>(
            key: &u64,
            storage: &S,
        ) -> Self::Query {
            let key = <Self as self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMap<u64, (T::Hash, T::Balance, bool, Option<T::AccountId>)>>::key_for(key);
            storage.take(&key[..]).unwrap_or_else(|| Default::default())
        }
        /// Mutate the value under a key
        fn mutate<
            R,
            F: FnOnce(&mut Self::Query) -> R,
            S: self::sr_api_hidden_includes_decl_storage::hidden_include::GenericStorage,
        >(
            key: &u64,
            f: F,
            storage: &S,
        ) -> R {
            let mut val = <Self as self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMap<u64, (T::Hash, T::Balance, bool, Option<T::AccountId>)>>::get(key, storage);
            let ret = f(&mut val);
            <Self as self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMap<u64, (T::Hash, T::Balance, bool, Option<T::AccountId>)>>::insert(key, &val, storage);
            ret
        }
    }

    struct PropertyForsaleCount<T: Trait>(
        self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::PhantomData<
            (T),
        >,
    );

    impl<T: Trait>
    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageValue<
        u64,
    > for PropertyForsaleCount<T>
    {
        type Query = u64;
        /// Get the storage key.
        fn key() -> &'static [u8] {
            "RealEstateStorage PropertyForsaleCount".as_bytes()
        }
        /// Load the value from the provided storage instance.
        fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::GenericStorage>(
            storage: &S,
        ) -> Self::Query {
            storage.get(<Self as self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageValue<u64>>::key()).unwrap_or_else(|| Default::default())
        }
        /// Take a value from storage, removing it afterwards.
        fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::GenericStorage>(
            storage: &S,
        ) -> Self::Query {
            storage.take(<Self as self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageValue<u64>>::key()).unwrap_or_else(|| Default::default())
        }
        /// Mutate the value under a key.
        fn mutate<
            R,
            F: FnOnce(&mut Self::Query) -> R,
            S: self::sr_api_hidden_includes_decl_storage::hidden_include::GenericStorage,
        >(
            f: F,
            storage: &S,
        ) -> R {
            let mut val = <Self as self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageValue<u64>>::get(storage);
            let ret = f(&mut val);
            <Self as self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageValue<u64>>::put(&val, storage);
            ret
        }
    }

    struct PropertyForsaleIndex<T: Trait>(
        self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::PhantomData<
            (T),
        >,
    );

    impl<T: Trait>
    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMap<
        T::Hash,
        u64,
    > for PropertyForsaleIndex<T>
    {
        type Query = u64;
        /// Get the prefix key in storage.
        fn prefix() -> &'static [u8] {
            "RealEstateStorage PropertyForsaleIndex".as_bytes()
        }
        /// Get the storage key used to fetch a value corresponding to a specific key.
        fn key_for(
            x: &T::Hash,
        ) -> self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            let mut key = <Self as self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMap<T::Hash, u64>>::prefix().to_vec();
            self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode::encode_to(
                x, &mut key,
            );
            key
        }
        /// Load the value associated with the given key from the map.
        fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::GenericStorage>(
            key: &T::Hash,
            storage: &S,
        ) -> Self::Query {
            let key = <Self as self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMap<T::Hash, u64>>::key_for(key);
            storage.get(&key[..]).unwrap_or_else(|| Default::default())
        }
        /// Take the value, reading and removing it.
        fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::GenericStorage>(
            key: &T::Hash,
            storage: &S,
        ) -> Self::Query {
            let key = <Self as self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMap<T::Hash, u64>>::key_for(key);
            storage.take(&key[..]).unwrap_or_else(|| Default::default())
        }
        /// Mutate the value under a key
        fn mutate<
            R,
            F: FnOnce(&mut Self::Query) -> R,
            S: self::sr_api_hidden_includes_decl_storage::hidden_include::GenericStorage,
        >(
            key: &T::Hash,
            f: F,
            storage: &S,
        ) -> R {
            let mut val = <Self as self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMap<T::Hash, u64>>::get(key, storage);
            let ret = f(&mut val);
            <Self as self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMap<T::Hash, u64>>::insert(key, &val, storage);
            ret
        }
    }

    trait Store {
        type Something;
        type Nonce;
        type Properties;
        type AllPropertiesArray;
        type PropertyOwner;
        type Managers;
        type ManagersIndex;
        type ManagerNonce;
        type PropertiesForsale;
        type PropertyForsaleCount;
        type PropertyForsaleIndex;
    }

    #[doc(hidden)]
    pub struct __GetByteStructSomething<T>(
        pub  self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<
            (T),
        >,
    );

    #[cfg(feature = "std")]
    #[allow(non_upper_case_globals)]
    static __CACHE_GET_BYTE_STRUCT_Something:
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<
        self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>,
    > =
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;

    #[cfg(feature = "std")]
    impl<T: Trait>
    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DefaultByte
    for __GetByteStructSomething<T>
    {
        fn default_byte(
            &self,
        ) -> self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
            __CACHE_GET_BYTE_STRUCT_Something
                .get_or_init(|| {
                    let def_val: Option<u32> = Default::default();
                    <Option<u32> as Encode>::encode(&def_val)
                })
                .clone()
        }
    }

    #[doc(hidden)]
    pub struct __GetByteStructNonce<T>(
        pub  self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<
            (T),
        >,
    );

    #[cfg(feature = "std")]
    #[allow(non_upper_case_globals)]
    static __CACHE_GET_BYTE_STRUCT_Nonce:
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<
        self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>,
    > =
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;

    #[cfg(feature = "std")]
    impl<T: Trait>
    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DefaultByte
    for __GetByteStructNonce<T>
    {
        fn default_byte(
            &self,
        ) -> self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
            __CACHE_GET_BYTE_STRUCT_Nonce
                .get_or_init(|| {
                    let def_val: u64 = Default::default();
                    <u64 as Encode>::encode(&def_val)
                })
                .clone()
        }
    }

    #[doc(hidden)]
    pub struct __GetByteStructProperties<T>(
        pub  self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<
            (T),
        >,
    );

    #[cfg(feature = "std")]
    #[allow(non_upper_case_globals)]
    static __CACHE_GET_BYTE_STRUCT_Properties:
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<
        self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>,
    > =
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;

    #[cfg(feature = "std")]
    impl<T: Trait>
    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DefaultByte
    for __GetByteStructProperties<T>
    {
        fn default_byte(
            &self,
        ) -> self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
            __CACHE_GET_BYTE_STRUCT_Properties
                .get_or_init(|| {
                    let def_val: Property<T::Hash> = Default::default();
                    <Property<T::Hash> as Encode>::encode(&def_val)
                })
                .clone()
        }
    }

    #[doc(hidden)]
    pub struct __GetByteStructAllPropertiesArray<T>(
        pub  self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<
            (T),
        >,
    );

    #[cfg(feature = "std")]
    #[allow(non_upper_case_globals)]
    static __CACHE_GET_BYTE_STRUCT_AllPropertiesArray:
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<
        self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>,
    > =
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;

    #[cfg(feature = "std")]
    impl<T: Trait>
    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DefaultByte
    for __GetByteStructAllPropertiesArray<T>
    {
        fn default_byte(
            &self,
        ) -> self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
            __CACHE_GET_BYTE_STRUCT_AllPropertiesArray
                .get_or_init(|| {
                    let def_val: T::Hash = Default::default();
                    <T::Hash as Encode>::encode(&def_val)
                })
                .clone()
        }
    }

    #[doc(hidden)]
    pub struct __GetByteStructPropertyOwner<T>(
        pub  self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<
            (T),
        >,
    );

    #[cfg(feature = "std")]
    #[allow(non_upper_case_globals)]
    static __CACHE_GET_BYTE_STRUCT_PropertyOwner:
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<
        self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>,
    > =
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;

    #[cfg(feature = "std")]
    impl<T: Trait>
    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DefaultByte
    for __GetByteStructPropertyOwner<T>
    {
        fn default_byte(
            &self,
        ) -> self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
            __CACHE_GET_BYTE_STRUCT_PropertyOwner
                .get_or_init(|| {
                    let def_val: T::AccountId = Default::default();
                    <T::AccountId as Encode>::encode(&def_val)
                })
                .clone()
        }
    }

    #[doc(hidden)]
    pub struct __GetByteStructManagers<T>(
        pub  self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<
            (T),
        >,
    );

    #[cfg(feature = "std")]
    #[allow(non_upper_case_globals)]
    static __CACHE_GET_BYTE_STRUCT_Managers:
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<
        self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>,
    > =
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;

    #[cfg(feature = "std")]
    impl<T: Trait>
    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DefaultByte
    for __GetByteStructManagers<T>
    {
        fn default_byte(
            &self,
        ) -> self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
            __CACHE_GET_BYTE_STRUCT_Managers
                .get_or_init(|| {
                    let def_val: T::AccountId = Default::default();
                    <T::AccountId as Encode>::encode(&def_val)
                })
                .clone()
        }
    }

    #[doc(hidden)]
    pub struct __GetByteStructManagersIndex<T>(
        pub  self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<
            (T),
        >,
    );

    #[cfg(feature = "std")]
    #[allow(non_upper_case_globals)]
    static __CACHE_GET_BYTE_STRUCT_ManagersIndex:
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<
        self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>,
    > =
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;

    #[cfg(feature = "std")]
    impl<T: Trait>
    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DefaultByte
    for __GetByteStructManagersIndex<T>
    {
        fn default_byte(
            &self,
        ) -> self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
            __CACHE_GET_BYTE_STRUCT_ManagersIndex
                .get_or_init(|| {
                    let def_val: u64 = Default::default();
                    <u64 as Encode>::encode(&def_val)
                })
                .clone()
        }
    }

    #[doc(hidden)]
    pub struct __GetByteStructManagerNonce<T>(
        pub  self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<
            (T),
        >,
    );

    #[cfg(feature = "std")]
    #[allow(non_upper_case_globals)]
    static __CACHE_GET_BYTE_STRUCT_ManagerNonce:
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<
        self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>,
    > =
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;

    #[cfg(feature = "std")]
    impl<T: Trait>
    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DefaultByte
    for __GetByteStructManagerNonce<T>
    {
        fn default_byte(
            &self,
        ) -> self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
            __CACHE_GET_BYTE_STRUCT_ManagerNonce
                .get_or_init(|| {
                    let def_val: u64 = Default::default();
                    <u64 as Encode>::encode(&def_val)
                })
                .clone()
        }
    }

    #[doc(hidden)]
    pub struct __GetByteStructPropertiesForsale<T>(
        pub  self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<
            (T),
        >,
    );

    #[cfg(feature = "std")]
    #[allow(non_upper_case_globals)]
    static __CACHE_GET_BYTE_STRUCT_PropertiesForsale:
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<
        self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>,
    > =
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;

    #[cfg(feature = "std")]
    impl<T: Trait>
    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DefaultByte
    for __GetByteStructPropertiesForsale<T>
    {
        fn default_byte(
            &self,
        ) -> self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
            __CACHE_GET_BYTE_STRUCT_PropertiesForsale
                .get_or_init(|| {
                    let def_val: (T::Hash, T::Balance, bool, Option<T::AccountId>) =
                        Default::default();
                    <(T::Hash, T::Balance, bool, Option<T::AccountId>) as Encode>::encode(&def_val)
                })
                .clone()
        }
    }

    #[doc(hidden)]
    pub struct __GetByteStructPropertyForsaleCount<T>(
        pub  self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<
            (T),
        >,
    );

    #[cfg(feature = "std")]
    #[allow(non_upper_case_globals)]
    static __CACHE_GET_BYTE_STRUCT_PropertyForsaleCount:
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<
        self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>,
    > =
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;

    #[cfg(feature = "std")]
    impl<T: Trait>
    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DefaultByte
    for __GetByteStructPropertyForsaleCount<T>
    {
        fn default_byte(
            &self,
        ) -> self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
            __CACHE_GET_BYTE_STRUCT_PropertyForsaleCount
                .get_or_init(|| {
                    let def_val: u64 = Default::default();
                    <u64 as Encode>::encode(&def_val)
                })
                .clone()
        }
    }

    #[doc(hidden)]
    pub struct __GetByteStructPropertyForsaleIndex<T>(
        pub  self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<
            (T),
        >,
    );

    #[cfg(feature = "std")]
    #[allow(non_upper_case_globals)]
    static __CACHE_GET_BYTE_STRUCT_PropertyForsaleIndex:
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<
        self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>,
    > =
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;

    #[cfg(feature = "std")]
    impl<T: Trait>
    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DefaultByte
    for __GetByteStructPropertyForsaleIndex<T>
    {
        fn default_byte(
            &self,
        ) -> self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
            __CACHE_GET_BYTE_STRUCT_PropertyForsaleIndex
                .get_or_init(|| {
                    let def_val: u64 = Default::default();
                    <u64 as Encode>::encode(&def_val)
                })
                .clone()
        }
    }

    impl<T: Trait> Store for Module<T> {
        type Something = Something<T>;
        type Nonce = Nonce<T>;
        type Properties = Properties<T>;
        type AllPropertiesArray = AllPropertiesArray<T>;
        type PropertyOwner = PropertyOwner<T>;
        type Managers = Managers<T>;
        type ManagersIndex = ManagersIndex<T>;
        type ManagerNonce = ManagerNonce<T>;
        type PropertiesForsale = PropertiesForsale<T>;
        type PropertyForsaleCount = PropertyForsaleCount<T>;
        type PropertyForsaleIndex = PropertyForsaleIndex<T>;
    }

    impl<T: 'static + Trait> Module<T> {
        pub fn something() -> Option<u32> {
            <Something<T> as self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageValue<u32>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
        }
        pub fn property<
            K: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::Borrow<
                T::Hash,
            >,
        >(
            key: K,
        ) -> Property<T::Hash> {
            <Properties<T> as self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMap<T::Hash, Property<T::Hash>>>::get(key.borrow(), &self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
        }
        pub fn property_owner<
            K: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::Borrow<
                T::Hash,
            >,
        >(
            key: K,
        ) -> T::AccountId {
            <PropertyOwner<T> as self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMap<T::Hash, T::AccountId>>::get(key.borrow(), &self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
        }
        pub fn manager<
            K: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::Borrow<
                u64,
            >,
        >(
            key: K,
        ) -> T::AccountId {
            <Managers<T> as self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMap<u64, T::AccountId>>::get(key.borrow(), &self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
        }
        pub fn forsale_property<
            K: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::Borrow<
                u64,
            >,
        >(
            key: K,
        ) -> (T::Hash, T::Balance, bool, Option<T::AccountId>) {
            <PropertiesForsale<T> as self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMap<u64, (T::Hash, T::Balance, bool, Option<T::AccountId>)>>::get(key.borrow(), &self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
        }
        pub fn property_forsale_count() -> u64 {
            <PropertyForsaleCount<T> as self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageValue<u64>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
        }
        #[doc(hidden)]
        pub fn store_metadata() -> self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMetadata {
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMetadata {
                functions: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode({
                    &[self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageFunctionMetadata {
                        name: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode("Something"),
                        modifier: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageFunctionModifier::Optional,
                        ty: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode("u32")),
                        default: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DefaultByteGetter(&__GetByteStructSomething::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                        documentation: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode(&[]),
                    }, self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageFunctionMetadata {
                        name: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode("Nonce"),
                        modifier: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageFunctionModifier::Default,
                        ty: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode("u64")),
                        default: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DefaultByteGetter(&__GetByteStructNonce::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                        documentation: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode(&[]),
                    }, self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageFunctionMetadata {
                        name: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode("Properties"),
                        modifier: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageFunctionModifier::Default,
                        ty: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageFunctionType::Map { key: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode("T::Hash"), value: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode("Property<T::Hash>"), is_linked: false },
                        default: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DefaultByteGetter(&__GetByteStructProperties::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                        documentation: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode(&[]),
                    }, self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageFunctionMetadata {
                        name: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode("AllPropertiesArray"),
                        modifier: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageFunctionModifier::Default,
                        ty: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageFunctionType::Map { key: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode("u64"), value: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode("T::Hash"), is_linked: false },
                        default: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DefaultByteGetter(&__GetByteStructAllPropertiesArray::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                        documentation: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode(&[]),
                    }, self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageFunctionMetadata {
                        name: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode("PropertyOwner"),
                        modifier: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageFunctionModifier::Default,
                        ty: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageFunctionType::Map { key: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode("T::Hash"), value: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode("T::AccountId"), is_linked: false },
                        default: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DefaultByteGetter(&__GetByteStructPropertyOwner::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                        documentation: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode(&[]),
                    }, self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageFunctionMetadata {
                        name: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode("Managers"),
                        modifier: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageFunctionModifier::Default,
                        ty: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageFunctionType::Map { key: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode("u64"), value: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode("T::AccountId"), is_linked: false },
                        default: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DefaultByteGetter(&__GetByteStructManagers::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                        documentation: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode(&[]),
                    }, self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageFunctionMetadata {
                        name: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode("ManagersIndex"),
                        modifier: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageFunctionModifier::Default,
                        ty: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageFunctionType::Map { key: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode("T::AccountId"), value: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode("u64"), is_linked: false },
                        default: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DefaultByteGetter(&__GetByteStructManagersIndex::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                        documentation: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode(&[]),
                    }, self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageFunctionMetadata {
                        name: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode("ManagerNonce"),
                        modifier: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageFunctionModifier::Default,
                        ty: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode("u64")),
                        default: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DefaultByteGetter(&__GetByteStructManagerNonce::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                        documentation: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode(&[]),
                    }, self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageFunctionMetadata {
                        name: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode("PropertiesForsale"),
                        modifier: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageFunctionModifier::Default,
                        ty: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageFunctionType::Map { key: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode("u64"), value: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode("(T::Hash, T::Balance, bool, Option<T::AccountId>)"), is_linked: false },
                        default: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DefaultByteGetter(&__GetByteStructPropertiesForsale::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                        documentation: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode(&[]),
                    }, self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageFunctionMetadata {
                        name: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode("PropertyForsaleCount"),
                        modifier: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageFunctionModifier::Default,
                        ty: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode("u64")),
                        default: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DefaultByteGetter(&__GetByteStructPropertyForsaleCount::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                        documentation: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode(&[]),
                    }, self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageFunctionMetadata {
                        name: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode("PropertyForsaleIndex"),
                        modifier: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageFunctionModifier::Default,
                        ty: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageFunctionType::Map { key: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode("T::Hash"), value: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode("u64"), is_linked: false },
                        default: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DefaultByteGetter(&__GetByteStructPropertyForsaleIndex::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                        documentation: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode(&[]),
                    }]
                }),
            }
        }
        #[doc(hidden)]
        pub fn store_metadata_functions() -> &'static [self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageFunctionMetadata] {
            {
                &[self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageFunctionMetadata {
                    name: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode("Something"),
                    modifier: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageFunctionModifier::Optional,
                    ty: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode("u32")),
                    default: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DefaultByteGetter(&__GetByteStructSomething::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                    documentation: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode(&[]),
                }, self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageFunctionMetadata {
                    name: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode("Nonce"),
                    modifier: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageFunctionModifier::Default,
                    ty: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode("u64")),
                    default: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DefaultByteGetter(&__GetByteStructNonce::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                    documentation: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode(&[]),
                }, self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageFunctionMetadata {
                    name: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode("Properties"),
                    modifier: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageFunctionModifier::Default,
                    ty: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageFunctionType::Map { key: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode("T::Hash"), value: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode("Property<T::Hash>"), is_linked: false },
                    default: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DefaultByteGetter(&__GetByteStructProperties::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                    documentation: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode(&[]),
                }, self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageFunctionMetadata {
                    name: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode("AllPropertiesArray"),
                    modifier: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageFunctionModifier::Default,
                    ty: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageFunctionType::Map { key: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode("u64"), value: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode("T::Hash"), is_linked: false },
                    default: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DefaultByteGetter(&__GetByteStructAllPropertiesArray::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                    documentation: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode(&[]),
                }, self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageFunctionMetadata {
                    name: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode("PropertyOwner"),
                    modifier: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageFunctionModifier::Default,
                    ty: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageFunctionType::Map { key: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode("T::Hash"), value: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode("T::AccountId"), is_linked: false },
                    default: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DefaultByteGetter(&__GetByteStructPropertyOwner::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                    documentation: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode(&[]),
                }, self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageFunctionMetadata {
                    name: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode("Managers"),
                    modifier: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageFunctionModifier::Default,
                    ty: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageFunctionType::Map { key: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode("u64"), value: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode("T::AccountId"), is_linked: false },
                    default: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DefaultByteGetter(&__GetByteStructManagers::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                    documentation: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode(&[]),
                }, self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageFunctionMetadata {
                    name: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode("ManagersIndex"),
                    modifier: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageFunctionModifier::Default,
                    ty: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageFunctionType::Map { key: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode("T::AccountId"), value: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode("u64"), is_linked: false },
                    default: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DefaultByteGetter(&__GetByteStructManagersIndex::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                    documentation: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode(&[]),
                }, self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageFunctionMetadata {
                    name: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode("ManagerNonce"),
                    modifier: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageFunctionModifier::Default,
                    ty: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode("u64")),
                    default: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DefaultByteGetter(&__GetByteStructManagerNonce::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                    documentation: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode(&[]),
                }, self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageFunctionMetadata {
                    name: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode("PropertiesForsale"),
                    modifier: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageFunctionModifier::Default,
                    ty: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageFunctionType::Map { key: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode("u64"), value: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode("(T::Hash, T::Balance, bool, Option<T::AccountId>)"), is_linked: false },
                    default: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DefaultByteGetter(&__GetByteStructPropertiesForsale::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                    documentation: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode(&[]),
                }, self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageFunctionMetadata {
                    name: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode("PropertyForsaleCount"),
                    modifier: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageFunctionModifier::Default,
                    ty: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode("u64")),
                    default: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DefaultByteGetter(&__GetByteStructPropertyForsaleCount::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                    documentation: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode(&[]),
                }, self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageFunctionMetadata {
                    name: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode("PropertyForsaleIndex"),
                    modifier: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageFunctionModifier::Default,
                    ty: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageFunctionType::Map { key: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode("T::Hash"), value: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode("u64"), is_linked: false },
                    default: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DefaultByteGetter(&__GetByteStructPropertyForsaleIndex::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                    documentation: self::sr_api_hidden_includes_decl_storage::hidden_include::storage::generator::DecodeDifferent::Encode(&[]),
                }]
            }
        }
        #[doc(hidden)]
        pub fn store_metadata_name() -> &'static str {
            "RealEstateStorage"
        }
    }

    #[cfg(feature = "std")]
    #[structural_match]
    #[rustc_copy_clone_marker]
    pub struct Module<T: Trait>(::std::marker::PhantomData<(T)>);

    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<T: ::std::clone::Clone + Trait> ::std::clone::Clone for Module<T> {
        #[inline]
        fn clone(&self) -> Module<T> {
            match *self {
                Module(ref __self_0_0) => Module(::std::clone::Clone::clone(&(*__self_0_0))),
            }
        }
    }

    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<T: ::std::marker::Copy + Trait> ::std::marker::Copy for Module<T> {}

    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<T: ::std::cmp::PartialEq + Trait> ::std::cmp::PartialEq for Module<T> {
        #[inline]
        fn eq(&self, other: &Module<T>) -> bool {
            match *other {
                Module(ref __self_1_0) => match *self {
                    Module(ref __self_0_0) => (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &Module<T>) -> bool {
            match *other {
                Module(ref __self_1_0) => match *self {
                    Module(ref __self_0_0) => (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }

    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<T: ::std::cmp::Eq + Trait> ::std::cmp::Eq for Module<T> {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::std::cmp::AssertParamIsEq<::std::marker::PhantomData<(T)>>;
            }
        }
    }

    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<T: ::std::fmt::Debug + Trait> ::std::fmt::Debug for Module<T> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                Module(ref __self_0_0) => {
                    let mut debug_trait_builder = f.debug_tuple("Module");
                    let _ = debug_trait_builder.field(&&(*__self_0_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }

    impl<T: Trait> ::srml_support::runtime_primitives::traits::OnInitialize<T::BlockNumber>
    for Module<T>
    {}

    impl<T: Trait> ::srml_support::runtime_primitives::traits::OnFinalize<T::BlockNumber>
    for Module<T>
    {}

    impl<T: Trait> ::srml_support::runtime_primitives::traits::OffchainWorker<T::BlockNumber>
    for Module<T>
    {}

    impl<T: Trait> Module<T> {
        fn deposit_event(event: Event<T>) {
            <system::Module<T>>::deposit_event(<T as Trait>::from(event).into());
        }
    }

    /// Can also be called using [`Call`].
    ///
    /// [`Call`]: enum.Call.html
    impl<T: Trait> Module<T> {
        pub fn do_something(origin: T::Origin, something: u32) -> Result {
            let who = ensure_signed(origin)?;
            <Something<T>>::put(something);
            Self::deposit_event(RawEvent::SomethingStored(something, who));
            Ok(())
        }
        pub fn record_property(origin: T::Origin, size: u64, certificate_no: u64) -> Result {
            let sender = ensure_signed(origin)?;
            let nonce = <Nonce<T>>::get();
            let random_seed = <system::Module<T>>::random_seed();
            let random_hash =
                (random_seed, &sender, nonce).using_encoded(<T as system::Trait>::hash);
            runtime_io::print("Random hash is:");
            runtime_io::print(&Encode::encode(&random_hash)[..]);
            let property = Property {
                id: random_hash,
                size: size,
                certificate_no: certificate_no,
                is_authenticated: false,
            };
            <Properties<T>>::insert(random_hash, property);
            <AllPropertiesArray<T>>::insert(nonce, random_hash);
            <PropertyOwner<T>>::insert(random_hash, sender);
            <Nonce<T>>::mutate(|n| *n += 1);
            Ok(())
        }
        pub fn add_manager(origin: T::Origin, account_id: T::AccountId) -> Result {
            let sender = ensure_root(origin)?;
            {
                if !!<ManagersIndex<T>>::exists(&account_id) {
                    {
                        return Err("The account is already manager");
                    };
                }
            };
            let nonce = <ManagerNonce<T>>::get();
            <Managers<T>>::insert(nonce, &account_id);
            <ManagersIndex<T>>::insert(&account_id, nonce);
            <ManagerNonce<T>>::mutate(|n| *n += 1);
            Self::deposit_event(RawEvent::ManagerAdded(account_id));
            Ok(())
        }
        pub fn authenticate(
            origin: T::Origin,
            property_id: T::Hash,
            is_authenticated: bool,
        ) -> Result {
            let sender = ensure_signed(origin)?;
            {
                if !<Properties<T>>::exists(property_id) {
                    {
                        return Err("The property is not exist");
                    };
                }
            };
            {
                if !<ManagersIndex<T>>::exists(sender.clone()) {
                    {
                        return Err("The sender is not a manager");
                    };
                }
            };
            let mut property = Self::property(property_id);
            property.is_authenticated = is_authenticated;
            <Properties<T>>::insert(property_id, property);
            Self::deposit_event(RawEvent::Authenticated(
                sender,
                property_id,
                is_authenticated,
            ));
            Ok(())
        }
        pub fn sell(origin: T::Origin, property_id: T::Hash, price: T::Balance) -> Result {
            let sender = ensure_signed(origin)?;
            {
                if !<Properties<T>>::exists(property_id) {
                    {
                        return Err("The property is not exist");
                    };
                }
            };
            let owner = Self::property_owner(property_id);
            {
                if !(owner == sender) {
                    {
                        return Err("You do not own this property");
                    };
                }
            };
            {
                if !!<PropertyForsaleIndex<T>>::exists(property_id) {
                    {
                        return Err("The property is already for sale");
                    };
                }
            };
            let property = Self::property(property_id);
            {
                if !property.is_authenticated {
                    {
                        return Err("The property is not authenticated");
                    };
                }
            };
            let property_forsale_count = Self::property_forsale_count();
            let new_property_forsale_count = property_forsale_count
                .checked_add(1)
                .ok_or("Overflow when adding new property")?;
            <PropertiesForsale<T>>::insert(
                property_forsale_count,
                (property_id, price, false, None),
            );
            <PropertyForsaleCount<T>>::put(new_property_forsale_count);
            <PropertyForsaleIndex<T>>::insert(property_id, property_forsale_count);
            Self::deposit_event(RawEvent::PropertyForsale(property_id, price));
            Ok(())
        }
        pub fn buy(origin: T::Origin, property_id: T::Hash) -> Result {
            let sender = ensure_signed(origin)?;
            {
                if !<Properties<T>>::exists(property_id) {
                    {
                        return Err("The property is not exist");
                    };
                }
            };
            let owner = Self::property_owner(property_id);
            {
                if !(owner != sender) {
                    {
                        return Err("You can not buy your own property");
                    };
                }
            };
            {
                if !<PropertyForsaleIndex<T>>::exists(property_id) {
                    {
                        return Err("The property is not ready for sale");
                    };
                }
            };
            let property_forsale_index = <PropertyForsaleIndex<T>>::get(property_id);
            let (_, price, is_lock, buyer) = Self::forsale_property(property_forsale_index);
            {
                if !(is_lock == false) {
                    {
                        return Err("The property is locked by another buyer");
                    };
                }
            };
            {
                if !(buyer == None) {
                    {
                        return Err("The property is locked by another buyer");
                    };
                }
            };
            {
                if !(<balances::Module<T>>::free_balance(&sender) >= price) {
                    {
                        return Err("You don't have enough free balance to buy this property");
                    };
                }
            };
            <balances::Module<T>>::reserve(&sender, price)?;
            <PropertiesForsale<T>>::insert(
                property_forsale_index,
                (property_id, price, true, Some(sender.clone())),
            );
            Self::deposit_event(RawEvent::BuyProperty(property_id, sender, owner));
            Ok(())
        }
        pub fn authenticate_trade(origin: T::Origin, property_id: T::Hash) -> Result {
            let sender = ensure_signed(origin)?;
            {
                if !<Properties<T>>::exists(property_id) {
                    {
                        return Err("The property is not exist");
                    };
                }
            };
            {
                if !<ManagersIndex<T>>::exists(sender.clone()) {
                    {
                        return Err("The sender is not a manager");
                    };
                }
            };
            {
                if !<PropertyForsaleIndex<T>>::exists(property_id) {
                    {
                        return Err("The property is not ready for sale");
                    };
                }
            };
            let property_forsale_index = <PropertyForsaleIndex<T>>::get(property_id);
            let (_, price, is_lock, buyer_option) = Self::forsale_property(property_forsale_index);
            {
                if !(is_lock == true) {
                    {
                        return Err("The property is unlocked");
                    };
                }
            };
            {
                if !(buyer_option != None) {
                    {
                        return Err("There is no buyer for this property");
                    };
                }
            };
            let buyer = buyer_option.clone().unwrap();
            <balances::Module<T>>::unreserve(&buyer, price);
            let property_forsale_count = Self::property_forsale_count();
            let new_property_forsale_count = property_forsale_count
                .checked_sub(1)
                .ok_or("Underflow when decrease a kitty")?;
            if (property_forsale_index != new_property_forsale_count) {
                let last_property_forsale_info = Self::forsale_property(new_property_forsale_count);
                <PropertiesForsale<T>>::insert(
                    new_property_forsale_count,
                    (property_id, price, is_lock, buyer_option),
                );
                <PropertiesForsale<T>>::insert(property_forsale_index, last_property_forsale_info);
            }
            <PropertiesForsale<T>>::remove(new_property_forsale_count);
            <PropertyForsaleIndex<T>>::remove(property_id);
            <PropertyForsaleCount<T>>::put(new_property_forsale_count);
            let owner = Self::property_owner(property_id);
            <PropertyOwner<T>>::insert(property_id, &buyer);
            <balances::Module<T> as Currency<_>>::transfer(&buyer, &owner, price)?;
            Self::deposit_event(RawEvent::TradeAuthenticated(
                property_id,
                buyer,
                owner,
                sender,
            ));
            Ok(())
        }
    }

    #[cfg(feature = "std")]
    /// The module declaration.
    pub enum Call<T: Trait> {
        #[doc(hidden)]
        __PhantomItem(
            ::std::marker::PhantomData<(T)>,
            ::srml_support::dispatch::Never,
        ),
        #[allow(non_camel_case_types)]
        do_something(u32),
        #[allow(non_camel_case_types)]
        record_property(u64, u64),
        #[allow(non_camel_case_types)]
        add_manager(T::AccountId),
        #[allow(non_camel_case_types)]
        authenticate(T::Hash, bool),
        #[allow(non_camel_case_types)]
        sell(T::Hash, T::Balance),
        #[allow(non_camel_case_types)]
        buy(T::Hash),
        #[allow(non_camel_case_types)]
        authenticate_trade(T::Hash),
    }

    impl<T: Trait> ::srml_support::dispatch::Clone for Call<T> {
        fn clone(&self) -> Self {
            match *self {
                Call::do_something(ref something) => Call::do_something((*something).clone()),
                Call::record_property(ref size, ref certificate_no) => {
                    Call::record_property((*size).clone(), (*certificate_no).clone())
                }
                Call::add_manager(ref account_id) => Call::add_manager((*account_id).clone()),
                Call::authenticate(ref property_id, ref is_authenticated) => {
                    Call::authenticate((*property_id).clone(), (*is_authenticated).clone())
                }
                Call::sell(ref property_id, ref price) => {
                    Call::sell((*property_id).clone(), (*price).clone())
                }
                Call::buy(ref property_id) => Call::buy((*property_id).clone()),
                Call::authenticate_trade(ref property_id) => {
                    Call::authenticate_trade((*property_id).clone())
                }
                _ => ::std::rt::begin_panic(
                    "internal error: entered unreachable code",
                    &("src/realestate.rs", 62u32, 1u32),
                ),
            }
        }
    }

    impl<T: Trait> ::srml_support::dispatch::PartialEq for Call<T> {
        fn eq(&self, _other: &Self) -> bool {
            match *self {
                Call::do_something(ref something) => {
                    let self_params = (something, );
                    if let Call::do_something(ref something) = *_other {
                        self_params == (something, )
                    } else {
                        match *_other {
                            Call::__PhantomItem(_, _) => ::std::rt::begin_panic(
                                "internal error: entered unreachable code",
                                &("src/realestate.rs", 62u32, 1u32),
                            ),
                            _ => false,
                        }
                    }
                }
                Call::record_property(ref size, ref certificate_no) => {
                    let self_params = (size, certificate_no);
                    if let Call::record_property(ref size, ref certificate_no) = *_other {
                        self_params == (size, certificate_no)
                    } else {
                        match *_other {
                            Call::__PhantomItem(_, _) => ::std::rt::begin_panic(
                                "internal error: entered unreachable code",
                                &("src/realestate.rs", 62u32, 1u32),
                            ),
                            _ => false,
                        }
                    }
                }
                Call::add_manager(ref account_id) => {
                    let self_params = (account_id, );
                    if let Call::add_manager(ref account_id) = *_other {
                        self_params == (account_id, )
                    } else {
                        match *_other {
                            Call::__PhantomItem(_, _) => ::std::rt::begin_panic(
                                "internal error: entered unreachable code",
                                &("src/realestate.rs", 62u32, 1u32),
                            ),
                            _ => false,
                        }
                    }
                }
                Call::authenticate(ref property_id, ref is_authenticated) => {
                    let self_params = (property_id, is_authenticated);
                    if let Call::authenticate(ref property_id, ref is_authenticated) = *_other {
                        self_params == (property_id, is_authenticated)
                    } else {
                        match *_other {
                            Call::__PhantomItem(_, _) => ::std::rt::begin_panic(
                                "internal error: entered unreachable code",
                                &("src/realestate.rs", 62u32, 1u32),
                            ),
                            _ => false,
                        }
                    }
                }
                Call::sell(ref property_id, ref price) => {
                    let self_params = (property_id, price);
                    if let Call::sell(ref property_id, ref price) = *_other {
                        self_params == (property_id, price)
                    } else {
                        match *_other {
                            Call::__PhantomItem(_, _) => ::std::rt::begin_panic(
                                "internal error: entered unreachable code",
                                &("src/realestate.rs", 62u32, 1u32),
                            ),
                            _ => false,
                        }
                    }
                }
                Call::buy(ref property_id) => {
                    let self_params = (property_id, );
                    if let Call::buy(ref property_id) = *_other {
                        self_params == (property_id, )
                    } else {
                        match *_other {
                            Call::__PhantomItem(_, _) => ::std::rt::begin_panic(
                                "internal error: entered unreachable code",
                                &("src/realestate.rs", 62u32, 1u32),
                            ),
                            _ => false,
                        }
                    }
                }
                Call::authenticate_trade(ref property_id) => {
                    let self_params = (property_id, );
                    if let Call::authenticate_trade(ref property_id) = *_other {
                        self_params == (property_id, )
                    } else {
                        match *_other {
                            Call::__PhantomItem(_, _) => ::std::rt::begin_panic(
                                "internal error: entered unreachable code",
                                &("src/realestate.rs", 62u32, 1u32),
                            ),
                            _ => false,
                        }
                    }
                }
                _ => ::std::rt::begin_panic(
                    "internal error: entered unreachable code",
                    &("src/realestate.rs", 62u32, 1u32),
                ),
            }
        }
    }

    impl<T: Trait> ::srml_support::dispatch::Eq for Call<T> {}

    #[cfg(feature = "std")]
    impl<T: Trait> ::srml_support::dispatch::fmt::Debug for Call<T> {
        fn fmt(
            &self,
            _f: &mut ::srml_support::dispatch::fmt::Formatter,
        ) -> ::srml_support::dispatch::result::Result<(), ::srml_support::dispatch::fmt::Error>
        {
            match *self {
                Call::do_something(ref something) => _f.write_fmt(::std::fmt::Arguments::new_v1(
                    &["", ""],
                    &match (&"do_something", &(something.clone(), )) {
                        (arg0, arg1) => [
                            ::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Display::fmt),
                            ::std::fmt::ArgumentV1::new(arg1, ::std::fmt::Debug::fmt),
                        ],
                    },
                )),
                Call::record_property(ref size, ref certificate_no) => {
                    _f.write_fmt(::std::fmt::Arguments::new_v1(
                        &["", ""],
                        &match (&"record_property", &(size.clone(), certificate_no.clone())) {
                            (arg0, arg1) => [
                                ::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Display::fmt),
                                ::std::fmt::ArgumentV1::new(arg1, ::std::fmt::Debug::fmt),
                            ],
                        },
                    ))
                }
                Call::add_manager(ref account_id) => _f.write_fmt(::std::fmt::Arguments::new_v1(
                    &["", ""],
                    &match (&"add_manager", &(account_id.clone(), )) {
                        (arg0, arg1) => [
                            ::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Display::fmt),
                            ::std::fmt::ArgumentV1::new(arg1, ::std::fmt::Debug::fmt),
                        ],
                    },
                )),
                Call::authenticate(ref property_id, ref is_authenticated) => {
                    _f.write_fmt(::std::fmt::Arguments::new_v1(
                        &["", ""],
                        &match (
                            &"authenticate",
                            &(property_id.clone(), is_authenticated.clone()),
                        ) {
                            (arg0, arg1) => [
                                ::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Display::fmt),
                                ::std::fmt::ArgumentV1::new(arg1, ::std::fmt::Debug::fmt),
                            ],
                        },
                    ))
                }
                Call::sell(ref property_id, ref price) => {
                    _f.write_fmt(::std::fmt::Arguments::new_v1(
                        &["", ""],
                        &match (&"sell", &(property_id.clone(), price.clone())) {
                            (arg0, arg1) => [
                                ::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Display::fmt),
                                ::std::fmt::ArgumentV1::new(arg1, ::std::fmt::Debug::fmt),
                            ],
                        },
                    ))
                }
                Call::buy(ref property_id) => _f.write_fmt(::std::fmt::Arguments::new_v1(
                    &["", ""],
                    &match (&"buy", &(property_id.clone(), )) {
                        (arg0, arg1) => [
                            ::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Display::fmt),
                            ::std::fmt::ArgumentV1::new(arg1, ::std::fmt::Debug::fmt),
                        ],
                    },
                )),
                Call::authenticate_trade(ref property_id) => {
                    _f.write_fmt(::std::fmt::Arguments::new_v1(
                        &["", ""],
                        &match (&"authenticate_trade", &(property_id.clone(), )) {
                            (arg0, arg1) => [
                                ::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Display::fmt),
                                ::std::fmt::ArgumentV1::new(arg1, ::std::fmt::Debug::fmt),
                            ],
                        },
                    ))
                }
                _ => ::std::rt::begin_panic(
                    "internal error: entered unreachable code",
                    &("src/realestate.rs", 62u32, 1u32),
                ),
            }
        }
    }

    impl<T: Trait> ::srml_support::dispatch::Decode for Call<T> {
        fn decode<Input: ::srml_support::dispatch::Input>(input: &mut Input) -> Option<Self> {
            let _input_id = input.read_byte()?;
            {
                if _input_id == (0) {
                    let something = ::srml_support::dispatch::Decode::decode(input)?;
                    return Some(Call::do_something(something));
                }
                {
                    if _input_id == (0 + 1) {
                        let size = ::srml_support::dispatch::Decode::decode(input)?;
                        let certificate_no = ::srml_support::dispatch::Decode::decode(input)?;
                        return Some(Call::record_property(size, certificate_no));
                    }
                    {
                        if _input_id == (0 + 1 + 1) {
                            let account_id = ::srml_support::dispatch::Decode::decode(input)?;
                            return Some(Call::add_manager(account_id));
                        }
                        {
                            if _input_id == (0 + 1 + 1 + 1) {
                                let property_id = ::srml_support::dispatch::Decode::decode(input)?;
                                let is_authenticated =
                                    ::srml_support::dispatch::Decode::decode(input)?;
                                return Some(Call::authenticate(property_id, is_authenticated));
                            }
                            {
                                if _input_id == (0 + 1 + 1 + 1 + 1) {
                                    let property_id =
                                        ::srml_support::dispatch::Decode::decode(input)?;
                                    let price = ::srml_support::dispatch::Decode::decode(input)?;
                                    return Some(Call::sell(property_id, price));
                                }
                                {
                                    if _input_id == (0 + 1 + 1 + 1 + 1 + 1) {
                                        let property_id =
                                            ::srml_support::dispatch::Decode::decode(input)?;
                                        return Some(Call::buy(property_id));
                                    }
                                    {
                                        if _input_id == (0 + 1 + 1 + 1 + 1 + 1 + 1) {
                                            let property_id =
                                                ::srml_support::dispatch::Decode::decode(input)?;
                                            return Some(Call::authenticate_trade(property_id));
                                        }
                                        None
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    impl<T: Trait> ::srml_support::dispatch::Encode for Call<T> {
        fn encode_to<W: ::srml_support::dispatch::Output>(&self, _dest: &mut W) {
            {
                if let Call::do_something(ref something) = *self {
                    _dest.push_byte((0) as u8);
                    something.encode_to(_dest);
                }
                {
                    if let Call::record_property(ref size, ref certificate_no) = *self {
                        _dest.push_byte((0 + 1) as u8);
                        size.encode_to(_dest);
                        certificate_no.encode_to(_dest);
                    }
                    {
                        if let Call::add_manager(ref account_id) = *self {
                            _dest.push_byte((0 + 1 + 1) as u8);
                            account_id.encode_to(_dest);
                        }
                        {
                            if let Call::authenticate(ref property_id, ref is_authenticated) = *self
                            {
                                _dest.push_byte((0 + 1 + 1 + 1) as u8);
                                property_id.encode_to(_dest);
                                is_authenticated.encode_to(_dest);
                            }
                            {
                                if let Call::sell(ref property_id, ref price) = *self {
                                    _dest.push_byte((0 + 1 + 1 + 1 + 1) as u8);
                                    property_id.encode_to(_dest);
                                    price.encode_to(_dest);
                                }
                                {
                                    if let Call::buy(ref property_id) = *self {
                                        _dest.push_byte((0 + 1 + 1 + 1 + 1 + 1) as u8);
                                        property_id.encode_to(_dest);
                                    }
                                    {
                                        if let Call::authenticate_trade(ref property_id) = *self {
                                            _dest.push_byte((0 + 1 + 1 + 1 + 1 + 1 + 1) as u8);
                                            property_id.encode_to(_dest);
                                        }
                                        {}
                                    }
                                }
                            }
                        }
                    }
                }
            };
        }
    }

    impl<T: Trait> ::srml_support::dispatch::Dispatchable for Call<T> {
        type Trait = T;
        type Origin = T::Origin;
        fn dispatch(self, _origin: Self::Origin) -> ::srml_support::dispatch::Result {
            match self {
                Call::do_something(something) => <Module<T>>::do_something(_origin, something),
                Call::record_property(size, certificate_no) => {
                    <Module<T>>::record_property(_origin, size, certificate_no)
                }
                Call::add_manager(account_id) => <Module<T>>::add_manager(_origin, account_id),
                Call::authenticate(property_id, is_authenticated) => {
                    <Module<T>>::authenticate(_origin, property_id, is_authenticated)
                }
                Call::sell(property_id, price) => <Module<T>>::sell(_origin, property_id, price),
                Call::buy(property_id) => <Module<T>>::buy(_origin, property_id),
                Call::authenticate_trade(property_id) => {
                    <Module<T>>::authenticate_trade(_origin, property_id)
                }
                Call::__PhantomItem(_, _) => ::std::rt::begin_panic_fmt(
                    &::std::fmt::Arguments::new_v1(
                        &["internal error: entered unreachable code: "],
                        &match (&"__PhantomItem should never be used.", ) {
                            (arg0, ) => {
                                [::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Display::fmt)]
                            }
                        },
                    ),
                    &("src/realestate.rs", 62u32, 1u32),
                ),
            }
        }
    }

    impl<T: Trait> ::srml_support::dispatch::Callable for Module<T> {
        type Call = Call<T>;
    }

    impl<T: Trait> Module<T> {
        #[doc(hidden)]
        pub fn dispatch<D: ::srml_support::dispatch::Dispatchable<Trait=T>>(
            d: D,
            origin: D::Origin,
        ) -> ::srml_support::dispatch::Result {
            d.dispatch(origin)
        }
    }

    impl<T: Trait> Module<T> {
        #[doc(hidden)]
        pub fn call_functions() -> &'static [::srml_support::dispatch::FunctionMetadata] {
            &[
                ::srml_support::dispatch::FunctionMetadata {
                    name: ::srml_support::dispatch::DecodeDifferent::Encode("do_something"),
                    arguments: ::srml_support::dispatch::DecodeDifferent::Encode(&[
                        ::srml_support::dispatch::FunctionArgumentMetadata {
                            name: ::srml_support::dispatch::DecodeDifferent::Encode("something"),
                            ty: ::srml_support::dispatch::DecodeDifferent::Encode("u32"),
                        },
                    ]),
                    documentation: ::srml_support::dispatch::DecodeDifferent::Encode(&[]),
                },
                ::srml_support::dispatch::FunctionMetadata {
                    name: ::srml_support::dispatch::DecodeDifferent::Encode("record_property"),
                    arguments: ::srml_support::dispatch::DecodeDifferent::Encode(&[
                        ::srml_support::dispatch::FunctionArgumentMetadata {
                            name: ::srml_support::dispatch::DecodeDifferent::Encode("size"),
                            ty: ::srml_support::dispatch::DecodeDifferent::Encode("u64"),
                        },
                        ::srml_support::dispatch::FunctionArgumentMetadata {
                            name: ::srml_support::dispatch::DecodeDifferent::Encode(
                                "certificate_no",
                            ),
                            ty: ::srml_support::dispatch::DecodeDifferent::Encode("u64"),
                        },
                    ]),
                    documentation: ::srml_support::dispatch::DecodeDifferent::Encode(&[]),
                },
                ::srml_support::dispatch::FunctionMetadata {
                    name: ::srml_support::dispatch::DecodeDifferent::Encode("add_manager"),
                    arguments: ::srml_support::dispatch::DecodeDifferent::Encode(&[
                        ::srml_support::dispatch::FunctionArgumentMetadata {
                            name: ::srml_support::dispatch::DecodeDifferent::Encode("account_id"),
                            ty: ::srml_support::dispatch::DecodeDifferent::Encode("T::AccountId"),
                        },
                    ]),
                    documentation: ::srml_support::dispatch::DecodeDifferent::Encode(&[]),
                },
                ::srml_support::dispatch::FunctionMetadata {
                    name: ::srml_support::dispatch::DecodeDifferent::Encode("authenticate"),
                    arguments: ::srml_support::dispatch::DecodeDifferent::Encode(&[
                        ::srml_support::dispatch::FunctionArgumentMetadata {
                            name: ::srml_support::dispatch::DecodeDifferent::Encode("property_id"),
                            ty: ::srml_support::dispatch::DecodeDifferent::Encode("T::Hash"),
                        },
                        ::srml_support::dispatch::FunctionArgumentMetadata {
                            name: ::srml_support::dispatch::DecodeDifferent::Encode(
                                "is_authenticated",
                            ),
                            ty: ::srml_support::dispatch::DecodeDifferent::Encode("bool"),
                        },
                    ]),
                    documentation: ::srml_support::dispatch::DecodeDifferent::Encode(&[]),
                },
                ::srml_support::dispatch::FunctionMetadata {
                    name: ::srml_support::dispatch::DecodeDifferent::Encode("sell"),
                    arguments: ::srml_support::dispatch::DecodeDifferent::Encode(&[
                        ::srml_support::dispatch::FunctionArgumentMetadata {
                            name: ::srml_support::dispatch::DecodeDifferent::Encode("property_id"),
                            ty: ::srml_support::dispatch::DecodeDifferent::Encode("T::Hash"),
                        },
                        ::srml_support::dispatch::FunctionArgumentMetadata {
                            name: ::srml_support::dispatch::DecodeDifferent::Encode("price"),
                            ty: ::srml_support::dispatch::DecodeDifferent::Encode("T::Balance"),
                        },
                    ]),
                    documentation: ::srml_support::dispatch::DecodeDifferent::Encode(&[]),
                },
                ::srml_support::dispatch::FunctionMetadata {
                    name: ::srml_support::dispatch::DecodeDifferent::Encode("buy"),
                    arguments: ::srml_support::dispatch::DecodeDifferent::Encode(&[
                        ::srml_support::dispatch::FunctionArgumentMetadata {
                            name: ::srml_support::dispatch::DecodeDifferent::Encode("property_id"),
                            ty: ::srml_support::dispatch::DecodeDifferent::Encode("T::Hash"),
                        },
                    ]),
                    documentation: ::srml_support::dispatch::DecodeDifferent::Encode(&[]),
                },
                ::srml_support::dispatch::FunctionMetadata {
                    name: ::srml_support::dispatch::DecodeDifferent::Encode("authenticate_trade"),
                    arguments: ::srml_support::dispatch::DecodeDifferent::Encode(&[
                        ::srml_support::dispatch::FunctionArgumentMetadata {
                            name: ::srml_support::dispatch::DecodeDifferent::Encode("property_id"),
                            ty: ::srml_support::dispatch::DecodeDifferent::Encode("T::Hash"),
                        },
                    ]),
                    documentation: ::srml_support::dispatch::DecodeDifferent::Encode(&[]),
                },
            ]
        }
    }

    /// [`RawEvent`] specialized for the configuration [`Trait`]
    ///
    /// [`RawEvent`]: enum.RawEvent.html
    /// [`Trait`]: trait.Trait.html
    pub type Event<T> = RawEvent<
        <T as system::Trait>::AccountId,
        <T as system::Trait>::Hash,
        <T as balances::Trait>::Balance,
    >;

    /// Events for this module.
    ///
    #[structural_match]
    pub enum RawEvent<AccountId, Hash, Balance> {
        SomethingStored(u32, AccountId),
        Authenticated(AccountId, Hash, bool),
        ManagerAdded(AccountId),
        PropertyForsale(Hash, Balance),
        BuyProperty(Hash, AccountId, AccountId),
        TradeAuthenticated(Hash, AccountId, AccountId, AccountId),
    }

    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<
        AccountId: ::std::clone::Clone,
        Hash: ::std::clone::Clone,
        Balance: ::std::clone::Clone,
    > ::std::clone::Clone for RawEvent<AccountId, Hash, Balance>
    {
        #[inline]
        fn clone(&self) -> RawEvent<AccountId, Hash, Balance> {
            match (&*self, ) {
                (&RawEvent::SomethingStored(ref __self_0, ref __self_1), ) => {
                    RawEvent::SomethingStored(
                        ::std::clone::Clone::clone(&(*__self_0)),
                        ::std::clone::Clone::clone(&(*__self_1)),
                    )
                }
                (&RawEvent::Authenticated(ref __self_0, ref __self_1, ref __self_2), ) => {
                    RawEvent::Authenticated(
                        ::std::clone::Clone::clone(&(*__self_0)),
                        ::std::clone::Clone::clone(&(*__self_1)),
                        ::std::clone::Clone::clone(&(*__self_2)),
                    )
                }
                (&RawEvent::ManagerAdded(ref __self_0), ) => {
                    RawEvent::ManagerAdded(::std::clone::Clone::clone(&(*__self_0)))
                }
                (&RawEvent::PropertyForsale(ref __self_0, ref __self_1), ) => {
                    RawEvent::PropertyForsale(
                        ::std::clone::Clone::clone(&(*__self_0)),
                        ::std::clone::Clone::clone(&(*__self_1)),
                    )
                }
                (&RawEvent::BuyProperty(ref __self_0, ref __self_1, ref __self_2), ) => {
                    RawEvent::BuyProperty(
                        ::std::clone::Clone::clone(&(*__self_0)),
                        ::std::clone::Clone::clone(&(*__self_1)),
                        ::std::clone::Clone::clone(&(*__self_2)),
                    )
                }
                (&RawEvent::TradeAuthenticated(
                    ref __self_0,
                    ref __self_1,
                    ref __self_2,
                    ref __self_3,
                ), ) => RawEvent::TradeAuthenticated(
                    ::std::clone::Clone::clone(&(*__self_0)),
                    ::std::clone::Clone::clone(&(*__self_1)),
                    ::std::clone::Clone::clone(&(*__self_2)),
                    ::std::clone::Clone::clone(&(*__self_3)),
                ),
            }
        }
    }

    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<
        AccountId: ::std::cmp::PartialEq,
        Hash: ::std::cmp::PartialEq,
        Balance: ::std::cmp::PartialEq,
    > ::std::cmp::PartialEq for RawEvent<AccountId, Hash, Balance>
    {
        #[inline]
        fn eq(&self, other: &RawEvent<AccountId, Hash, Balance>) -> bool {
            {
                let __self_vi = unsafe { ::std::intrinsics::discriminant_value(&*self) } as isize;
                let __arg_1_vi = unsafe { ::std::intrinsics::discriminant_value(&*other) } as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (
                            &RawEvent::SomethingStored(ref __self_0, ref __self_1),
                            &RawEvent::SomethingStored(ref __arg_1_0, ref __arg_1_1),
                        ) => (*__self_0) == (*__arg_1_0) && (*__self_1) == (*__arg_1_1),
                        (
                            &RawEvent::Authenticated(ref __self_0, ref __self_1, ref __self_2),
                            &RawEvent::Authenticated(ref __arg_1_0, ref __arg_1_1, ref __arg_1_2),
                        ) => {
                            (*__self_0) == (*__arg_1_0)
                                && (*__self_1) == (*__arg_1_1)
                                && (*__self_2) == (*__arg_1_2)
                        }
                        (
                            &RawEvent::ManagerAdded(ref __self_0),
                            &RawEvent::ManagerAdded(ref __arg_1_0),
                        ) => (*__self_0) == (*__arg_1_0),
                        (
                            &RawEvent::PropertyForsale(ref __self_0, ref __self_1),
                            &RawEvent::PropertyForsale(ref __arg_1_0, ref __arg_1_1),
                        ) => (*__self_0) == (*__arg_1_0) && (*__self_1) == (*__arg_1_1),
                        (
                            &RawEvent::BuyProperty(ref __self_0, ref __self_1, ref __self_2),
                            &RawEvent::BuyProperty(ref __arg_1_0, ref __arg_1_1, ref __arg_1_2),
                        ) => {
                            (*__self_0) == (*__arg_1_0)
                                && (*__self_1) == (*__arg_1_1)
                                && (*__self_2) == (*__arg_1_2)
                        }
                        (
                            &RawEvent::TradeAuthenticated(
                                ref __self_0,
                                ref __self_1,
                                ref __self_2,
                                ref __self_3,
                            ),
                            &RawEvent::TradeAuthenticated(
                                ref __arg_1_0,
                                ref __arg_1_1,
                                ref __arg_1_2,
                                ref __arg_1_3,
                            ),
                        ) => {
                            (*__self_0) == (*__arg_1_0)
                                && (*__self_1) == (*__arg_1_1)
                                && (*__self_2) == (*__arg_1_2)
                                && (*__self_3) == (*__arg_1_3)
                        }
                        _ => unsafe { ::std::intrinsics::unreachable() },
                    }
                } else {
                    false
                }
            }
        }
        #[inline]
        fn ne(&self, other: &RawEvent<AccountId, Hash, Balance>) -> bool {
            {
                let __self_vi = unsafe { ::std::intrinsics::discriminant_value(&*self) } as isize;
                let __arg_1_vi = unsafe { ::std::intrinsics::discriminant_value(&*other) } as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (
                            &RawEvent::SomethingStored(ref __self_0, ref __self_1),
                            &RawEvent::SomethingStored(ref __arg_1_0, ref __arg_1_1),
                        ) => (*__self_0) != (*__arg_1_0) || (*__self_1) != (*__arg_1_1),
                        (
                            &RawEvent::Authenticated(ref __self_0, ref __self_1, ref __self_2),
                            &RawEvent::Authenticated(ref __arg_1_0, ref __arg_1_1, ref __arg_1_2),
                        ) => {
                            (*__self_0) != (*__arg_1_0)
                                || (*__self_1) != (*__arg_1_1)
                                || (*__self_2) != (*__arg_1_2)
                        }
                        (
                            &RawEvent::ManagerAdded(ref __self_0),
                            &RawEvent::ManagerAdded(ref __arg_1_0),
                        ) => (*__self_0) != (*__arg_1_0),
                        (
                            &RawEvent::PropertyForsale(ref __self_0, ref __self_1),
                            &RawEvent::PropertyForsale(ref __arg_1_0, ref __arg_1_1),
                        ) => (*__self_0) != (*__arg_1_0) || (*__self_1) != (*__arg_1_1),
                        (
                            &RawEvent::BuyProperty(ref __self_0, ref __self_1, ref __self_2),
                            &RawEvent::BuyProperty(ref __arg_1_0, ref __arg_1_1, ref __arg_1_2),
                        ) => {
                            (*__self_0) != (*__arg_1_0)
                                || (*__self_1) != (*__arg_1_1)
                                || (*__self_2) != (*__arg_1_2)
                        }
                        (
                            &RawEvent::TradeAuthenticated(
                                ref __self_0,
                                ref __self_1,
                                ref __self_2,
                                ref __self_3,
                            ),
                            &RawEvent::TradeAuthenticated(
                                ref __arg_1_0,
                                ref __arg_1_1,
                                ref __arg_1_2,
                                ref __arg_1_3,
                            ),
                        ) => {
                            (*__self_0) != (*__arg_1_0)
                                || (*__self_1) != (*__arg_1_1)
                                || (*__self_2) != (*__arg_1_2)
                                || (*__self_3) != (*__arg_1_3)
                        }
                        _ => unsafe { ::std::intrinsics::unreachable() },
                    }
                } else {
                    true
                }
            }
        }
    }

    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<AccountId: ::std::cmp::Eq, Hash: ::std::cmp::Eq, Balance: ::std::cmp::Eq> ::std::cmp::Eq
    for RawEvent<AccountId, Hash, Balance>
    {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::std::cmp::AssertParamIsEq<u32>;
                let _: ::std::cmp::AssertParamIsEq<AccountId>;
                let _: ::std::cmp::AssertParamIsEq<AccountId>;
                let _: ::std::cmp::AssertParamIsEq<Hash>;
                let _: ::std::cmp::AssertParamIsEq<bool>;
                let _: ::std::cmp::AssertParamIsEq<AccountId>;
                let _: ::std::cmp::AssertParamIsEq<Hash>;
                let _: ::std::cmp::AssertParamIsEq<Balance>;
                let _: ::std::cmp::AssertParamIsEq<Hash>;
                let _: ::std::cmp::AssertParamIsEq<AccountId>;
                let _: ::std::cmp::AssertParamIsEq<AccountId>;
                let _: ::std::cmp::AssertParamIsEq<Hash>;
                let _: ::std::cmp::AssertParamIsEq<AccountId>;
                let _: ::std::cmp::AssertParamIsEq<AccountId>;
                let _: ::std::cmp::AssertParamIsEq<AccountId>;
            }
        }
    }

    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_ENCODE_FOR_RawEvent: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl<AccountId, Hash, Balance> _parity_codec::Encode for RawEvent<AccountId, Hash, Balance>
            where
                AccountId: _parity_codec::Encode,
                AccountId: _parity_codec::Encode,
                AccountId: _parity_codec::Encode,
                AccountId: _parity_codec::Encode,
                Hash: _parity_codec::Encode,
                Hash: _parity_codec::Encode,
                AccountId: _parity_codec::Encode,
                AccountId: _parity_codec::Encode,
                Hash: _parity_codec::Encode,
                Hash: _parity_codec::Encode,
                Balance: _parity_codec::Encode,
                Balance: _parity_codec::Encode,
                Hash: _parity_codec::Encode,
                Hash: _parity_codec::Encode,
                AccountId: _parity_codec::Encode,
                AccountId: _parity_codec::Encode,
                AccountId: _parity_codec::Encode,
                AccountId: _parity_codec::Encode,
                Hash: _parity_codec::Encode,
                Hash: _parity_codec::Encode,
                AccountId: _parity_codec::Encode,
                AccountId: _parity_codec::Encode,
                AccountId: _parity_codec::Encode,
                AccountId: _parity_codec::Encode,
                AccountId: _parity_codec::Encode,
                AccountId: _parity_codec::Encode,
        {
            fn encode_to<EncOut: _parity_codec::Output>(&self, dest: &mut EncOut) {
                match *self {
                    RawEvent::SomethingStored(ref aa, ref ba) => {
                        dest.push_byte(0usize as u8);
                        dest.push(aa);
                        dest.push(ba);
                    }
                    RawEvent::Authenticated(ref aa, ref ba, ref ca) => {
                        dest.push_byte(1usize as u8);
                        dest.push(aa);
                        dest.push(ba);
                        dest.push(ca);
                    }
                    RawEvent::ManagerAdded(ref aa) => {
                        dest.push_byte(2usize as u8);
                        dest.push(aa);
                    }
                    RawEvent::PropertyForsale(ref aa, ref ba) => {
                        dest.push_byte(3usize as u8);
                        dest.push(aa);
                        dest.push(ba);
                    }
                    RawEvent::BuyProperty(ref aa, ref ba, ref ca) => {
                        dest.push_byte(4usize as u8);
                        dest.push(aa);
                        dest.push(ba);
                        dest.push(ca);
                    }
                    RawEvent::TradeAuthenticated(ref aa, ref ba, ref ca, ref da) => {
                        dest.push_byte(5usize as u8);
                        dest.push(aa);
                        dest.push(ba);
                        dest.push(ca);
                        dest.push(da);
                    }
                    _ => (),
                }
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DECODE_FOR_RawEvent: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl<AccountId, Hash, Balance> _parity_codec::Decode for RawEvent<AccountId, Hash, Balance>
            where
                AccountId: _parity_codec::Decode,
                AccountId: _parity_codec::Decode,
                AccountId: _parity_codec::Decode,
                AccountId: _parity_codec::Decode,
                Hash: _parity_codec::Decode,
                Hash: _parity_codec::Decode,
                AccountId: _parity_codec::Decode,
                AccountId: _parity_codec::Decode,
                Hash: _parity_codec::Decode,
                Hash: _parity_codec::Decode,
                Balance: _parity_codec::Decode,
                Balance: _parity_codec::Decode,
                Hash: _parity_codec::Decode,
                Hash: _parity_codec::Decode,
                AccountId: _parity_codec::Decode,
                AccountId: _parity_codec::Decode,
                AccountId: _parity_codec::Decode,
                AccountId: _parity_codec::Decode,
                Hash: _parity_codec::Decode,
                Hash: _parity_codec::Decode,
                AccountId: _parity_codec::Decode,
                AccountId: _parity_codec::Decode,
                AccountId: _parity_codec::Decode,
                AccountId: _parity_codec::Decode,
                AccountId: _parity_codec::Decode,
                AccountId: _parity_codec::Decode,
        {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn) -> Option<Self> {
                match input.read_byte()? {
                    x if x == 0usize as u8 => Some(RawEvent::SomethingStored(
                        _parity_codec::Decode::decode(input)?,
                        _parity_codec::Decode::decode(input)?,
                    )),
                    x if x == 1usize as u8 => Some(RawEvent::Authenticated(
                        _parity_codec::Decode::decode(input)?,
                        _parity_codec::Decode::decode(input)?,
                        _parity_codec::Decode::decode(input)?,
                    )),
                    x if x == 2usize as u8 => Some(RawEvent::ManagerAdded(
                        _parity_codec::Decode::decode(input)?,
                    )),
                    x if x == 3usize as u8 => Some(RawEvent::PropertyForsale(
                        _parity_codec::Decode::decode(input)?,
                        _parity_codec::Decode::decode(input)?,
                    )),
                    x if x == 4usize as u8 => Some(RawEvent::BuyProperty(
                        _parity_codec::Decode::decode(input)?,
                        _parity_codec::Decode::decode(input)?,
                        _parity_codec::Decode::decode(input)?,
                    )),
                    x if x == 5usize as u8 => Some(RawEvent::TradeAuthenticated(
                        _parity_codec::Decode::decode(input)?,
                        _parity_codec::Decode::decode(input)?,
                        _parity_codec::Decode::decode(input)?,
                        _parity_codec::Decode::decode(input)?,
                    )),
                    _ => None,
                }
            }
        }
    };

    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<AccountId: ::std::fmt::Debug, Hash: ::std::fmt::Debug, Balance: ::std::fmt::Debug>
    ::std::fmt::Debug for RawEvent<AccountId, Hash, Balance>
    {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match (&*self, ) {
                (&RawEvent::SomethingStored(ref __self_0, ref __self_1), ) => {
                    let mut debug_trait_builder = f.debug_tuple("SomethingStored");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    let _ = debug_trait_builder.field(&&(*__self_1));
                    debug_trait_builder.finish()
                }
                (&RawEvent::Authenticated(ref __self_0, ref __self_1, ref __self_2), ) => {
                    let mut debug_trait_builder = f.debug_tuple("Authenticated");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    let _ = debug_trait_builder.field(&&(*__self_1));
                    let _ = debug_trait_builder.field(&&(*__self_2));
                    debug_trait_builder.finish()
                }
                (&RawEvent::ManagerAdded(ref __self_0), ) => {
                    let mut debug_trait_builder = f.debug_tuple("ManagerAdded");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&RawEvent::PropertyForsale(ref __self_0, ref __self_1), ) => {
                    let mut debug_trait_builder = f.debug_tuple("PropertyForsale");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    let _ = debug_trait_builder.field(&&(*__self_1));
                    debug_trait_builder.finish()
                }
                (&RawEvent::BuyProperty(ref __self_0, ref __self_1, ref __self_2), ) => {
                    let mut debug_trait_builder = f.debug_tuple("BuyProperty");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    let _ = debug_trait_builder.field(&&(*__self_1));
                    let _ = debug_trait_builder.field(&&(*__self_2));
                    debug_trait_builder.finish()
                }
                (&RawEvent::TradeAuthenticated(
                    ref __self_0,
                    ref __self_1,
                    ref __self_2,
                    ref __self_3,
                ), ) => {
                    let mut debug_trait_builder = f.debug_tuple("TradeAuthenticated");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    let _ = debug_trait_builder.field(&&(*__self_1));
                    let _ = debug_trait_builder.field(&&(*__self_2));
                    let _ = debug_trait_builder.field(&&(*__self_3));
                    debug_trait_builder.finish()
                }
            }
        }
    }

    impl<AccountId, Hash, Balance> From<RawEvent<AccountId, Hash, Balance>> for () {
        fn from(_: RawEvent<AccountId, Hash, Balance>) -> () {
            ()
        }
    }

    impl<AccountId, Hash, Balance> RawEvent<AccountId, Hash, Balance> {
        #[allow(dead_code)]
        pub fn metadata() -> &'static [::srml_support::event::EventMetadata] {
            &[
                ::srml_support::event::EventMetadata {
                    name: ::srml_support::event::DecodeDifferent::Encode("SomethingStored"),
                    arguments: ::srml_support::event::DecodeDifferent::Encode(&[
                        "u32",
                        "AccountId",
                    ]),
                    documentation: ::srml_support::event::DecodeDifferent::Encode(&[]),
                },
                ::srml_support::event::EventMetadata {
                    name: ::srml_support::event::DecodeDifferent::Encode("Authenticated"),
                    arguments: ::srml_support::event::DecodeDifferent::Encode(&[
                        "AccountId",
                        "Hash",
                        "bool",
                    ]),
                    documentation: ::srml_support::event::DecodeDifferent::Encode(&[]),
                },
                ::srml_support::event::EventMetadata {
                    name: ::srml_support::event::DecodeDifferent::Encode("ManagerAdded"),
                    arguments: ::srml_support::event::DecodeDifferent::Encode(&["AccountId"]),
                    documentation: ::srml_support::event::DecodeDifferent::Encode(&[]),
                },
                ::srml_support::event::EventMetadata {
                    name: ::srml_support::event::DecodeDifferent::Encode("PropertyForsale"),
                    arguments: ::srml_support::event::DecodeDifferent::Encode(&["Hash", "Balance"]),
                    documentation: ::srml_support::event::DecodeDifferent::Encode(&[]),
                },
                ::srml_support::event::EventMetadata {
                    name: ::srml_support::event::DecodeDifferent::Encode("BuyProperty"),
                    arguments: ::srml_support::event::DecodeDifferent::Encode(&[
                        "Hash",
                        "AccountId",
                        "AccountId",
                    ]),
                    documentation: ::srml_support::event::DecodeDifferent::Encode(&[]),
                },
                ::srml_support::event::EventMetadata {
                    name: ::srml_support::event::DecodeDifferent::Encode("TradeAuthenticated"),
                    arguments: ::srml_support::event::DecodeDifferent::Encode(&[
                        "Hash",
                        "AccountId",
                        "AccountId",
                        "AccountId",
                    ]),
                    documentation: ::srml_support::event::DecodeDifferent::Encode(&[]),
                },
            ]
        }
    }
}

/// Opaque types. These are used by the CLI to instantiate machinery that don't need to know
/// the specifics of the runtime. They can then be made to be agnostic over specific formats
/// of data like extrinsics, allowing for them to continue syncing the network through upgrades
/// to even the core datastructures.
pub mod opaque {
    use super::*;

    /// Opaque, encoded, unchecked extrinsic.
    #[structural_match]
    pub struct UncheckedExtrinsic(#[serde(with = "bytes")] pub Vec<u8>);

    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for UncheckedExtrinsic {
        #[inline]
        fn eq(&self, other: &UncheckedExtrinsic) -> bool {
            match *other {
                UncheckedExtrinsic(ref __self_1_0) => match *self {
                    UncheckedExtrinsic(ref __self_0_0) => (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &UncheckedExtrinsic) -> bool {
            match *other {
                UncheckedExtrinsic(ref __self_1_0) => match *self {
                    UncheckedExtrinsic(ref __self_0_0) => (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }

    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::Eq for UncheckedExtrinsic {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::std::cmp::AssertParamIsEq<Vec<u8>>;
            }
        }
    }

    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for UncheckedExtrinsic {
        #[inline]
        fn clone(&self) -> UncheckedExtrinsic {
            match *self {
                UncheckedExtrinsic(ref __self_0_0) => {
                    UncheckedExtrinsic(::std::clone::Clone::clone(&(*__self_0_0)))
                }
            }
        }
    }

    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::default::Default for UncheckedExtrinsic {
        #[inline]
        fn default() -> UncheckedExtrinsic {
            UncheckedExtrinsic(::std::default::Default::default())
        }
    }

    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_ENCODE_FOR_UncheckedExtrinsic: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Encode for UncheckedExtrinsic {
            fn encode_to<EncOut: _parity_codec::Output>(&self, dest: &mut EncOut) {
                dest.push(&self.0);
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DECODE_FOR_UncheckedExtrinsic: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Decode for UncheckedExtrinsic {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn) -> Option<Self> {
                Some(UncheckedExtrinsic(_parity_codec::Decode::decode(input)?))
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_UncheckedExtrinsic: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for UncheckedExtrinsic {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
            {
                _serde::Serializer::serialize_newtype_struct(__serializer, "UncheckedExtrinsic", {
                    struct __SerializeWith<'__a> {
                        values: (&'__a Vec<u8>, ),
                        phantom: _serde::export::PhantomData<UncheckedExtrinsic>,
                    }
                    impl<'__a> _serde::Serialize for __SerializeWith<'__a> {
                        fn serialize<__S>(
                            &self,
                            __s: __S,
                        ) -> _serde::export::Result<__S::Ok, __S::Error>
                            where
                                __S: _serde::Serializer,
                        {
                            bytes::serialize(self.values.0, __s)
                        }
                    }
                    &__SerializeWith {
                        values: (&self.0, ),
                        phantom: _serde::export::PhantomData::<UncheckedExtrinsic>,
                    }
                })
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_UncheckedExtrinsic: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for UncheckedExtrinsic {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
            {
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<UncheckedExtrinsic>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = UncheckedExtrinsic;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(
                            __formatter,
                            "tuple struct UncheckedExtrinsic",
                        )
                    }
                    #[inline]
                    fn visit_newtype_struct<__E>(
                        self,
                        __e: __E,
                    ) -> _serde::export::Result<Self::Value, __E::Error>
                        where
                            __E: _serde::Deserializer<'de>,
                    {
                        let __field0: Vec<u8> = match bytes::deserialize(__e) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        };
                        _serde::export::Ok(UncheckedExtrinsic(__field0))
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match {
                            struct __DeserializeWith<'de> {
                                value: Vec<u8>,
                                phantom: _serde::export::PhantomData<UncheckedExtrinsic>,
                                lifetime: _serde::export::PhantomData<&'de ()>,
                            }
                            impl<'de> _serde::Deserialize<'de> for __DeserializeWith<'de> {
                                fn deserialize<__D>(
                                    __deserializer: __D,
                                ) -> _serde::export::Result<Self, __D::Error>
                                    where
                                        __D: _serde::Deserializer<'de>,
                                {
                                    _serde::export::Ok(__DeserializeWith {
                                        value: match bytes::deserialize(__deserializer) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                        phantom: _serde::export::PhantomData,
                                        lifetime: _serde::export::PhantomData,
                                    })
                                }
                            }
                            _serde::export::Option::map(
                                match _serde::de::SeqAccess::next_element::<__DeserializeWith<'de>>(
                                    &mut __seq,
                                ) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                },
                                |__wrap| __wrap.value,
                            )
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"tuple struct UncheckedExtrinsic with 1 element",
                                ));
                            }
                        };
                        _serde::export::Ok(UncheckedExtrinsic(__field0))
                    }
                }
                _serde::Deserializer::deserialize_newtype_struct(
                    __deserializer,
                    "UncheckedExtrinsic",
                    __Visitor {
                        marker: _serde::export::PhantomData::<UncheckedExtrinsic>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };

    #[cfg(feature = "std")]
    impl std::fmt::Debug for UncheckedExtrinsic {
        fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
            fmt.write_fmt(::std::fmt::Arguments::new_v1(
                &[""],
                &match (&primitives::hexdisplay::HexDisplay::from(&self.0), ) {
                    (arg0, ) => [::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Display::fmt)],
                },
            ))
        }
    }

    impl traits::Extrinsic for UncheckedExtrinsic {
        fn is_signed(&self) -> Option<bool> {
            None
        }
    }

    /// Opaque block header type.
    pub type Header = generic::Header<
        BlockNumber,
        BlakeTwo256,
        generic::DigestItem<Hash, AuthorityId, AuthoritySignature>,
    >;
    /// Opaque block type.
    pub type Block = generic::Block<Header, UncheckedExtrinsic>;
    /// Opaque block identifier type.
    pub type BlockId = generic::BlockId<Block>;
    /// Opaque session key type.
    pub type SessionKey = AuthorityId;
}

/// This runtime version.
pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: { ::std::borrow::Cow::Borrowed("template-node") },
    impl_name: { ::std::borrow::Cow::Borrowed("template-node") },
    authoring_version: 3,
    spec_version: 3,
    impl_version: 0,
    apis: RUNTIME_API_VERSIONS,
};

/// The version infromation used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
    NativeVersion {
        runtime_version: VERSION,
        can_author_with: Default::default(),
    }
}

impl system::Trait for Runtime {
    /// The identifier used to distinguish between accounts.
    type AccountId = AccountId;
    /// The lookup mechanism to get account ID from whatever is passed in dispatchers.
    type Lookup = Indices;
    /// The index type for storing how many extrinsics an account has signed.
    type Index = Nonce;
    /// The index type for blocks.
    type BlockNumber = BlockNumber;
    /// The type for hashing blocks and tries.
    type Hash = Hash;
    /// The hashing algorithm used.
    type Hashing = BlakeTwo256;
    /// The header digest type.
    type Digest = generic::Digest<Log>;
    /// The header type.
    type Header = generic::Header<BlockNumber, BlakeTwo256, Log>;
    /// The ubiquitous event type.
    type Event = Event;
    /// The ubiquitous log type.
    type Log = Log;
    /// The ubiquitous origin type.
    type Origin = Origin;
}

impl aura::Trait for Runtime {
    type HandleReport = ();
}

impl consensus::Trait for Runtime {
    /// The identifier we use to refer to authorities.
    type SessionKey = AuthorityId;
    type InherentOfflineReport = ();
    /// The ubiquitous log type.
    type Log = Log;
}

impl indices::Trait for Runtime {
    /// The type for recording indexing into the account enumeration. If this ever overflows, there
    /// will be problems!
    type AccountIndex = u32;
    /// Use the standard means of resolving an index hint from an id.
    type ResolveHint = indices::SimpleResolveHint<Self::AccountId, Self::AccountIndex>;
    /// Determine whether an account is dead.
    type IsDeadAccount = Balances;
    /// The uniquitous event type.
    type Event = Event;
}

impl timestamp::Trait for Runtime {
    /// A timestamp: seconds since the unix epoch.
    type Moment = u64;
    type OnTimestampSet = Aura;
}

impl balances::Trait for Runtime {
    /// The type for recording an account's balance.
    type Balance = u128;
    /// What to do if an account's free balance gets zeroed.
    type OnFreeBalanceZero = ();
    /// What to do if a new account is created.
    type OnNewAccount = Indices;
    /// The uniquitous event type.
    type Event = Event;
    type TransactionPayment = ();
    type DustRemoval = ();
    type TransferPayment = ();
}

impl sudo::Trait for Runtime {
    /// The uniquitous event type.
    type Event = Event;
    type Proposal = Call;
}

/// Used for the module template in `./template.rs`
impl template::Trait for Runtime {
    type Event = Event;
}

impl realestate::Trait for Runtime {
    type Event = Event;
}

#[structural_match]
#[rustc_copy_clone_marker]
pub struct Runtime;

#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::clone::Clone for Runtime {
    #[inline]
    fn clone(&self) -> Runtime {
        {
            *self
        }
    }
}

#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::marker::Copy for Runtime {}

#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::PartialEq for Runtime {
    #[inline]
    fn eq(&self, other: &Runtime) -> bool {
        match *other {
            Runtime => match *self {
                Runtime => true,
            },
        }
    }
}

#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::Eq for Runtime {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {}
    }
}

#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::fmt::Debug for Runtime {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Runtime => {
                let mut debug_trait_builder = f.debug_tuple("Runtime");
                debug_trait_builder.finish()
            }
        }
    }
}

impl ::srml_support::runtime_primitives::traits::GetNodeBlockType for Runtime {
    type NodeBlock = opaque::Block;
}

impl ::srml_support::runtime_primitives::traits::GetRuntimeBlockType for Runtime {
    type RuntimeBlock = Block;
}

#[allow(non_camel_case_types)]
#[structural_match]
pub enum Event {
    system(system::Event),
    indices(indices::Event<Runtime>),
    balances(balances::Event<Runtime>),
    sudo(sudo::Event<Runtime>),
    template(template::Event<Runtime>),
    realestate(realestate::Event<Runtime>),
}

#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(non_camel_case_types)]
impl ::std::clone::Clone for Event {
    #[inline]
    fn clone(&self) -> Event {
        match (&*self, ) {
            (&Event::system(ref __self_0), ) => {
                Event::system(::std::clone::Clone::clone(&(*__self_0)))
            }
            (&Event::indices(ref __self_0), ) => {
                Event::indices(::std::clone::Clone::clone(&(*__self_0)))
            }
            (&Event::balances(ref __self_0), ) => {
                Event::balances(::std::clone::Clone::clone(&(*__self_0)))
            }
            (&Event::sudo(ref __self_0), ) => Event::sudo(::std::clone::Clone::clone(&(*__self_0))),
            (&Event::template(ref __self_0), ) => {
                Event::template(::std::clone::Clone::clone(&(*__self_0)))
            }
            (&Event::realestate(ref __self_0), ) => {
                Event::realestate(::std::clone::Clone::clone(&(*__self_0)))
            }
        }
    }
}

#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(non_camel_case_types)]
impl ::std::cmp::PartialEq for Event {
    #[inline]
    fn eq(&self, other: &Event) -> bool {
        {
            let __self_vi = unsafe { ::std::intrinsics::discriminant_value(&*self) } as isize;
            let __arg_1_vi = unsafe { ::std::intrinsics::discriminant_value(&*other) } as isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&Event::system(ref __self_0), &Event::system(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (&Event::indices(ref __self_0), &Event::indices(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (&Event::balances(ref __self_0), &Event::balances(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (&Event::sudo(ref __self_0), &Event::sudo(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (&Event::template(ref __self_0), &Event::template(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (&Event::realestate(ref __self_0), &Event::realestate(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    _ => unsafe { ::std::intrinsics::unreachable() },
                }
            } else {
                false
            }
        }
    }
    #[inline]
    fn ne(&self, other: &Event) -> bool {
        {
            let __self_vi = unsafe { ::std::intrinsics::discriminant_value(&*self) } as isize;
            let __arg_1_vi = unsafe { ::std::intrinsics::discriminant_value(&*other) } as isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&Event::system(ref __self_0), &Event::system(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (&Event::indices(ref __self_0), &Event::indices(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (&Event::balances(ref __self_0), &Event::balances(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (&Event::sudo(ref __self_0), &Event::sudo(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (&Event::template(ref __self_0), &Event::template(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (&Event::realestate(ref __self_0), &Event::realestate(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    _ => unsafe { ::std::intrinsics::unreachable() },
                }
            } else {
                true
            }
        }
    }
}

#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(non_camel_case_types)]
impl ::std::cmp::Eq for Event {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::std::cmp::AssertParamIsEq<system::Event>;
            let _: ::std::cmp::AssertParamIsEq<indices::Event<Runtime>>;
            let _: ::std::cmp::AssertParamIsEq<balances::Event<Runtime>>;
            let _: ::std::cmp::AssertParamIsEq<sudo::Event<Runtime>>;
            let _: ::std::cmp::AssertParamIsEq<template::Event<Runtime>>;
            let _: ::std::cmp::AssertParamIsEq<realestate::Event<Runtime>>;
        }
    }
}

#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_Event: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate parity_codec as _parity_codec;
    impl _parity_codec::Encode for Event {
        fn encode_to<EncOut: _parity_codec::Output>(&self, dest: &mut EncOut) {
            match *self {
                Event::system(ref aa) => {
                    dest.push_byte(0usize as u8);
                    dest.push(aa);
                }
                Event::indices(ref aa) => {
                    dest.push_byte(1usize as u8);
                    dest.push(aa);
                }
                Event::balances(ref aa) => {
                    dest.push_byte(2usize as u8);
                    dest.push(aa);
                }
                Event::sudo(ref aa) => {
                    dest.push_byte(3usize as u8);
                    dest.push(aa);
                }
                Event::template(ref aa) => {
                    dest.push_byte(4usize as u8);
                    dest.push(aa);
                }
                Event::realestate(ref aa) => {
                    dest.push_byte(5usize as u8);
                    dest.push(aa);
                }
                _ => (),
            }
        }
    }
};
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DECODE_FOR_Event: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate parity_codec as _parity_codec;
    impl _parity_codec::Decode for Event {
        fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn) -> Option<Self> {
            match input.read_byte()? {
                x if x == 0usize as u8 => {
                    Some(Event::system(_parity_codec::Decode::decode(input)?))
                }
                x if x == 1usize as u8 => {
                    Some(Event::indices(_parity_codec::Decode::decode(input)?))
                }
                x if x == 2usize as u8 => {
                    Some(Event::balances(_parity_codec::Decode::decode(input)?))
                }
                x if x == 3usize as u8 => Some(Event::sudo(_parity_codec::Decode::decode(input)?)),
                x if x == 4usize as u8 => {
                    Some(Event::template(_parity_codec::Decode::decode(input)?))
                }
                x if x == 5usize as u8 => {
                    Some(Event::realestate(_parity_codec::Decode::decode(input)?))
                }
                _ => None,
            }
        }
    }
};

#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(non_camel_case_types)]
impl ::std::fmt::Debug for Event {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match (&*self, ) {
            (&Event::system(ref __self_0), ) => {
                let mut debug_trait_builder = f.debug_tuple("system");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&Event::indices(ref __self_0), ) => {
                let mut debug_trait_builder = f.debug_tuple("indices");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&Event::balances(ref __self_0), ) => {
                let mut debug_trait_builder = f.debug_tuple("balances");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&Event::sudo(ref __self_0), ) => {
                let mut debug_trait_builder = f.debug_tuple("sudo");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&Event::template(ref __self_0), ) => {
                let mut debug_trait_builder = f.debug_tuple("template");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&Event::realestate(ref __self_0), ) => {
                let mut debug_trait_builder = f.debug_tuple("realestate");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
        }
    }
}

impl From<system::Event> for Event {
    fn from(x: system::Event) -> Self {
        Event::system(x)
    }
}

impl From<indices::Event<Runtime>> for Event {
    fn from(x: indices::Event<Runtime>) -> Self {
        Event::indices(x)
    }
}

impl From<balances::Event<Runtime>> for Event {
    fn from(x: balances::Event<Runtime>) -> Self {
        Event::balances(x)
    }
}

impl From<sudo::Event<Runtime>> for Event {
    fn from(x: sudo::Event<Runtime>) -> Self {
        Event::sudo(x)
    }
}

impl From<template::Event<Runtime>> for Event {
    fn from(x: template::Event<Runtime>) -> Self {
        Event::template(x)
    }
}

impl From<realestate::Event<Runtime>> for Event {
    fn from(x: realestate::Event<Runtime>) -> Self {
        Event::realestate(x)
    }
}

impl Runtime {
    #[allow(dead_code)]
    pub fn outer_event_metadata() -> ::srml_support::event::OuterEventMetadata {
        ::srml_support::event::OuterEventMetadata {
            name: ::srml_support::event::DecodeDifferent::Encode("Event"),
            events: ::srml_support::event::DecodeDifferent::Encode(&[
                (
                    "system",
                    ::srml_support::event::FnEncode(system::Event::metadata),
                ),
                (
                    "indices",
                    ::srml_support::event::FnEncode(indices::Event::<Runtime>::metadata),
                ),
                (
                    "balances",
                    ::srml_support::event::FnEncode(balances::Event::<Runtime>::metadata),
                ),
                (
                    "sudo",
                    ::srml_support::event::FnEncode(sudo::Event::<Runtime>::metadata),
                ),
                (
                    "template",
                    ::srml_support::event::FnEncode(template::Event::<Runtime>::metadata),
                ),
                (
                    "realestate",
                    ::srml_support::event::FnEncode(realestate::Event::<Runtime>::metadata),
                ),
            ]),
        }
    }
    #[allow(dead_code)]
    pub fn __module_events_system() -> &'static [::srml_support::event::EventMetadata] {
        system::Event::metadata()
    }
    pub fn __module_events_indices() -> &'static [::srml_support::event::EventMetadata] {
        indices::Event::<Runtime>::metadata()
    }
    pub fn __module_events_balances() -> &'static [::srml_support::event::EventMetadata] {
        balances::Event::<Runtime>::metadata()
    }
    pub fn __module_events_sudo() -> &'static [::srml_support::event::EventMetadata] {
        sudo::Event::<Runtime>::metadata()
    }
    pub fn __module_events_template() -> &'static [::srml_support::event::EventMetadata] {
        template::Event::<Runtime>::metadata()
    }
    pub fn __module_events_realestate() -> &'static [::srml_support::event::EventMetadata] {
        realestate::Event::<Runtime>::metadata()
    }
}

#[allow(non_camel_case_types)]
#[structural_match]
pub enum Origin {
    system(system::Origin<Runtime>),
    #[allow(dead_code)]
    Void(::srml_support::Void),
}

#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(non_camel_case_types)]
impl ::std::clone::Clone for Origin {
    #[inline]
    fn clone(&self) -> Origin {
        match (&*self, ) {
            (&Origin::system(ref __self_0), ) => {
                Origin::system(::std::clone::Clone::clone(&(*__self_0)))
            }
            (&Origin::Void(ref __self_0), ) => {
                Origin::Void(::std::clone::Clone::clone(&(*__self_0)))
            }
        }
    }
}

#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(non_camel_case_types)]
impl ::std::cmp::PartialEq for Origin {
    #[inline]
    fn eq(&self, other: &Origin) -> bool {
        {
            let __self_vi = unsafe { ::std::intrinsics::discriminant_value(&*self) } as isize;
            let __arg_1_vi = unsafe { ::std::intrinsics::discriminant_value(&*other) } as isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&Origin::system(ref __self_0), &Origin::system(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (&Origin::Void(ref __self_0), &Origin::Void(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    _ => unsafe { ::std::intrinsics::unreachable() },
                }
            } else {
                false
            }
        }
    }
    #[inline]
    fn ne(&self, other: &Origin) -> bool {
        {
            let __self_vi = unsafe { ::std::intrinsics::discriminant_value(&*self) } as isize;
            let __arg_1_vi = unsafe { ::std::intrinsics::discriminant_value(&*other) } as isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&Origin::system(ref __self_0), &Origin::system(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (&Origin::Void(ref __self_0), &Origin::Void(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    _ => unsafe { ::std::intrinsics::unreachable() },
                }
            } else {
                true
            }
        }
    }
}

#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(non_camel_case_types)]
impl ::std::cmp::Eq for Origin {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::std::cmp::AssertParamIsEq<system::Origin<Runtime>>;
            let _: ::std::cmp::AssertParamIsEq<::srml_support::Void>;
        }
    }
}

#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(non_camel_case_types)]
impl ::std::fmt::Debug for Origin {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match (&*self, ) {
            (&Origin::system(ref __self_0), ) => {
                let mut debug_trait_builder = f.debug_tuple("system");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&Origin::Void(ref __self_0), ) => {
                let mut debug_trait_builder = f.debug_tuple("Void");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
        }
    }
}

#[allow(dead_code)]
impl Origin {
    pub const INHERENT: Self = Origin::system(system::RawOrigin::Inherent);
    pub const ROOT: Self = Origin::system(system::RawOrigin::Root);
    pub fn signed(by: <Runtime as system::Trait>::AccountId) -> Self {
        Origin::system(system::RawOrigin::Signed(by))
    }
}

impl From<system::Origin<Runtime>> for Origin {
    fn from(x: system::Origin<Runtime>) -> Self {
        Origin::system(x)
    }
}

impl Into<Option<system::Origin<Runtime>>> for Origin {
    fn into(self) -> Option<system::Origin<Runtime>> {
        if let Origin::system(l) = self {
            Some(l)
        } else {
            None
        }
    }
}

impl From<Option<<Runtime as system::Trait>::AccountId>> for Origin {
    fn from(x: Option<<Runtime as system::Trait>::AccountId>) -> Self {
        <system::Origin<Runtime>>::from(x).into()
    }
}

pub type System = system::Module<Runtime>;
pub type Timestamp = timestamp::Module<Runtime>;
pub type Consensus = consensus::Module<Runtime>;
pub type Aura = aura::Module<Runtime>;
pub type Indices = indices::Module<Runtime>;
pub type Balances = balances::Module<Runtime>;
pub type Sudo = sudo::Module<Runtime>;
pub type TemplateModule = template::Module<Runtime>;
pub type RealEstateModule = realestate::Module<Runtime>;
type AllModules = (
    Timestamp,
    Consensus,
    Aura,
    Indices,
    Balances,
    Sudo,
    TemplateModule,
    RealEstateModule,
);

#[structural_match]
pub enum Call {
    Timestamp(::srml_support::dispatch::CallableCallFor<Timestamp>),
    Consensus(::srml_support::dispatch::CallableCallFor<Consensus>),
    Indices(::srml_support::dispatch::CallableCallFor<Indices>),
    Balances(::srml_support::dispatch::CallableCallFor<Balances>),
    Sudo(::srml_support::dispatch::CallableCallFor<Sudo>),
    TemplateModule(::srml_support::dispatch::CallableCallFor<TemplateModule>),
    RealEstateModule(::srml_support::dispatch::CallableCallFor<RealEstateModule>),
}

#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::clone::Clone for Call {
    #[inline]
    fn clone(&self) -> Call {
        match (&*self, ) {
            (&Call::Timestamp(ref __self_0), ) => {
                Call::Timestamp(::std::clone::Clone::clone(&(*__self_0)))
            }
            (&Call::Consensus(ref __self_0), ) => {
                Call::Consensus(::std::clone::Clone::clone(&(*__self_0)))
            }
            (&Call::Indices(ref __self_0), ) => {
                Call::Indices(::std::clone::Clone::clone(&(*__self_0)))
            }
            (&Call::Balances(ref __self_0), ) => {
                Call::Balances(::std::clone::Clone::clone(&(*__self_0)))
            }
            (&Call::Sudo(ref __self_0), ) => Call::Sudo(::std::clone::Clone::clone(&(*__self_0))),
            (&Call::TemplateModule(ref __self_0), ) => {
                Call::TemplateModule(::std::clone::Clone::clone(&(*__self_0)))
            }
            (&Call::RealEstateModule(ref __self_0), ) => {
                Call::RealEstateModule(::std::clone::Clone::clone(&(*__self_0)))
            }
        }
    }
}

#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::PartialEq for Call {
    #[inline]
    fn eq(&self, other: &Call) -> bool {
        {
            let __self_vi = unsafe { ::std::intrinsics::discriminant_value(&*self) } as isize;
            let __arg_1_vi = unsafe { ::std::intrinsics::discriminant_value(&*other) } as isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&Call::Timestamp(ref __self_0), &Call::Timestamp(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (&Call::Consensus(ref __self_0), &Call::Consensus(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (&Call::Indices(ref __self_0), &Call::Indices(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (&Call::Balances(ref __self_0), &Call::Balances(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (&Call::Sudo(ref __self_0), &Call::Sudo(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (&Call::TemplateModule(ref __self_0), &Call::TemplateModule(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (
                        &Call::RealEstateModule(ref __self_0),
                        &Call::RealEstateModule(ref __arg_1_0),
                    ) => (*__self_0) == (*__arg_1_0),
                    _ => unsafe { ::std::intrinsics::unreachable() },
                }
            } else {
                false
            }
        }
    }
    #[inline]
    fn ne(&self, other: &Call) -> bool {
        {
            let __self_vi = unsafe { ::std::intrinsics::discriminant_value(&*self) } as isize;
            let __arg_1_vi = unsafe { ::std::intrinsics::discriminant_value(&*other) } as isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&Call::Timestamp(ref __self_0), &Call::Timestamp(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (&Call::Consensus(ref __self_0), &Call::Consensus(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (&Call::Indices(ref __self_0), &Call::Indices(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (&Call::Balances(ref __self_0), &Call::Balances(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (&Call::Sudo(ref __self_0), &Call::Sudo(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (&Call::TemplateModule(ref __self_0), &Call::TemplateModule(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (
                        &Call::RealEstateModule(ref __self_0),
                        &Call::RealEstateModule(ref __arg_1_0),
                    ) => (*__self_0) != (*__arg_1_0),
                    _ => unsafe { ::std::intrinsics::unreachable() },
                }
            } else {
                true
            }
        }
    }
}

#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::Eq for Call {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::std::cmp::AssertParamIsEq<
                ::srml_support::dispatch::CallableCallFor<Timestamp>,
            >;
            let _: ::std::cmp::AssertParamIsEq<
                ::srml_support::dispatch::CallableCallFor<Consensus>,
            >;
            let _: ::std::cmp::AssertParamIsEq<::srml_support::dispatch::CallableCallFor<Indices>>;
            let _: ::std::cmp::AssertParamIsEq<
                ::srml_support::dispatch::CallableCallFor<Balances>,
            >;
            let _: ::std::cmp::AssertParamIsEq<::srml_support::dispatch::CallableCallFor<Sudo>>;
            let _: ::std::cmp::AssertParamIsEq<
                ::srml_support::dispatch::CallableCallFor<TemplateModule>,
            >;
            let _: ::std::cmp::AssertParamIsEq<
                ::srml_support::dispatch::CallableCallFor<RealEstateModule>,
            >;
        }
    }
}

#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::fmt::Debug for Call {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match (&*self, ) {
            (&Call::Timestamp(ref __self_0), ) => {
                let mut debug_trait_builder = f.debug_tuple("Timestamp");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&Call::Consensus(ref __self_0), ) => {
                let mut debug_trait_builder = f.debug_tuple("Consensus");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&Call::Indices(ref __self_0), ) => {
                let mut debug_trait_builder = f.debug_tuple("Indices");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&Call::Balances(ref __self_0), ) => {
                let mut debug_trait_builder = f.debug_tuple("Balances");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&Call::Sudo(ref __self_0), ) => {
                let mut debug_trait_builder = f.debug_tuple("Sudo");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&Call::TemplateModule(ref __self_0), ) => {
                let mut debug_trait_builder = f.debug_tuple("TemplateModule");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&Call::RealEstateModule(ref __self_0), ) => {
                let mut debug_trait_builder = f.debug_tuple("RealEstateModule");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
        }
    }
}

impl ::srml_support::dispatch::Decode for Call {
    fn decode<I: ::srml_support::dispatch::Input>(input: &mut I) -> Option<Self> {
        let input_id = input.read_byte()?;
        {
            if input_id == (0) {
                let outer_dispatch_param = ::srml_support::dispatch::Decode::decode(input)?;
                return Some(Call::Timestamp(outer_dispatch_param));
            }
            {
                if input_id == (0 + 1) {
                    let outer_dispatch_param = ::srml_support::dispatch::Decode::decode(input)?;
                    return Some(Call::Consensus(outer_dispatch_param));
                }
                {
                    if input_id == (0 + 1 + 1) {
                        let outer_dispatch_param = ::srml_support::dispatch::Decode::decode(input)?;
                        return Some(Call::Indices(outer_dispatch_param));
                    }
                    {
                        if input_id == (0 + 1 + 1 + 1) {
                            let outer_dispatch_param =
                                ::srml_support::dispatch::Decode::decode(input)?;
                            return Some(Call::Balances(outer_dispatch_param));
                        }
                        {
                            if input_id == (0 + 1 + 1 + 1 + 1) {
                                let outer_dispatch_param =
                                    ::srml_support::dispatch::Decode::decode(input)?;
                                return Some(Call::Sudo(outer_dispatch_param));
                            }
                            {
                                if input_id == (0 + 1 + 1 + 1 + 1 + 1) {
                                    let outer_dispatch_param =
                                        ::srml_support::dispatch::Decode::decode(input)?;
                                    return Some(Call::TemplateModule(outer_dispatch_param));
                                }
                                {
                                    if input_id == (0 + 1 + 1 + 1 + 1 + 1 + 1) {
                                        let outer_dispatch_param =
                                            ::srml_support::dispatch::Decode::decode(input)?;
                                        return Some(Call::RealEstateModule(outer_dispatch_param));
                                    }
                                    None
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

impl ::srml_support::dispatch::Encode for Call {
    fn encode_to<W: ::srml_support::dispatch::Output>(&self, dest: &mut W) {
        {
            if let Call::Timestamp(ref outer_dispatch_param) = *self {
                dest.push_byte((0) as u8);
                outer_dispatch_param.encode_to(dest);
            }
            {
                if let Call::Consensus(ref outer_dispatch_param) = *self {
                    dest.push_byte((0 + 1) as u8);
                    outer_dispatch_param.encode_to(dest);
                }
                {
                    if let Call::Indices(ref outer_dispatch_param) = *self {
                        dest.push_byte((0 + 1 + 1) as u8);
                        outer_dispatch_param.encode_to(dest);
                    }
                    {
                        if let Call::Balances(ref outer_dispatch_param) = *self {
                            dest.push_byte((0 + 1 + 1 + 1) as u8);
                            outer_dispatch_param.encode_to(dest);
                        }
                        {
                            if let Call::Sudo(ref outer_dispatch_param) = *self {
                                dest.push_byte((0 + 1 + 1 + 1 + 1) as u8);
                                outer_dispatch_param.encode_to(dest);
                            }
                            {
                                if let Call::TemplateModule(ref outer_dispatch_param) = *self {
                                    dest.push_byte((0 + 1 + 1 + 1 + 1 + 1) as u8);
                                    outer_dispatch_param.encode_to(dest);
                                }
                                {
                                    if let Call::RealEstateModule(ref outer_dispatch_param) = *self
                                    {
                                        dest.push_byte((0 + 1 + 1 + 1 + 1 + 1 + 1) as u8);
                                        outer_dispatch_param.encode_to(dest);
                                    }
                                    {}
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

impl ::srml_support::dispatch::Dispatchable for Call {
    type Origin = Origin;
    type Trait = Call;
    fn dispatch(self, origin: Origin) -> ::srml_support::dispatch::Result {
        match self {
            Call::Timestamp(call) => call.dispatch(origin),
            Call::Consensus(call) => call.dispatch(origin),
            Call::Indices(call) => call.dispatch(origin),
            Call::Balances(call) => call.dispatch(origin),
            Call::Sudo(call) => call.dispatch(origin),
            Call::TemplateModule(call) => call.dispatch(origin),
            Call::RealEstateModule(call) => call.dispatch(origin),
        }
    }
}

impl ::srml_support::dispatch::IsSubType<Timestamp> for Call {
    fn is_aux_sub_type(&self) -> Option<&<Timestamp as ::srml_support::dispatch::Callable>::Call> {
        if let Call::Timestamp(ref r) = *self {
            Some(r)
        } else {
            None
        }
    }
}

impl ::srml_support::dispatch::IsSubType<Consensus> for Call {
    fn is_aux_sub_type(&self) -> Option<&<Consensus as ::srml_support::dispatch::Callable>::Call> {
        if let Call::Consensus(ref r) = *self {
            Some(r)
        } else {
            None
        }
    }
}

impl ::srml_support::dispatch::IsSubType<Indices> for Call {
    fn is_aux_sub_type(&self) -> Option<&<Indices as ::srml_support::dispatch::Callable>::Call> {
        if let Call::Indices(ref r) = *self {
            Some(r)
        } else {
            None
        }
    }
}

impl ::srml_support::dispatch::IsSubType<Balances> for Call {
    fn is_aux_sub_type(&self) -> Option<&<Balances as ::srml_support::dispatch::Callable>::Call> {
        if let Call::Balances(ref r) = *self {
            Some(r)
        } else {
            None
        }
    }
}

impl ::srml_support::dispatch::IsSubType<Sudo> for Call {
    fn is_aux_sub_type(&self) -> Option<&<Sudo as ::srml_support::dispatch::Callable>::Call> {
        if let Call::Sudo(ref r) = *self {
            Some(r)
        } else {
            None
        }
    }
}

impl ::srml_support::dispatch::IsSubType<TemplateModule> for Call {
    fn is_aux_sub_type(
        &self,
    ) -> Option<&<TemplateModule as ::srml_support::dispatch::Callable>::Call> {
        if let Call::TemplateModule(ref r) = *self {
            Some(r)
        } else {
            None
        }
    }
}

impl ::srml_support::dispatch::IsSubType<RealEstateModule> for Call {
    fn is_aux_sub_type(
        &self,
    ) -> Option<&<RealEstateModule as ::srml_support::dispatch::Callable>::Call> {
        if let Call::RealEstateModule(ref r) = *self {
            Some(r)
        } else {
            None
        }
    }
}

impl Runtime {
    pub fn metadata() -> ::srml_support::metadata::RuntimeMetadataPrefixed {
        ::srml_support::metadata::RuntimeMetadata::V3(::srml_support::metadata::RuntimeMetadataV3 {
            modules: ::srml_support::metadata::DecodeDifferent::Encode(&[
                ::srml_support::metadata::ModuleMetadata {
                    name: ::srml_support::metadata::DecodeDifferent::Encode("system"),
                    prefix: ::srml_support::metadata::DecodeDifferent::Encode(
                        ::srml_support::metadata::FnEncode(
                            system::Module::<Runtime>::store_metadata_name,
                        ),
                    ),
                    storage: Some(::srml_support::metadata::DecodeDifferent::Encode(
                        ::srml_support::metadata::FnEncode(
                            system::Module::<Runtime>::store_metadata_functions,
                        ),
                    )),
                    calls: None,
                    event: Some(::srml_support::metadata::DecodeDifferent::Encode(
                        ::srml_support::metadata::FnEncode({
                            enum ProcMacroHack {
                                Value = ("Runtime :: [ < __module_events_ system > ]", 0).1,
                            }
                            {
                                Runtime::__module_events_system
                            }
                        }),
                    )),
                },
                ::srml_support::metadata::ModuleMetadata {
                    name: ::srml_support::metadata::DecodeDifferent::Encode("timestamp"),
                    prefix: ::srml_support::metadata::DecodeDifferent::Encode(
                        ::srml_support::metadata::FnEncode(
                            timestamp::Module::<Runtime>::store_metadata_name,
                        ),
                    ),
                    storage: Some(::srml_support::metadata::DecodeDifferent::Encode(
                        ::srml_support::metadata::FnEncode(
                            timestamp::Module::<Runtime>::store_metadata_functions,
                        ),
                    )),
                    calls: Some(::srml_support::metadata::DecodeDifferent::Encode(
                        ::srml_support::metadata::FnEncode(
                            timestamp::Module::<Runtime>::call_functions,
                        ),
                    )),
                    event: None,
                },
                ::srml_support::metadata::ModuleMetadata {
                    name: ::srml_support::metadata::DecodeDifferent::Encode("consensus"),
                    prefix: ::srml_support::metadata::DecodeDifferent::Encode(
                        ::srml_support::metadata::FnEncode(
                            consensus::Module::<Runtime>::store_metadata_name,
                        ),
                    ),
                    storage: Some(::srml_support::metadata::DecodeDifferent::Encode(
                        ::srml_support::metadata::FnEncode(
                            consensus::Module::<Runtime>::store_metadata_functions,
                        ),
                    )),
                    calls: Some(::srml_support::metadata::DecodeDifferent::Encode(
                        ::srml_support::metadata::FnEncode(
                            consensus::Module::<Runtime>::call_functions,
                        ),
                    )),
                    event: None,
                },
                ::srml_support::metadata::ModuleMetadata {
                    name: ::srml_support::metadata::DecodeDifferent::Encode("aura"),
                    prefix: ::srml_support::metadata::DecodeDifferent::Encode(
                        ::srml_support::metadata::FnEncode(|| ""),
                    ),
                    storage: None,
                    calls: None,
                    event: None,
                },
                ::srml_support::metadata::ModuleMetadata {
                    name: ::srml_support::metadata::DecodeDifferent::Encode("indices"),
                    prefix: ::srml_support::metadata::DecodeDifferent::Encode(
                        ::srml_support::metadata::FnEncode(
                            indices::Module::<Runtime>::store_metadata_name,
                        ),
                    ),
                    storage: Some(::srml_support::metadata::DecodeDifferent::Encode(
                        ::srml_support::metadata::FnEncode(
                            indices::Module::<Runtime>::store_metadata_functions,
                        ),
                    )),
                    calls: Some(::srml_support::metadata::DecodeDifferent::Encode(
                        ::srml_support::metadata::FnEncode(
                            indices::Module::<Runtime>::call_functions,
                        ),
                    )),
                    event: Some(::srml_support::metadata::DecodeDifferent::Encode(
                        ::srml_support::metadata::FnEncode({
                            enum ProcMacroHack {
                                Value = ("Runtime :: [ < __module_events_ indices > ]", 0).1,
                            }
                            {
                                Runtime::__module_events_indices
                            }
                        }),
                    )),
                },
                ::srml_support::metadata::ModuleMetadata {
                    name: ::srml_support::metadata::DecodeDifferent::Encode("balances"),
                    prefix: ::srml_support::metadata::DecodeDifferent::Encode(
                        ::srml_support::metadata::FnEncode(
                            balances::Module::<Runtime>::store_metadata_name,
                        ),
                    ),
                    storage: Some(::srml_support::metadata::DecodeDifferent::Encode(
                        ::srml_support::metadata::FnEncode(
                            balances::Module::<Runtime>::store_metadata_functions,
                        ),
                    )),
                    calls: Some(::srml_support::metadata::DecodeDifferent::Encode(
                        ::srml_support::metadata::FnEncode(
                            balances::Module::<Runtime>::call_functions,
                        ),
                    )),
                    event: Some(::srml_support::metadata::DecodeDifferent::Encode(
                        ::srml_support::metadata::FnEncode({
                            enum ProcMacroHack {
                                Value = ("Runtime :: [ < __module_events_ balances > ]", 0).1,
                            }
                            {
                                Runtime::__module_events_balances
                            }
                        }),
                    )),
                },
                ::srml_support::metadata::ModuleMetadata {
                    name: ::srml_support::metadata::DecodeDifferent::Encode("sudo"),
                    prefix: ::srml_support::metadata::DecodeDifferent::Encode(
                        ::srml_support::metadata::FnEncode(
                            sudo::Module::<Runtime>::store_metadata_name,
                        ),
                    ),
                    storage: Some(::srml_support::metadata::DecodeDifferent::Encode(
                        ::srml_support::metadata::FnEncode(
                            sudo::Module::<Runtime>::store_metadata_functions,
                        ),
                    )),
                    calls: Some(::srml_support::metadata::DecodeDifferent::Encode(
                        ::srml_support::metadata::FnEncode(sudo::Module::<Runtime>::call_functions),
                    )),
                    event: Some(::srml_support::metadata::DecodeDifferent::Encode(
                        ::srml_support::metadata::FnEncode({
                            enum ProcMacroHack {
                                Value = ("Runtime :: [ < __module_events_ sudo > ]", 0).1,
                            }
                            {
                                Runtime::__module_events_sudo
                            }
                        }),
                    )),
                },
                ::srml_support::metadata::ModuleMetadata {
                    name: ::srml_support::metadata::DecodeDifferent::Encode("template"),
                    prefix: ::srml_support::metadata::DecodeDifferent::Encode(
                        ::srml_support::metadata::FnEncode(
                            template::Module::<Runtime>::store_metadata_name,
                        ),
                    ),
                    storage: Some(::srml_support::metadata::DecodeDifferent::Encode(
                        ::srml_support::metadata::FnEncode(
                            template::Module::<Runtime>::store_metadata_functions,
                        ),
                    )),
                    calls: Some(::srml_support::metadata::DecodeDifferent::Encode(
                        ::srml_support::metadata::FnEncode(
                            template::Module::<Runtime>::call_functions,
                        ),
                    )),
                    event: Some(::srml_support::metadata::DecodeDifferent::Encode(
                        ::srml_support::metadata::FnEncode({
                            enum ProcMacroHack {
                                Value = ("Runtime :: [ < __module_events_ template > ]", 0).1,
                            }
                            {
                                Runtime::__module_events_template
                            }
                        }),
                    )),
                },
                ::srml_support::metadata::ModuleMetadata {
                    name: ::srml_support::metadata::DecodeDifferent::Encode("realestate"),
                    prefix: ::srml_support::metadata::DecodeDifferent::Encode(
                        ::srml_support::metadata::FnEncode(
                            realestate::Module::<Runtime>::store_metadata_name,
                        ),
                    ),
                    storage: Some(::srml_support::metadata::DecodeDifferent::Encode(
                        ::srml_support::metadata::FnEncode(
                            realestate::Module::<Runtime>::store_metadata_functions,
                        ),
                    )),
                    calls: Some(::srml_support::metadata::DecodeDifferent::Encode(
                        ::srml_support::metadata::FnEncode(
                            realestate::Module::<Runtime>::call_functions,
                        ),
                    )),
                    event: Some(::srml_support::metadata::DecodeDifferent::Encode(
                        ::srml_support::metadata::FnEncode({
                            enum ProcMacroHack {
                                Value = ("Runtime :: [ < __module_events_ realestate > ]", 0).1,
                            }
                            {
                                Runtime::__module_events_realestate
                            }
                        }),
                    )),
                },
            ]),
        })
            .into()
    }
}

/// Wrapper for all possible log entries for the `$trait` runtime. Provides binary-compatible
/// `Encode`/`Decode` implementations with the corresponding `generic::DigestItem`.
#[allow(non_camel_case_types)]
#[structural_match]
pub struct Log(InternalLog);

#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(non_camel_case_types)]
impl ::std::clone::Clone for Log {
    #[inline]
    fn clone(&self) -> Log {
        match *self {
            Log(ref __self_0_0) => Log(::std::clone::Clone::clone(&(*__self_0_0))),
        }
    }
}

#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(non_camel_case_types)]
impl ::std::cmp::PartialEq for Log {
    #[inline]
    fn eq(&self, other: &Log) -> bool {
        match *other {
            Log(ref __self_1_0) => match *self {
                Log(ref __self_0_0) => (*__self_0_0) == (*__self_1_0),
            },
        }
    }
    #[inline]
    fn ne(&self, other: &Log) -> bool {
        match *other {
            Log(ref __self_1_0) => match *self {
                Log(ref __self_0_0) => (*__self_0_0) != (*__self_1_0),
            },
        }
    }
}

#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(non_camel_case_types)]
impl ::std::cmp::Eq for Log {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::std::cmp::AssertParamIsEq<InternalLog>;
        }
    }
}

#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(non_camel_case_types)]
impl ::std::fmt::Debug for Log {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Log(ref __self_0_0) => {
                let mut debug_trait_builder = f.debug_tuple("Log");
                let _ = debug_trait_builder.field(&&(*__self_0_0));
                debug_trait_builder.finish()
            }
        }
    }
}

#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_Log: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for Log {
        fn serialize<__S>(&self, __serializer: __S) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
        {
            _serde::Serializer::serialize_newtype_struct(__serializer, "Log", &self.0)
        }
    }
};

/// All possible log entries for the `$trait` runtime. `Encode`/`Decode` implementations
/// are auto-generated => it is not binary-compatible with `generic::DigestItem`.
#[allow(non_camel_case_types)]
#[structural_match]
pub enum InternalLog {
    system(system::Log<Runtime>),
    consensus(consensus::Log<Runtime>),
}

#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(non_camel_case_types)]
impl ::std::clone::Clone for InternalLog {
    #[inline]
    fn clone(&self) -> InternalLog {
        match (&*self, ) {
            (&InternalLog::system(ref __self_0), ) => {
                InternalLog::system(::std::clone::Clone::clone(&(*__self_0)))
            }
            (&InternalLog::consensus(ref __self_0), ) => {
                InternalLog::consensus(::std::clone::Clone::clone(&(*__self_0)))
            }
        }
    }
}

#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(non_camel_case_types)]
impl ::std::cmp::PartialEq for InternalLog {
    #[inline]
    fn eq(&self, other: &InternalLog) -> bool {
        {
            let __self_vi = unsafe { ::std::intrinsics::discriminant_value(&*self) } as isize;
            let __arg_1_vi = unsafe { ::std::intrinsics::discriminant_value(&*other) } as isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&InternalLog::system(ref __self_0), &InternalLog::system(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (
                        &InternalLog::consensus(ref __self_0),
                        &InternalLog::consensus(ref __arg_1_0),
                    ) => (*__self_0) == (*__arg_1_0),
                    _ => unsafe { ::std::intrinsics::unreachable() },
                }
            } else {
                false
            }
        }
    }
    #[inline]
    fn ne(&self, other: &InternalLog) -> bool {
        {
            let __self_vi = unsafe { ::std::intrinsics::discriminant_value(&*self) } as isize;
            let __arg_1_vi = unsafe { ::std::intrinsics::discriminant_value(&*other) } as isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&InternalLog::system(ref __self_0), &InternalLog::system(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (
                        &InternalLog::consensus(ref __self_0),
                        &InternalLog::consensus(ref __arg_1_0),
                    ) => (*__self_0) != (*__arg_1_0),
                    _ => unsafe { ::std::intrinsics::unreachable() },
                }
            } else {
                true
            }
        }
    }
}

#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(non_camel_case_types)]
impl ::std::cmp::Eq for InternalLog {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::std::cmp::AssertParamIsEq<system::Log<Runtime>>;
            let _: ::std::cmp::AssertParamIsEq<consensus::Log<Runtime>>;
        }
    }
}

#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_InternalLog: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate parity_codec as _parity_codec;
    impl _parity_codec::Encode for InternalLog {
        fn encode_to<EncOut: _parity_codec::Output>(&self, dest: &mut EncOut) {
            match *self {
                InternalLog::system(ref aa) => {
                    dest.push_byte(0usize as u8);
                    dest.push(aa);
                }
                InternalLog::consensus(ref aa) => {
                    dest.push_byte(1usize as u8);
                    dest.push(aa);
                }
                _ => (),
            }
        }
    }
};
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DECODE_FOR_InternalLog: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate parity_codec as _parity_codec;
    impl _parity_codec::Decode for InternalLog {
        fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn) -> Option<Self> {
            match input.read_byte()? {
                x if x == 0usize as u8 => {
                    Some(InternalLog::system(_parity_codec::Decode::decode(input)?))
                }
                x if x == 1usize as u8 => Some(InternalLog::consensus(
                    _parity_codec::Decode::decode(input)?,
                )),
                _ => None,
            }
        }
    }
};

#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(non_camel_case_types)]
impl ::std::fmt::Debug for InternalLog {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match (&*self, ) {
            (&InternalLog::system(ref __self_0), ) => {
                let mut debug_trait_builder = f.debug_tuple("system");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&InternalLog::consensus(ref __self_0), ) => {
                let mut debug_trait_builder = f.debug_tuple("consensus");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
        }
    }
}

#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_InternalLog: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for InternalLog {
        fn serialize<__S>(&self, __serializer: __S) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
        {
            match *self {
                InternalLog::system(ref __field0) => _serde::Serializer::serialize_newtype_variant(
                    __serializer,
                    "InternalLog",
                    0u32,
                    "system",
                    __field0,
                ),
                InternalLog::consensus(ref __field0) => {
                    _serde::Serializer::serialize_newtype_variant(
                        __serializer,
                        "InternalLog",
                        1u32,
                        "consensus",
                        __field0,
                    )
                }
            }
        }
    }
};

impl Log {
    /// Try to convert `$name` into `generic::DigestItemRef`. Returns Some when
    /// `self` is a 'system' log && it has been marked as 'system' in macro call.
    /// Otherwise, None is returned.
    #[allow(unreachable_patterns)]
    fn dref<'a>(
        &'a self,
    ) -> Option<::sr_primitives::generic::DigestItemRef<'a, Hash, AuthorityId, AuthoritySignature>>
    {
        match self.0 {
            InternalLog::system(system::RawLog::ChangesTrieRoot(ref v)) => {
                Some(::sr_primitives::generic::DigestItemRef::ChangesTrieRoot(v))
            }
            InternalLog::consensus(consensus::RawLog::AuthoritiesChange(ref v)) => Some(
                ::sr_primitives::generic::DigestItemRef::AuthoritiesChange(v),
            ),
            _ => None,
        }
    }
}

impl ::sr_primitives::traits::DigestItem for Log {
    type Hash = <::sr_primitives::generic::DigestItem<Hash, AuthorityId, AuthoritySignature> as ::sr_primitives::traits::DigestItem>::Hash;
    type AuthorityId = <::sr_primitives::generic::DigestItem<Hash, AuthorityId, AuthoritySignature> as ::sr_primitives::traits::DigestItem>::AuthorityId;
    fn as_authorities_change(&self) -> Option<&[Self::AuthorityId]> {
        self.dref().and_then(|dref| dref.as_authorities_change())
    }
    fn as_changes_trie_root(&self) -> Option<&Self::Hash> {
        self.dref().and_then(|dref| dref.as_changes_trie_root())
    }
}

impl From<::sr_primitives::generic::DigestItem<Hash, AuthorityId, AuthoritySignature>> for Log {
    /// Converts `generic::DigestItem` into `$name`. If `generic::DigestItem` represents
    /// a system item which is supported by the runtime, it is returned.
    /// Otherwise we expect a `Other` log item. Trying to convert from anything other
    /// will lead to panic in runtime, since the runtime does not supports this 'system'
    /// log item.
    #[allow(unreachable_patterns)]
    fn from(
        gen: ::sr_primitives::generic::DigestItem<Hash, AuthorityId, AuthoritySignature>,
    ) -> Self {
        match gen {
            ::sr_primitives::generic::DigestItem::ChangesTrieRoot(value) => {
                Log(InternalLog::system(system::RawLog::ChangesTrieRoot(value)))
            }
            ::sr_primitives::generic::DigestItem::AuthoritiesChange(value) => Log(
                InternalLog::consensus(consensus::RawLog::AuthoritiesChange(value)),
            ),
            _ => gen
                .as_other()
                .and_then(|value| ::sr_primitives::codec::Decode::decode(&mut &value[..]))
                .map(Log)
                .expect("not allowed to fail in runtime"),
        }
    }
}

impl ::sr_primitives::codec::Decode for Log {
    /// `generic::DigestItem` binary compatible decode.
    fn decode<I: ::sr_primitives::codec::Input>(input: &mut I) -> Option<Self> {
        let gen: ::sr_primitives::generic::DigestItem<Hash, AuthorityId, AuthoritySignature> =
            ::sr_primitives::codec::Decode::decode(input)?;
        Some(Log::from(gen))
    }
}

impl ::sr_primitives::codec::Encode for Log {
    /// `generic::DigestItem` binary compatible encode.
    fn encode(&self) -> Vec<u8> {
        match self.dref() {
            Some(dref) => dref.encode(),
            None => {
                let gen: ::sr_primitives::generic::DigestItem<
                    Hash,
                    AuthorityId,
                    AuthoritySignature,
                > = ::sr_primitives::generic::DigestItem::Other(self.0.encode());
                gen.encode()
            }
        }
    }
}

impl From<system::Log<Runtime>> for Log {
    /// Converts single module log item into `$name`.
    fn from(x: system::Log<Runtime>) -> Self {
        Log(x.into())
    }
}

impl From<system::Log<Runtime>> for InternalLog {
    /// Converts single module log item into `$internal`.
    fn from(x: system::Log<Runtime>) -> Self {
        InternalLog::system(x)
    }
}

impl From<consensus::Log<Runtime>> for Log {
    /// Converts single module log item into `$name`.
    fn from(x: consensus::Log<Runtime>) -> Self {
        Log(x.into())
    }
}

impl From<consensus::Log<Runtime>> for InternalLog {
    /// Converts single module log item into `$internal`.
    fn from(x: consensus::Log<Runtime>) -> Self {
        InternalLog::consensus(x)
    }
}

#[cfg(any(feature = "std", test))]
pub type SystemConfig = system::GenesisConfig<Runtime>;
#[cfg(any(feature = "std", test))]
pub type TimestampConfig = timestamp::GenesisConfig<Runtime>;
#[cfg(any(feature = "std", test))]
pub type ConsensusConfig = consensus::GenesisConfig<Runtime>;
#[cfg(any(feature = "std", test))]
pub type IndicesConfig = indices::GenesisConfig<Runtime>;
#[cfg(any(feature = "std", test))]
pub type BalancesConfig = balances::GenesisConfig<Runtime>;
#[cfg(any(feature = "std", test))]
pub type SudoConfig = sudo::GenesisConfig<Runtime>;

#[cfg(any(feature = "std", test))]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct GenesisConfig {
    pub system: Option<SystemConfig>,
    pub timestamp: Option<TimestampConfig>,
    pub consensus: Option<ConsensusConfig>,
    pub indices: Option<IndicesConfig>,
    pub balances: Option<BalancesConfig>,
    pub sudo: Option<SudoConfig>,
}

#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_GenesisConfig: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for GenesisConfig {
        fn serialize<__S>(&self, __serializer: __S) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
        {
            let mut __serde_state = match _serde::Serializer::serialize_struct(
                __serializer,
                "GenesisConfig",
                false as usize + 1 + 1 + 1 + 1 + 1 + 1,
            ) {
                _serde::export::Ok(__val) => __val,
                _serde::export::Err(__err) => {
                    return _serde::export::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "system",
                &self.system,
            ) {
                _serde::export::Ok(__val) => __val,
                _serde::export::Err(__err) => {
                    return _serde::export::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "timestamp",
                &self.timestamp,
            ) {
                _serde::export::Ok(__val) => __val,
                _serde::export::Err(__err) => {
                    return _serde::export::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "consensus",
                &self.consensus,
            ) {
                _serde::export::Ok(__val) => __val,
                _serde::export::Err(__err) => {
                    return _serde::export::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "indices",
                &self.indices,
            ) {
                _serde::export::Ok(__val) => __val,
                _serde::export::Err(__err) => {
                    return _serde::export::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "balances",
                &self.balances,
            ) {
                _serde::export::Ok(__val) => __val,
                _serde::export::Err(__err) => {
                    return _serde::export::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "sudo",
                &self.sudo,
            ) {
                _serde::export::Ok(__val) => __val,
                _serde::export::Err(__err) => {
                    return _serde::export::Err(__err);
                }
            };
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_GenesisConfig: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for GenesisConfig {
        fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                __field0,
                __field1,
                __field2,
                __field3,
                __field4,
                __field5,
            }
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::export::Formatter,
                ) -> _serde::export::fmt::Result {
                    _serde::export::Formatter::write_str(__formatter, "field identifier")
                }
                fn visit_u64<__E>(self, __value: u64) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::export::Ok(__Field::__field0),
                        1u64 => _serde::export::Ok(__Field::__field1),
                        2u64 => _serde::export::Ok(__Field::__field2),
                        3u64 => _serde::export::Ok(__Field::__field3),
                        4u64 => _serde::export::Ok(__Field::__field4),
                        5u64 => _serde::export::Ok(__Field::__field5),
                        _ => _serde::export::Err(_serde::de::Error::invalid_value(
                            _serde::de::Unexpected::Unsigned(__value),
                            &"field index 0 <= i < 6",
                        )),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                {
                    match __value {
                        "system" => _serde::export::Ok(__Field::__field0),
                        "timestamp" => _serde::export::Ok(__Field::__field1),
                        "consensus" => _serde::export::Ok(__Field::__field2),
                        "indices" => _serde::export::Ok(__Field::__field3),
                        "balances" => _serde::export::Ok(__Field::__field4),
                        "sudo" => _serde::export::Ok(__Field::__field5),
                        _ => _serde::export::Err(_serde::de::Error::unknown_field(__value, FIELDS)),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                {
                    match __value {
                        b"system" => _serde::export::Ok(__Field::__field0),
                        b"timestamp" => _serde::export::Ok(__Field::__field1),
                        b"consensus" => _serde::export::Ok(__Field::__field2),
                        b"indices" => _serde::export::Ok(__Field::__field3),
                        b"balances" => _serde::export::Ok(__Field::__field4),
                        b"sudo" => _serde::export::Ok(__Field::__field5),
                        _ => {
                            let __value = &_serde::export::from_utf8_lossy(__value);
                            _serde::export::Err(_serde::de::Error::unknown_field(__value, FIELDS))
                        }
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                }
            }
            struct __Visitor<'de> {
                marker: _serde::export::PhantomData<GenesisConfig>,
                lifetime: _serde::export::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = GenesisConfig;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::export::Formatter,
                ) -> _serde::export::fmt::Result {
                    _serde::export::Formatter::write_str(__formatter, "struct GenesisConfig")
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match match _serde::de::SeqAccess::next_element::<
                        Option<SystemConfig>,
                    >(&mut __seq)
                        {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                        _serde::export::Some(__value) => __value,
                        _serde::export::None => {
                            return _serde::export::Err(_serde::de::Error::invalid_length(
                                0usize,
                                &"struct GenesisConfig with 6 elements",
                            ));
                        }
                    };
                    let __field1 = match match _serde::de::SeqAccess::next_element::<
                        Option<TimestampConfig>,
                    >(&mut __seq)
                        {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                        _serde::export::Some(__value) => __value,
                        _serde::export::None => {
                            return _serde::export::Err(_serde::de::Error::invalid_length(
                                1usize,
                                &"struct GenesisConfig with 6 elements",
                            ));
                        }
                    };
                    let __field2 = match match _serde::de::SeqAccess::next_element::<
                        Option<ConsensusConfig>,
                    >(&mut __seq)
                        {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                        _serde::export::Some(__value) => __value,
                        _serde::export::None => {
                            return _serde::export::Err(_serde::de::Error::invalid_length(
                                2usize,
                                &"struct GenesisConfig with 6 elements",
                            ));
                        }
                    };
                    let __field3 = match match _serde::de::SeqAccess::next_element::<
                        Option<IndicesConfig>,
                    >(&mut __seq)
                        {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                        _serde::export::Some(__value) => __value,
                        _serde::export::None => {
                            return _serde::export::Err(_serde::de::Error::invalid_length(
                                3usize,
                                &"struct GenesisConfig with 6 elements",
                            ));
                        }
                    };
                    let __field4 = match match _serde::de::SeqAccess::next_element::<
                        Option<BalancesConfig>,
                    >(&mut __seq)
                        {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                        _serde::export::Some(__value) => __value,
                        _serde::export::None => {
                            return _serde::export::Err(_serde::de::Error::invalid_length(
                                4usize,
                                &"struct GenesisConfig with 6 elements",
                            ));
                        }
                    };
                    let __field5 = match match _serde::de::SeqAccess::next_element::<
                        Option<SudoConfig>,
                    >(&mut __seq)
                        {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                        _serde::export::Some(__value) => __value,
                        _serde::export::None => {
                            return _serde::export::Err(_serde::de::Error::invalid_length(
                                5usize,
                                &"struct GenesisConfig with 6 elements",
                            ));
                        }
                    };
                    _serde::export::Ok(GenesisConfig {
                        system: __field0,
                        timestamp: __field1,
                        consensus: __field2,
                        indices: __field3,
                        balances: __field4,
                        sudo: __field5,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::export::Option<Option<SystemConfig>> =
                        _serde::export::None;
                    let mut __field1: _serde::export::Option<Option<TimestampConfig>> =
                        _serde::export::None;
                    let mut __field2: _serde::export::Option<Option<ConsensusConfig>> =
                        _serde::export::None;
                    let mut __field3: _serde::export::Option<Option<IndicesConfig>> =
                        _serde::export::None;
                    let mut __field4: _serde::export::Option<Option<BalancesConfig>> =
                        _serde::export::None;
                    let mut __field5: _serde::export::Option<Option<SudoConfig>> =
                        _serde::export::None;
                    while let _serde::export::Some(__key) =
                    match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::export::Option::is_some(&__field0) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "system",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Option<SystemConfig>>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::export::Option::is_some(&__field1) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "timestamp",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Option<TimestampConfig>>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::export::Option::is_some(&__field2) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "consensus",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Option<ConsensusConfig>>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::export::Option::is_some(&__field3) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "indices",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Option<IndicesConfig>>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field4 => {
                                    if _serde::export::Option::is_some(&__field4) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "balances",
                                            ),
                                        );
                                    }
                                    __field4 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Option<BalancesConfig>>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field5 => {
                                    if _serde::export::Option::is_some(&__field5) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("sudo"),
                                        );
                                    }
                                    __field5 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Option<SudoConfig>>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                            }
                        }
                    let __field0 = match __field0 {
                        _serde::export::Some(__field0) => __field0,
                        _serde::export::None => {
                            match _serde::private::de::missing_field("system") {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        }
                    };
                    let __field1 = match __field1 {
                        _serde::export::Some(__field1) => __field1,
                        _serde::export::None => {
                            match _serde::private::de::missing_field("timestamp") {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        }
                    };
                    let __field2 = match __field2 {
                        _serde::export::Some(__field2) => __field2,
                        _serde::export::None => {
                            match _serde::private::de::missing_field("consensus") {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        }
                    };
                    let __field3 = match __field3 {
                        _serde::export::Some(__field3) => __field3,
                        _serde::export::None => match _serde::private::de::missing_field("indices")
                            {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            },
                    };
                    let __field4 = match __field4 {
                        _serde::export::Some(__field4) => __field4,
                        _serde::export::None => {
                            match _serde::private::de::missing_field("balances") {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        }
                    };
                    let __field5 = match __field5 {
                        _serde::export::Some(__field5) => __field5,
                        _serde::export::None => match _serde::private::de::missing_field("sudo") {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        },
                    };
                    _serde::export::Ok(GenesisConfig {
                        system: __field0,
                        timestamp: __field1,
                        consensus: __field2,
                        indices: __field3,
                        balances: __field4,
                        sudo: __field5,
                    })
                }
            }
            const FIELDS: &'static [&'static str] = &[
                "system",
                "timestamp",
                "consensus",
                "indices",
                "balances",
                "sudo",
            ];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "GenesisConfig",
                FIELDS,
                __Visitor {
                    marker: _serde::export::PhantomData::<GenesisConfig>,
                    lifetime: _serde::export::PhantomData,
                },
            )
        }
    }
};

#[cfg(any(feature = "std", test))]
impl ::sr_primitives::BuildStorage for GenesisConfig {
    fn assimilate_storage(
        self,
        top: &mut ::sr_primitives::StorageOverlay,
        children: &mut ::sr_primitives::ChildrenStorageOverlay,
    ) -> ::std::result::Result<(), String> {
        if let Some(extra) = self.system {
            extra.assimilate_storage(top, children)?;
        }
        if let Some(extra) = self.timestamp {
            extra.assimilate_storage(top, children)?;
        }
        if let Some(extra) = self.consensus {
            extra.assimilate_storage(top, children)?;
        }
        if let Some(extra) = self.indices {
            extra.assimilate_storage(top, children)?;
        }
        if let Some(extra) = self.balances {
            extra.assimilate_storage(top, children)?;
        }
        if let Some(extra) = self.sudo {
            extra.assimilate_storage(top, children)?;
        }
        Ok(())
    }
}

trait InherentDataExt {
    fn create_extrinsics(
        &self,
    ) -> ::srml_support::inherent::Vec<<Block as ::srml_support::inherent::BlockT>::Extrinsic>;
    fn check_extrinsics(&self, block: &Block) -> ::srml_support::inherent::CheckInherentsResult;
}

impl InherentDataExt for ::srml_support::inherent::InherentData {
    fn create_extrinsics(
        &self,
    ) -> ::srml_support::inherent::Vec<<Block as ::srml_support::inherent::BlockT>::Extrinsic> {
        use ::srml_support::inherent::ProvideInherent;
        let mut inherents = Vec::new();
        if let Some(inherent) = Timestamp::create_inherent(self) {
            inherents.push(UncheckedExtrinsic::new_unsigned(Call::Timestamp(inherent)));
        }
        if let Some(inherent) = Consensus::create_inherent(self) {
            inherents.push(UncheckedExtrinsic::new_unsigned(Call::Consensus(inherent)));
        }
        inherents
    }
    fn check_extrinsics(&self, block: &Block) -> ::srml_support::inherent::CheckInherentsResult {
        use ::srml_support::inherent::{IsFatalError, ProvideInherent};
        let mut result = ::srml_support::inherent::CheckInherentsResult::new();
        for xt in block.extrinsics() {
            if ::srml_support::inherent::Extrinsic::is_signed(xt).unwrap_or(false) {
                break;
            }
            match xt.function {
                Call::Timestamp(ref call) => {
                    if let Err(e) = Timestamp::check_inherent(call, self) {
                        result
                            .put_error(Timestamp::INHERENT_IDENTIFIER, &e)
                            .expect("There is only one fatal error; qed");
                        if e.is_fatal_error() {
                            return result;
                        }
                    }
                }
                _ => {}
            }
            match xt.function {
                Call::Consensus(ref call) => {
                    if let Err(e) = Consensus::check_inherent(call, self) {
                        result
                            .put_error(Consensus::INHERENT_IDENTIFIER, &e)
                            .expect("There is only one fatal error; qed");
                        if e.is_fatal_error() {
                            return result;
                        }
                    }
                }
                _ => {}
            }
        }
        result
    }
}

/// The type used as a helper for interpreting the sender of transactions.
type Context = system::ChainContext<Runtime>;
/// The address format for describing accounts.
type Address = <Indices as StaticLookup>::Source;
/// Block header type as expected by this runtime.
pub type Header = generic::Header<BlockNumber, BlakeTwo256, Log>;
/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
/// BlockId type as expected by this runtime.
pub type BlockId = generic::BlockId<Block>;
/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic =
generic::UncheckedMortalCompactExtrinsic<Address, Nonce, Call, AccountSignature>;
/// Extrinsic type that has already been checked.
pub type CheckedExtrinsic = generic::CheckedExtrinsic<AccountId, Nonce, Call>;
/// Executive: handles dispatch to the various modules.
pub type Executive = executive::Executive<Runtime, Block, Context, Balances, AllModules>;

#[doc(hidden)]
mod sr_api_hidden_includes_IMPL_RUNTIME_APIS {
    pub extern crate client as sr_api_client;
}

pub struct RuntimeApi {}

/// Implements all runtime apis for the client side.
#[cfg(any(feature = "std", test))]
pub struct RuntimeApiImpl<C: self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::CallRuntimeAt<<Runtime as self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock> + 'static> {
    call: &'static C,
    commit_on_success: ::std::cell::RefCell<bool>,
    initialized_block: ::std::cell::RefCell<Option<self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::BlockId<<Runtime as self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock>>>,
    changes: ::std::cell::RefCell<self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::OverlayedChanges>
}

#[cfg(any(feature = "std", test))]
unsafe impl<C: self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::CallRuntimeAt<<Runtime as self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock>> Send for RuntimeApiImpl<C> {}

#[cfg(any(feature = "std", test))]
unsafe impl<C: self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::CallRuntimeAt<<Runtime as self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock>> Sync for RuntimeApiImpl<C> {}

#[cfg(any(feature = "std", test))]
impl<C: self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::CallRuntimeAt<<Runtime as self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock>> self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::ApiExt<<Runtime as self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock> for RuntimeApiImpl<C> {
    fn map_api_result<F: FnOnce(&Self) -> ::std::result::Result<R, E>, R, E>(&self, map_call: F) -> ::std::result::Result<R, E> where Self: Sized {
        *self.commit_on_success.borrow_mut() = false;
        let res = map_call(self);
        *self.commit_on_success.borrow_mut() = true;
        self.commit_on_ok(&res);
        res
    }
    fn runtime_version_at(&self, at: &self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::BlockId<<Runtime as self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock>) -> self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::error::Result<self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::RuntimeVersion> { self.call.runtime_version_at(at) }
}

#[cfg(any(feature = "std", test))]
impl<C: self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::CallRuntimeAt<<Runtime as self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock> + 'static> self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::ConstructRuntimeApi<<Runtime as self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock, C> for RuntimeApi {
    type RuntimeApi = RuntimeApiImpl<C>;
    fn construct_runtime_api<'a>(call: &'a C) -> self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::ApiRef<'a, Self::RuntimeApi> { RuntimeApiImpl { call: unsafe { ::std::mem::transmute(call) }, commit_on_success: true.into(), initialized_block: None.into(), changes: Default::default() }.into() }
}

#[cfg(any(feature = "std", test))]
impl<C: self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::CallRuntimeAt<<Runtime as self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock>> RuntimeApiImpl<C> {
    fn call_api_at<R: self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::Encode + self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::Decode + PartialEq, F: FnOnce(&C, &mut self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::OverlayedChanges, &mut Option<self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::BlockId<<Runtime as self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock>>) -> self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::error::Result<self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::NativeOrEncoded<R>>>(&self, call_api_at: F) -> self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::error::Result<self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::NativeOrEncoded<R>> {
        let res = unsafe { call_api_at(&self.call, &mut *self.changes.borrow_mut(), &mut *self.initialized_block.borrow_mut()) };
        self.commit_on_ok(&res);
        res
    }
    fn commit_on_ok<R, E>(&self, res: &::std::result::Result<R, E>) { if *self.commit_on_success.borrow() { if res.is_err() { self.changes.borrow_mut().discard_prospective(); } else { self.changes.borrow_mut().commit_prospective(); } } }
}

impl runtime_api::runtime_decl_for_Core::Core<Block> for Runtime {
    fn version() -> RuntimeVersion {
        VERSION
    }
    fn execute_block(block: Block) {
        Executive::execute_block(block)
    }
    fn initialize_block(header: &<Block as BlockT>::Header) {
        Executive::initialize_block(header)
    }
    fn authorities() -> Vec<AuthorityId> {
        {
            ::std::rt::begin_panic(
                "Deprecated, please use `AuthoritiesApi`.",
                &("src/lib.rs", 252u32, 4u32),
            )
        }
    }
}

impl runtime_api::runtime_decl_for_Metadata::Metadata<Block> for Runtime {
    fn metadata() -> OpaqueMetadata {
        Runtime::metadata().into()
    }
}

impl block_builder_api::runtime_decl_for_BlockBuilder::BlockBuilder<Block> for Runtime {
    fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyResult {
        Executive::apply_extrinsic(extrinsic)
    }
    fn finalize_block() -> <Block as BlockT>::Header {
        Executive::finalize_block()
    }
    fn inherent_extrinsics(data: InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
        data.create_extrinsics()
    }
    fn check_inherents(block: Block, data: InherentData) -> CheckInherentsResult {
        data.check_extrinsics(&block)
    }
    fn random_seed() -> <Block as BlockT>::Hash {
        System::random_seed()
    }
}

impl runtime_api::runtime_decl_for_TaggedTransactionQueue::TaggedTransactionQueue<Block>
for Runtime
{
    fn validate_transaction(tx: <Block as BlockT>::Extrinsic) -> TransactionValidity {
        Executive::validate_transaction(tx)
    }
}

impl consensus_aura::runtime_decl_for_AuraApi::AuraApi<Block> for Runtime {
    fn slot_duration() -> u64 {
        Aura::slot_duration()
    }
}

impl offchain_primitives::runtime_decl_for_OffchainWorkerApi::OffchainWorkerApi<Block> for Runtime {
    fn offchain_worker(n: NumberFor<Block>) {
        Executive::offchain_worker(n)
    }
}

impl consensus_authorities::runtime_decl_for_AuthoritiesApi::AuthoritiesApi<Block> for Runtime {
    fn authorities() -> Vec<AuthorityId> {
        Consensus::authorities()
    }
}

#[cfg(any(feature = "std", test))]
impl<RuntimeApiImplCall: self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::CallRuntimeAt<<Runtime as self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock> + 'static> runtime_api::Core<<Runtime as self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock> for RuntimeApiImpl<RuntimeApiImplCall> {
    fn Core_version_runtime_api_impl(&self, at: &self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::BlockId<<Runtime as self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock>, context: self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::ExecutionContext, params: Option<()>, params_encoded: Vec<u8>) -> self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::error::Result<self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::NativeOrEncoded<RuntimeVersion>> { self.call_api_at(|call_runtime_at, changes, initialized_block| { runtime_api::runtime_decl_for_Core::version_call_api_at(call_runtime_at, at, params_encoded, changes, initialized_block, params.map(|p| { runtime_api::runtime_decl_for_Core::version_native_call_generator::<Runtime, <Runtime as self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock, Block>() }), context) }) }
    fn Core_execute_block_runtime_api_impl(&self, at: &self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::BlockId<<Runtime as self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock>, context: self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::ExecutionContext, params: Option<( <Runtime as self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock )>, params_encoded: Vec<u8>) -> self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::error::Result<self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::NativeOrEncoded<()>> { self.call_api_at(|call_runtime_at, changes, initialized_block| { runtime_api::runtime_decl_for_Core::execute_block_call_api_at(call_runtime_at, at, params_encoded, changes, initialized_block, params.map(|p| { runtime_api::runtime_decl_for_Core::execute_block_native_call_generator::<Runtime, <Runtime as self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock, Block>(p) }), context) }) }
    fn Core_initialize_block_runtime_api_impl(&self, at: &self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::BlockId<<Runtime as self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock>, context: self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::ExecutionContext, params: Option<( &<<Runtime as self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock as BlockT>::Header )>, params_encoded: Vec<u8>) -> self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::error::Result<self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::NativeOrEncoded<()>> { self.call_api_at(|call_runtime_at, changes, initialized_block| { runtime_api::runtime_decl_for_Core::initialize_block_call_api_at(call_runtime_at, at, params_encoded, changes, initialized_block, params.map(|p| { runtime_api::runtime_decl_for_Core::initialize_block_native_call_generator::<Runtime, <Runtime as self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock, Block>(p) }), context) }) }
    fn Core_authorities_runtime_api_impl(&self, at: &self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::BlockId<<Runtime as self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock>, context: self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::ExecutionContext, params: Option<()>, params_encoded: Vec<u8>) -> self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::error::Result<self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::NativeOrEncoded<Vec<AuthorityId>>> { self.call_api_at(|call_runtime_at, changes, initialized_block| { runtime_api::runtime_decl_for_Core::authorities_call_api_at(call_runtime_at, at, params_encoded, changes, initialized_block, params.map(|p| { runtime_api::runtime_decl_for_Core::authorities_native_call_generator::<Runtime, <Runtime as self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock, Block>() }), context) }) }
}

#[cfg(any(feature = "std", test))]
impl<RuntimeApiImplCall: self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::CallRuntimeAt<<Runtime as self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock> + 'static> runtime_api::Metadata<<Runtime as self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock> for RuntimeApiImpl<RuntimeApiImplCall> {
    fn Metadata_metadata_runtime_api_impl(&self, at: &self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::BlockId<<Runtime as self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock>, context: self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::ExecutionContext, params: Option<()>, params_encoded: Vec<u8>) -> self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::error::Result<self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::NativeOrEncoded<OpaqueMetadata>> { self.call_api_at(|call_runtime_at, changes, initialized_block| { runtime_api::runtime_decl_for_Metadata::metadata_call_api_at(call_runtime_at, at, params_encoded, changes, initialized_block, params.map(|p| { runtime_api::runtime_decl_for_Metadata::metadata_native_call_generator::<Runtime, <Runtime as self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock, Block>() }), context) }) }
}

#[cfg(any(feature = "std", test))]
impl<RuntimeApiImplCall: self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::CallRuntimeAt<<Runtime as self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock> + 'static> block_builder_api::BlockBuilder<<Runtime as self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock> for RuntimeApiImpl<RuntimeApiImplCall> {
    fn BlockBuilder_apply_extrinsic_runtime_api_impl(&self, at: &self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::BlockId<<Runtime as self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock>, context: self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::ExecutionContext, params: Option<( <<Runtime as self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock as BlockT>::Extrinsic )>, params_encoded: Vec<u8>) -> self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::error::Result<self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::NativeOrEncoded<ApplyResult>> { self.call_api_at(|call_runtime_at, changes, initialized_block| { block_builder_api::runtime_decl_for_BlockBuilder::apply_extrinsic_call_api_at(call_runtime_at, at, params_encoded, changes, initialized_block, params.map(|p| { block_builder_api::runtime_decl_for_BlockBuilder::apply_extrinsic_native_call_generator::<Runtime, <Runtime as self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock, Block>(p) }), context) }) }
    fn BlockBuilder_finalize_block_runtime_api_impl(&self, at: &self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::BlockId<<Runtime as self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock>, context: self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::ExecutionContext, params: Option<()>, params_encoded: Vec<u8>) -> self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::error::Result<self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::NativeOrEncoded<<<Runtime as self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock as BlockT>::Header>> { self.call_api_at(|call_runtime_at, changes, initialized_block| { block_builder_api::runtime_decl_for_BlockBuilder::finalize_block_call_api_at(call_runtime_at, at, params_encoded, changes, initialized_block, params.map(|p| { block_builder_api::runtime_decl_for_BlockBuilder::finalize_block_native_call_generator::<Runtime, <Runtime as self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock, Block>() }), context) }) }
    fn BlockBuilder_inherent_extrinsics_runtime_api_impl(&self, at: &self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::BlockId<<Runtime as self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock>, context: self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::ExecutionContext, params: Option<( InherentData )>, params_encoded: Vec<u8>) -> self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::error::Result<self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::NativeOrEncoded<Vec<<<Runtime as self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock as BlockT>::Extrinsic>>> { self.call_api_at(|call_runtime_at, changes, initialized_block| { block_builder_api::runtime_decl_for_BlockBuilder::inherent_extrinsics_call_api_at(call_runtime_at, at, params_encoded, changes, initialized_block, params.map(|p| { block_builder_api::runtime_decl_for_BlockBuilder::inherent_extrinsics_native_call_generator::<Runtime, <Runtime as self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock, Block>(p) }), context) }) }
    fn BlockBuilder_check_inherents_runtime_api_impl(&self, at: &self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::BlockId<<Runtime as self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock>, context: self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::ExecutionContext, params: Option<(<Runtime as self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock, InherentData)>, params_encoded: Vec<u8>) -> self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::error::Result<self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::NativeOrEncoded<CheckInherentsResult>> { self.call_api_at(|call_runtime_at, changes, initialized_block| { block_builder_api::runtime_decl_for_BlockBuilder::check_inherents_call_api_at(call_runtime_at, at, params_encoded, changes, initialized_block, params.map(|p| { block_builder_api::runtime_decl_for_BlockBuilder::check_inherents_native_call_generator::<Runtime, <Runtime as self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock, Block>(p.0, p.1) }), context) }) }
    fn BlockBuilder_random_seed_runtime_api_impl(&self, at: &self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::BlockId<<Runtime as self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock>, context: self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::ExecutionContext, params: Option<()>, params_encoded: Vec<u8>) -> self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::error::Result<self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::NativeOrEncoded<<<Runtime as self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock as BlockT>::Hash>> { self.call_api_at(|call_runtime_at, changes, initialized_block| { block_builder_api::runtime_decl_for_BlockBuilder::random_seed_call_api_at(call_runtime_at, at, params_encoded, changes, initialized_block, params.map(|p| { block_builder_api::runtime_decl_for_BlockBuilder::random_seed_native_call_generator::<Runtime, <Runtime as self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock, Block>() }), context) }) }
}

#[cfg(any(feature = "std", test))]
impl<RuntimeApiImplCall: self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::CallRuntimeAt<<Runtime as self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock> + 'static> runtime_api::TaggedTransactionQueue<<Runtime as self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock> for RuntimeApiImpl<RuntimeApiImplCall> {
    fn TaggedTransactionQueue_validate_transaction_runtime_api_impl(&self, at: &self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::BlockId<<Runtime as self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock>, context: self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::ExecutionContext, params: Option<( <<Runtime as self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock as BlockT>::Extrinsic )>, params_encoded: Vec<u8>) -> self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::error::Result<self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::NativeOrEncoded<TransactionValidity>> { self.call_api_at(|call_runtime_at, changes, initialized_block| { runtime_api::runtime_decl_for_TaggedTransactionQueue::validate_transaction_call_api_at(call_runtime_at, at, params_encoded, changes, initialized_block, params.map(|p| { runtime_api::runtime_decl_for_TaggedTransactionQueue::validate_transaction_native_call_generator::<Runtime, <Runtime as self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock, Block>(p) }), context) }) }
}

#[cfg(any(feature = "std", test))]
impl<RuntimeApiImplCall: self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::CallRuntimeAt<<Runtime as self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock> + 'static> consensus_aura::AuraApi<<Runtime as self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock> for RuntimeApiImpl<RuntimeApiImplCall> {
    fn AuraApi_slot_duration_runtime_api_impl(&self, at: &self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::BlockId<<Runtime as self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock>, context: self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::ExecutionContext, params: Option<()>, params_encoded: Vec<u8>) -> self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::error::Result<self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::NativeOrEncoded<u64>> { self.call_api_at(|call_runtime_at, changes, initialized_block| { consensus_aura::runtime_decl_for_AuraApi::slot_duration_call_api_at(call_runtime_at, at, params_encoded, changes, initialized_block, params.map(|p| { consensus_aura::runtime_decl_for_AuraApi::slot_duration_native_call_generator::<Runtime, <Runtime as self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock, Block>() }), context) }) }
}

#[cfg(any(feature = "std", test))]
impl<RuntimeApiImplCall: self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::CallRuntimeAt<<Runtime as self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock> + 'static> offchain_primitives::OffchainWorkerApi<<Runtime as self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock> for RuntimeApiImpl<RuntimeApiImplCall> {
    fn OffchainWorkerApi_offchain_worker_runtime_api_impl(&self, at: &self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::BlockId<<Runtime as self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock>, context: self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::ExecutionContext, params: Option<( NumberFor<<Runtime as self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock> )>, params_encoded: Vec<u8>) -> self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::error::Result<self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::NativeOrEncoded<()>> { self.call_api_at(|call_runtime_at, changes, initialized_block| { offchain_primitives::runtime_decl_for_OffchainWorkerApi::offchain_worker_call_api_at(call_runtime_at, at, params_encoded, changes, initialized_block, params.map(|p| { offchain_primitives::runtime_decl_for_OffchainWorkerApi::offchain_worker_native_call_generator::<Runtime, <Runtime as self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock, Block>(p) }), context) }) }
}

#[cfg(any(feature = "std", test))]
impl<RuntimeApiImplCall: self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::CallRuntimeAt<<Runtime as self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock> + 'static> consensus_authorities::AuthoritiesApi<<Runtime as self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock> for RuntimeApiImpl<RuntimeApiImplCall> {
    fn AuthoritiesApi_authorities_runtime_api_impl(&self, at: &self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::BlockId<<Runtime as self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock>, context: self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::ExecutionContext, params: Option<()>, params_encoded: Vec<u8>) -> self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::error::Result<self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::NativeOrEncoded<Vec<AuthorityId>>> { self.call_api_at(|call_runtime_at, changes, initialized_block| { consensus_authorities::runtime_decl_for_AuthoritiesApi::authorities_call_api_at(call_runtime_at, at, params_encoded, changes, initialized_block, params.map(|p| { consensus_authorities::runtime_decl_for_AuthoritiesApi::authorities_native_call_generator::<Runtime, <Runtime as self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock, Block>() }), context) }) }
}

const RUNTIME_API_VERSIONS:
self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::ApisVec =
    ::std::borrow::Cow::Borrowed(&[
        (
            runtime_api::runtime_decl_for_Core::ID,
            runtime_api::runtime_decl_for_Core::VERSION,
        ),
        (
            runtime_api::runtime_decl_for_Metadata::ID,
            runtime_api::runtime_decl_for_Metadata::VERSION,
        ),
        (
            block_builder_api::runtime_decl_for_BlockBuilder::ID,
            block_builder_api::runtime_decl_for_BlockBuilder::VERSION,
        ),
        (
            runtime_api::runtime_decl_for_TaggedTransactionQueue::ID,
            runtime_api::runtime_decl_for_TaggedTransactionQueue::VERSION,
        ),
        (
            consensus_aura::runtime_decl_for_AuraApi::ID,
            consensus_aura::runtime_decl_for_AuraApi::VERSION,
        ),
        (
            offchain_primitives::runtime_decl_for_OffchainWorkerApi::ID,
            offchain_primitives::runtime_decl_for_OffchainWorkerApi::VERSION,
        ),
        (
            consensus_authorities::runtime_decl_for_AuthoritiesApi::ID,
            consensus_authorities::runtime_decl_for_AuthoritiesApi::VERSION,
        ),
    ]);

pub mod api {
    use super::*;

    #[cfg(feature = "std")]
    pub fn dispatch(method: &str, mut data: &[u8]) -> Option<Vec<u8>> {
        match method {
            "Core_version" => Some({
                let output =
                    <Runtime as runtime_api::runtime_decl_for_Core::Core<Block>>::version();
                self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::Encode::encode(&output)
            }),
            "Core_execute_block" => Some({
                let block: Block = match self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::Decode::decode(&mut data) {
                    Some(input) => input,
                    None => { ::std::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1(&["Bad input data provided to "], &match (&"execute_block", ) { (arg0, ) => [::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Display::fmt)], }), &("src/lib.rs", 237u32, 1u32)) }
                };
                let output =
                    <Runtime as runtime_api::runtime_decl_for_Core::Core<Block>>::execute_block(
                        block,
                    );
                self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::Encode::encode(&output)
            }),
            "Core_initialize_block" => {
                Some({
                    let header: <Block as BlockT>::Header = match self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::Decode::decode(&mut data) {
                        Some(input) => input,
                        None => { ::std::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1(&["Bad input data provided to "], &match (&"initialize_block", ) { (arg0, ) => [::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Display::fmt)], }), &("src/lib.rs", 237u32, 1u32)) }
                    };
                    let output = <Runtime as runtime_api::runtime_decl_for_Core::Core<Block>>::initialize_block(&header);
                    self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::Encode::encode(&output)
                })
            }
            "Core_authorities" => Some({
                let output =
                    <Runtime as runtime_api::runtime_decl_for_Core::Core<Block>>::authorities();
                self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::Encode::encode(&output)
            }),
            "Metadata_metadata" => Some({
                let output =
                    <Runtime as runtime_api::runtime_decl_for_Metadata::Metadata<Block>>::metadata();
                self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::Encode::encode(&output)
            }),
            "BlockBuilder_apply_extrinsic" => {
                Some({
                    let extrinsic: <Block as BlockT>::Extrinsic = match self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::Decode::decode(&mut data) {
                        Some(input) => input,
                        None => { ::std::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1(&["Bad input data provided to "], &match (&"apply_extrinsic", ) { (arg0, ) => [::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Display::fmt)], }), &("src/lib.rs", 237u32, 1u32)) }
                    };
                    let output = <Runtime as block_builder_api::runtime_decl_for_BlockBuilder::BlockBuilder<Block>>::apply_extrinsic(extrinsic);
                    self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::Encode::encode(&output)
                })
            }
            "BlockBuilder_finalize_block" => {
                Some({
                    let output = <Runtime as block_builder_api::runtime_decl_for_BlockBuilder::BlockBuilder<Block>>::finalize_block();
                    self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::Encode::encode(&output)
                })
            }
            "BlockBuilder_inherent_extrinsics" => {
                Some({
                    let data: InherentData = match self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::Decode::decode(&mut data) {
                        Some(input) => input,
                        None => { ::std::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1(&["Bad input data provided to "], &match (&"inherent_extrinsics", ) { (arg0, ) => [::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Display::fmt)], }), &("src/lib.rs", 237u32, 1u32)) }
                    };
                    let output = <Runtime as block_builder_api::runtime_decl_for_BlockBuilder::BlockBuilder<Block>>::inherent_extrinsics(data);
                    self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::Encode::encode(&output)
                })
            }
            "BlockBuilder_check_inherents" => {
                Some({
                    let block: Block = match self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::Decode::decode(&mut data) {
                        Some(input) => input,
                        None => { ::std::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1(&["Bad input data provided to "], &match (&"check_inherents", ) { (arg0, ) => [::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Display::fmt)], }), &("src/lib.rs", 237u32, 1u32)) }
                    };
                    let data: InherentData = match self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::Decode::decode(&mut data) {
                        Some(input) => input,
                        None => { ::std::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1(&["Bad input data provided to "], &match (&"check_inherents", ) { (arg0, ) => [::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Display::fmt)], }), &("src/lib.rs", 237u32, 1u32)) }
                    };
                    let output = <Runtime as block_builder_api::runtime_decl_for_BlockBuilder::BlockBuilder<Block>>::check_inherents(block, data);
                    self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::Encode::encode(&output)
                })
            }
            "BlockBuilder_random_seed" => {
                Some({
                    let output = <Runtime as block_builder_api::runtime_decl_for_BlockBuilder::BlockBuilder<Block>>::random_seed();
                    self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::Encode::encode(&output)
                })
            }
            "TaggedTransactionQueue_validate_transaction" => Some({
                let tx: <Block as BlockT>::Extrinsic = match self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::Decode::decode(&mut data) {
                    Some(input) => input,
                    None => { ::std::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1(&["Bad input data provided to "], &match (&"validate_transaction", ) { (arg0, ) => [::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Display::fmt)], }), &("src/lib.rs", 237u32, 1u32)) }
                };
                let output = <Runtime as runtime_api::runtime_decl_for_TaggedTransactionQueue::TaggedTransactionQueue<Block>>::validate_transaction(tx);
                self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::Encode::encode(&output)
            }),
            "AuraApi_slot_duration" => Some({
                let output = <Runtime as consensus_aura::runtime_decl_for_AuraApi::AuraApi<
                    Block,
                >>::slot_duration();
                self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::Encode::encode(&output)
            }),
            "OffchainWorkerApi_offchain_worker" => Some({
                let n: NumberFor<Block> = match self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::Decode::decode(&mut data) {
                    Some(input) => input,
                    None => { ::std::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1(&["Bad input data provided to "], &match (&"offchain_worker", ) { (arg0, ) => [::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Display::fmt)], }), &("src/lib.rs", 237u32, 1u32)) }
                };
                let output = <Runtime as offchain_primitives::runtime_decl_for_OffchainWorkerApi::OffchainWorkerApi<Block>>::offchain_worker(n);
                self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::Encode::encode(&output)
            }),
            "AuthoritiesApi_authorities" => Some({
                let output = <Runtime as consensus_authorities::runtime_decl_for_AuthoritiesApi::AuthoritiesApi<Block>>::authorities();
                self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::Encode::encode(&output)
            }),
            _ => None,
        }
    }
}
