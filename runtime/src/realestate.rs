/// Realestate runtime module
use support::{
	decl_module,
	decl_storage,
	decl_event,
	StorageValue,
	StorageMap,
	ensure,
	dispatch::Result,
	traits::ReservableCurrency,
	traits::Currency
};
use system::{
	ensure_signed,
	ensure_root
};
use runtime_primitives::traits::Hash;
use parity_codec::{Encode, Decode};

/// The module's configuration trait.
pub trait Trait: balances::Trait {
	// TODO: Add other types and constants required configure this module.

	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Property<Hash> {
	id: Hash,
	size: u64,
	certificate_no: u64, // TODO use vec<u8>
	is_authenticated: bool
}

/// This module's storage items.
decl_storage! {
	trait Store for Module<T: Trait> as RealEstateStorage {
		Something get(something): Option<u32>;

		Nonce: u64;

		Properties get(property): map T::Hash => Property<T::Hash>;

		AllPropertiesArray: map u64 => T::Hash;

		PropertyOwner get(property_owner): map T::Hash => T::AccountId;

		Managers get(manager): map u64 => T::AccountId;

		ManagersIndex: map T::AccountId => u64;

		ManagerNonce: u64;

		PropertiesForsale get(forsale_property): map u64 => (T::Hash, T::Balance, bool, Option<T::AccountId>);
		PropertyForsaleCount get(property_forsale_count): u64;
        PropertyForsaleIndex: map T::Hash => u64;
	}
}

decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Initializing events
		// this is needed only if you are using events in your module
		fn deposit_event<T>() = default;

		// Just a dummy entry point.
		// function that can be called by the external world as an extrinsics call
		// takes a parameter of the type `AccountId`, stores it and emits an event
		pub fn do_something(origin, something: u32) -> Result {
			// TODO: You only need this if you want to check it was signed.
			let who = ensure_signed(origin)?;

			// TODO: Code to execute when something calls this.
			// For example: the following line stores the passed in u32 in the storage
			<Something<T>>::put(something);

			// here we are raising the Something event
			Self::deposit_event(RawEvent::SomethingStored(something, who));
			Ok(())
		}

		pub fn record_property(origin, size: u64, certificate_no: u64) -> Result {
			let sender = ensure_signed(origin)?;

            let nonce = <Nonce<T>>::get();
            let random_seed = <system::Module<T>>::random_seed();
            let random_hash = (random_seed, &sender, nonce).using_encoded(<T as system::Trait>::Hashing::hash);

			let property = Property {
				id: random_hash,
				size: size,
				certificate_no: certificate_no,
				is_authenticated: false
			};

			<Properties<T>>::insert(random_hash, property);
			<AllPropertiesArray<T>>::insert(nonce, random_hash);

			<PropertyOwner<T>>::insert(random_hash, sender);

			<Nonce<T>>::mutate(|n| *n += 1);

			Ok(())
		}

		pub fn add_manager(origin, account_id: T::AccountId) -> Result {
			let sender = ensure_root(origin)?;

			ensure!(!<ManagersIndex<T>>::exists(&account_id), "The account is already manager");

			let nonce = <ManagerNonce<T>>::get();

			<Managers<T>>::insert(nonce, &account_id);
			<ManagersIndex<T>>::insert(&account_id, nonce);

			<ManagerNonce<T>>::mutate(|n| *n += 1);

			Self::deposit_event(RawEvent::ManagerAdded(account_id));

			Ok(())
		}

		pub fn authenticate(origin, property_id: T::Hash, is_authenticated: bool) -> Result {
			let sender = ensure_signed(origin)?;

			ensure!(<Properties<T>>::exists(property_id), "The property is not exist");

			ensure!(<ManagersIndex<T>>::exists(sender.clone()), "The sender is not a manager");

			let mut property = Self::property(property_id);

			property.is_authenticated = is_authenticated;

			<Properties<T>>::insert(property_id, property);

			Self::deposit_event(RawEvent::Authenticated(sender, property_id, is_authenticated));

			Ok(())
		}

		pub fn sell(origin, property_id: T::Hash, price: T::Balance) -> Result {
			let sender = ensure_signed(origin)?;

			ensure!(<Properties<T>>::exists(property_id), "The property is not exist");

			let owner = Self::property_owner(property_id);

			ensure!(owner == sender, "You do not own this property");

			ensure!(!<PropertyForsaleIndex<T>>::exists(property_id), "The property is already for sale");

			let property = Self::property(property_id);
			ensure!(property.is_authenticated, "The property is not authenticated");

			let property_forsale_count = Self::property_forsale_count();
			let new_property_forsale_count = property_forsale_count.checked_add(1).ok_or("Overflow when adding new property")?;

			<PropertiesForsale<T>>::insert(property_forsale_count, (property_id, price, false, None));
			<PropertyForsaleCount<T>>::put(new_property_forsale_count);
			<PropertyForsaleIndex<T>>::insert(property_id, property_forsale_count);

			Self::deposit_event(RawEvent::PropertyForsale(property_id, price));

			Ok(())
		}

		pub fn buy(origin, property_id: T::Hash) -> Result {
			let sender = ensure_signed(origin)?;

			ensure!(<Properties<T>>::exists(property_id), "The property is not exist");

			let owner = Self::property_owner(property_id);

			ensure!(owner != sender, "You can not buy your own property");

			ensure!(<PropertyForsaleIndex<T>>::exists(property_id), "The property is not ready for sale");

			let property_forsale_index = <PropertyForsaleIndex<T>>::get(property_id);
			let (_, price, is_lock, buyer) = Self::forsale_property(property_forsale_index);
			ensure!(is_lock == false, "The property is locked by another buyer");
			ensure!(buyer == None, "The property is locked by another buyer");
			
			ensure!(<balances::Module<T>>::free_balance(&sender) >= price, "You don't have enough free balance to buy this property");

			<balances::Module<T>>::reserve(&sender, price)?;

			<PropertiesForsale<T>>::insert(property_forsale_index, (property_id, price, true, Some(sender.clone())));

			Self::deposit_event(RawEvent::BuyProperty(property_id, sender, owner));
			
			Ok(())
		}

		pub fn authenticate_trade(origin, property_id: T::Hash) -> Result {
			let sender = ensure_signed(origin)?;

			ensure!(<Properties<T>>::exists(property_id), "The property is not exist");

			ensure!(<ManagersIndex<T>>::exists(sender.clone()), "The sender is not a manager");

			ensure!(<PropertyForsaleIndex<T>>::exists(property_id), "The property is not ready for sale");

			let property_forsale_index = <PropertyForsaleIndex<T>>::get(property_id);
			let (_, price, is_lock, buyer_option) = Self::forsale_property(property_forsale_index);
			ensure!(is_lock == true, "The property is unlocked");
			ensure!(buyer_option != None, "There is no buyer for this property");
			let buyer = buyer_option.clone().unwrap();

			<balances::Module<T>>::unreserve(&buyer, price);
			
			let property_forsale_count = Self::property_forsale_count();
			let new_property_forsale_count = property_forsale_count.checked_sub(1).ok_or("Underflow when decrease a kitty")?;
			if (property_forsale_index != new_property_forsale_count) {
				let last_property_forsale_info = Self::forsale_property(new_property_forsale_count);
				<PropertiesForsale<T>>::insert(new_property_forsale_count, (property_id, price, is_lock, buyer_option));
				<PropertiesForsale<T>>::insert(property_forsale_index, last_property_forsale_info);
			}

			<PropertiesForsale<T>>::remove(new_property_forsale_count);
			<PropertyForsaleIndex<T>>::remove(property_id);
			<PropertyForsaleCount<T>>::put(new_property_forsale_count);
			
			let owner = Self::property_owner(property_id);

			<PropertyOwner<T>>::insert(property_id, &buyer);
			<balances::Module<T> as Currency<_>>::transfer(&buyer, &owner, price)?;

			Self::deposit_event(RawEvent::TradeAuthenticated(property_id, buyer, owner, sender));

			Ok(())
		}
	}
}

decl_event!(
	pub enum Event<T> where
	<T as system::Trait>::AccountId,
	<T as system::Trait>::Hash,
	<T as balances::Trait>::Balance
	{
		// Just a dummy event.
		// Event `Something` is declared with a parameter of the type `u32` and `AccountId`
		// To emit this event, we call the deposit funtion, from our runtime funtions
		SomethingStored(u32, AccountId),
		Authenticated(AccountId, Hash, bool),
		ManagerAdded(AccountId),
		PropertyForsale(Hash, Balance),
		BuyProperty(Hash, AccountId, AccountId),
		TradeAuthenticated(Hash, AccountId, AccountId, AccountId),
	}
);

/// tests for this module
#[cfg(test)]
mod tests {
	use super::*;

	use runtime_io::with_externalities;
	use primitives::{H256, Blake2Hasher};
	use support::{impl_outer_origin, assert_ok};
	use runtime_primitives::{
		BuildStorage,
		traits::{BlakeTwo256, IdentityLookup},
		testing::{Digest, DigestItem, Header}
	};

	impl_outer_origin! {
		pub enum Origin for Test {}
	}

	// For testing the module, we construct most of a mock runtime. This means
	// first constructing a configuration type (`Test`) which `impl`s each of the
	// configuration traits of modules we want to use.
	#[derive(Clone, Eq, PartialEq)]
	pub struct Test;
	impl system::Trait for Test {
		type Origin = Origin;
		type Index = u64;
		type BlockNumber = u64;
		type Hash = H256;
		type Hashing = BlakeTwo256;
		type Digest = Digest;
		type AccountId = u64;
		type Lookup = IdentityLookup<Self::AccountId>;
		type Header = Header;
		type Event = ();
		type Log = DigestItem;
	}
	impl Trait for Test {
		type Event = ();
	}
	type RealEstateModule = Module<Test>;

	// This function basically just builds a genesis storage key/value store according to
	// our desired mockup.
	fn new_test_ext() -> runtime_io::TestExternalities<Blake2Hasher> {
		system::GenesisConfig::<Test>::default().build_storage().unwrap().0.into()
	}

	#[test]
	fn it_works_for_default_value() {
		with_externalities(&mut new_test_ext(), || {
			// Just a dummy test for the dummy funtion `do_something`
			// calling the `do_something` function with a value 42
			assert_ok!(RealEstateModule::do_something(Origin::signed(1), 42));
			// asserting that the stored value is equal to what we stored
			assert_eq!(RealEstateModule::something(), Some(42));
		});
	}
}
