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
