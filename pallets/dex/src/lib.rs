// Humidefi Dex Pallet- By: hgminerva-20220709
// References
// 1. https://github.com/substrate-developer-hub/substrate-how-to-guides/tree/d3602a66d66be5b013f2e3330081ea4e0d6dd978/example-code/template-node/pallets/reward-coin
// 2. https://mlink.in/qa/?qa=1117564/ (String->Vec<u8>)
// 3. https://www.youtube.com/watch?v=69uCTnvzL60&t=1392s
// 4. https://chowdera.com/2021/08/20210809112247168h.html
// 5. https://github.com/gautamdhameja/substrate-runtime-contract-sample/blob/master/pallets/template/src/lib.rs
// 6. https://stackoverflow.com/questions/70559578/substrate-pallet-loosely-coupling-example-between-2-custom-pallets
// 7. https://github.com/justinFrevert/Runtime-Contract-Interactions/blob/master/pallets/template/src/lib.rs
#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;
//#[cfg(test)]
//mod mock;

//#[cfg(test)]
//mod tests;

//#[cfg(feature = "runtime-benchmarks")]
//mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use frame_support::inherent::Vec;
	use frame_support::traits::Currency;
	use frame_support::traits::ExistenceRequirement;
	use pallet_contracts::chain_extension::UncheckedFrom;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_contracts::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Currency: Currency<Self::AccountId>;
	}

	//pub type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
	type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
	type PHPUBalanceOf<T> = <<T as pallet_contracts::Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	#[pallet::pallet]
	#[pallet::without_storage_info]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn something)]
	pub type Something<T> = StorageValue<_, u32>;

	#[pallet::storage]
	#[pallet::getter(fn ticker_data)]
	pub type TickerDataStore<T> = StorageValue<_, Vec<u8>>;

	/// Holder of the DEX income 
	#[pallet::storage]
	#[pallet::getter(fn dex_account)]
	pub type DexDataStore<T: Config> = StorageValue<_, T::AccountId>;

	/// Holder of the PHPU contract
	#[pallet::storage]
	#[pallet::getter(fn phpu_account)]
	pub type PhpuDataStore<T: Config> = StorageValue<_, T::AccountId>;	

	/// Holder of the PHPU Liquidity Pool Contract
	#[pallet::storage]
	#[pallet::getter(fn phpu_liquidity_account)]
	pub type PhpuLiquidityDataStore<T: Config> = StorageValue<_, T::AccountId>;	

	/// Holder of the UMI Liquidity Pool Contract
	#[pallet::storage]
	#[pallet::getter(fn umi_liquidity_account)]
	pub type UmiLiquidityDataStore<T: Config> = StorageValue<_, T::AccountId>;	

	/// Swap fees in percentage
	#[pallet::storage]
	#[pallet::getter(fn swap_fees)]
	pub type SwapFeesDataStore<T: Config> = StorageValue<_, u32>;	

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32, T::AccountId),
		/// When there is a new ticker data stored
		TickerPriceDataStored(Vec<u8>),
		/// When there is a new DEX account stored
		DexAccountDataStored(T::AccountId),
		/// When there is a new PHPU account stored
		PhpuAccountDataStored(T::AccountId),
		/// When there is a new UMI liquidity account stored
		UmiLiquidityAccountDataStored(T::AccountId),
		/// When there is a new PHPU liquidity account stored
		PhpuLiquidityAccountDataStored(T::AccountId),
		/// When there is a new income account stored
		DexIncomeAccountDataStored(T::AccountId),
		/// When there is a swap
		SwapExecuted(T::AccountId),
		/// When there is a stake
		StakeExecuted(T::AccountId),
		/// When there is a redeem
		RedeemExecuted(T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		NoneValue,
		NoTickerPriceData,
		TickerPriceDataInvalid,
		TickerPriceInvalid,
		NoDexAccount,
		NoPhpuAccount,
		StorageOverflow,
		InsufficientFunds,
		NoUmiLiquidityAccount,
		NoPhpuLiquidityAccount,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> 
	where T::AccountId: UncheckedFrom<T::Hash>, T::AccountId: AsRef<[u8]>,
	{
		/// Setup the ticker prices
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn store_ticker_price(origin: OriginFor<T>, ticker_data: Vec<u8>) -> DispatchResult {
			ensure_root(origin)?;

			<TickerDataStore<T>>::put(ticker_data.clone());

			Self::deposit_event(Event::TickerPriceDataStored(ticker_data));

			Ok(())
		}

		/// Setup the DEX account
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn store_dex_account(origin: OriginFor<T>, dex_account: T::AccountId) -> DispatchResult {
			ensure_root(origin)?;

			<DexDataStore<T>>::put(dex_account.clone());

			Self::deposit_event(Event::DexAccountDataStored(dex_account));

			Ok(())
		}

		/// Setup the PHPU Smart Contract Account
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn store_phpu_account(origin: OriginFor<T>, phpu_account: T::AccountId) -> DispatchResult {
			ensure_root(origin)?;

			<PhpuDataStore<T>>::put(phpu_account.clone());

			Self::deposit_event(Event::PhpuAccountDataStored(phpu_account));

			Ok(())
		}	

		/// Setup the UMI Liquidity Smart Contract Account
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn store_umi_liquidity_account(origin: OriginFor<T>, umi_liquidity_account: T::AccountId) -> DispatchResult {
			ensure_root(origin)?;

			<UmiLiquidityDataStore<T>>::put(umi_liquidity_account.clone());

			Self::deposit_event(Event::UmiLiquidityAccountDataStored(umi_liquidity_account));

			Ok(())
		}	

		/// Setup the DEX Income Account
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn store_phpu_liquidity_account(origin: OriginFor<T>, phpu_liquidity_account: T::AccountId) -> DispatchResult {
			ensure_root(origin)?;

			<PhpuLiquidityDataStore<T>>::put(phpu_liquidity_account.clone());

			Self::deposit_event(Event::PhpuLiquidityAccountDataStored(phpu_liquidity_account));

			Ok(())
		}	

		/// Swap token
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn do_swap(origin: OriginFor<T>, source: T::AccountId, quantity:  BalanceOf<T>, source_ticker: Vec<u8>, destination_ticker: Vec<u8>,) -> DispatchResult {
			// from source: quantity <source_ticker> => to dex: quantity <source_ticker>
			// from dex: converted_quantity <destination_ticker> => to source: converted_quantity <destination_ticker>
			
			let _who = ensure_signed(origin)?;
			//let ticker_prices = TickerDataStore::<T>::get(); 

			let ticker1 = scale_info::prelude::string::String::from_utf8(source_ticker).expect("Invalid");
			let ticker2 = scale_info::prelude::string::String::from_utf8(destination_ticker).expect("Invalid");

			// PHPU Stable Coin Contract Selector
			let gas_limit = 10_000_000_000;
			let debug = false;
			let mut message_selector: Vec<u8> = [0x84, 0xA1, 0x5D, 0xA1].into();
			let value: PHPUBalanceOf<T> = Default::default();

			let decimal: Option<BalanceOf<T>> = 100_000_000_000_000u64.try_into().ok();
			let decimal_multiplier;
			match decimal { Some(multiplier) => { decimal_multiplier = multiplier; },  None => { decimal_multiplier = Default::default(); } };

			// UMI -> PHPU
			// From Source send UMI to DEX
			// From DEX send equivalent PHPU to source
			if ticker1.eq("UMI") && ticker2.eq("PHPU") {
				match TickerDataStore::<T>::get() {
					Some(ticker_price_data) => {
						let value_string = scale_info::prelude::string::String::from_utf8(ticker_price_data).expect("Invalid");
						let v: serde_json::Value = serde_json::from_str(&value_string).map_err(|_| <Error<T>>::TickerPriceDataInvalid)?;
						let umi_price: u64 = serde_json::from_value(v[0]["price_in_usd"].clone()).map_err(|_| <Error<T>>::TickerPriceInvalid)?;
						let phpu_price: u64 = serde_json::from_value(v[1]["price_in_usd"].clone()).map_err(|_| <Error<T>>::TickerPriceInvalid)?;
						let umi_price_balance: Option<BalanceOf<T>> = umi_price.try_into().ok();
						let phpu_price_balance: Option<BalanceOf<T>> = phpu_price.try_into().ok();
						
						let umi_multiplier;
						let phpu_multiplier;
						match umi_price_balance { Some(multiplier) => { umi_multiplier = multiplier; },  None => { umi_multiplier = Default::default(); } };
						match phpu_price_balance { Some(multiplier) => { phpu_multiplier = multiplier; },  None => { phpu_multiplier = Default::default(); } };
						
						match DexDataStore::<T>::get() {
							Some(dex_account) => {
								match PhpuDataStore::<T>::get() {
									Some(phpu_account) => {
										// Transfer from Source send UMI to DEX
										let umi = quantity * decimal_multiplier.clone();
										<T as Config>::Currency::transfer(&source, &dex_account, umi, ExistenceRequirement::KeepAlive)?;
										// Transfer from DEX send equivalent PHPU to source
										let factor = (phpu_multiplier.clone()*decimal_multiplier.clone()) / umi_multiplier.clone();
										let mut to = source.encode();
										let mut phpu =( quantity * factor.clone()).encode();
										let mut data = Vec::new();
										data.append(&mut message_selector);
										data.append(&mut to);
										data.append(&mut phpu);
										pallet_contracts::Pallet::<T>::bare_call(
											dex_account, 			
											phpu_account,			
											value,
											gas_limit,
											None,
											data,
											debug,
										).result?;
									}, None => return Err(Error::<T>::NoPhpuAccount.into()),
								}
							} , None => return Err(Error::<T>::NoDexAccount.into()),
						} 
					}, None => return Err(Error::<T>::NoTickerPriceData.into()),
				}	
			}

			// PHPU -> UMI
			// From Source send PHPU to DEX
			// From DEX send equivalent UMI to source
			if ticker1.eq("PHPU") && ticker2.eq("UMI") {
				match TickerDataStore::<T>::get() {
					Some(ticker_price_data) => {
						let value_string = scale_info::prelude::string::String::from_utf8(ticker_price_data).expect("Invalid");
						let v: serde_json::Value = serde_json::from_str(&value_string).map_err(|_| <Error<T>>::TickerPriceDataInvalid)?;
						let umi_price: u64 = serde_json::from_value(v[0]["price_in_usd"].clone()).map_err(|_| <Error<T>>::TickerPriceInvalid)?;
						let phpu_price: u64 = serde_json::from_value(v[1]["price_in_usd"].clone()).map_err(|_| <Error<T>>::TickerPriceInvalid)?;
						let umi_price_balance: Option<BalanceOf<T>> = umi_price.try_into().ok();
						let phpu_price_balance: Option<BalanceOf<T>> = phpu_price.try_into().ok();

						let umi_multiplier;
						let phpu_multiplier;
						match umi_price_balance { Some(multiplier) => { umi_multiplier = multiplier; },  None => { umi_multiplier = Default::default(); } };
						match phpu_price_balance { Some(multiplier) => { phpu_multiplier = multiplier; },  None => { phpu_multiplier = Default::default(); } };

						match DexDataStore::<T>::get() {
							Some(dex_account) => {
								match PhpuDataStore::<T>::get() {
									Some(phpu_account) => {
										// Transfer from source send equivalent PHPU to DEX
										let mut to = dex_account.encode();
										let mut phpu = (quantity * decimal_multiplier.clone()).encode();
										let mut data = Vec::new();
										data.append(&mut message_selector);
										data.append(&mut to);
										data.append(&mut phpu);
										pallet_contracts::Pallet::<T>::bare_call(
											source.clone(), 			
											phpu_account,			
											value,
											gas_limit,
											None,
											data,
											debug,
										).result?;
										// Transfer from DEX send UMI to Source
										let factor = (umi_multiplier.clone()*decimal_multiplier.clone()) / phpu_multiplier.clone();
										let umi = quantity * factor.clone();
										<T as Config>::Currency::transfer(&dex_account, &source, umi, ExistenceRequirement::KeepAlive)?;
									}, None => return Err(Error::<T>::NoPhpuAccount.into()),
								}
							}, None => return Err(Error::<T>::NoDexAccount.into()),
						}
					}, None => return Err(Error::<T>::NoTickerPriceData.into()),
				}
			}

			Self::deposit_event(Event::SwapExecuted(source));
			
			Ok(())
		}

		/// Stake token to get liquidity token
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn do_liquidity_stake(origin: OriginFor<T>, source: T::AccountId, quantity:  BalanceOf<T>,source_ticker: Vec<u8>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			let ticker = scale_info::prelude::string::String::from_utf8(source_ticker).expect("Invalid");
			
			if ticker.eq("UMI")  {
				match UmiLiquidityDataStore::<T>::get() {
					Some(umi_liquidity_account) => {
						// STEP 1: Transfer UMI to UMI Liquidity Account
						<T as Config>::Currency::transfer(&source, &umi_liquidity_account, quantity, ExistenceRequirement::KeepAlive)?;
						// STEP 2: Mint lUMI and transfer to the source
						// Contract settings
						let gas_limit:u64 = 10_000_000_000;
						let debug = false;
						let mut message_selector: Vec<u8> = [0x1D, 0x2F, 0x13, 0xC5].into();
						let contract_value: PHPUBalanceOf<T> = Default::default();
						// Message and parameters
						let mut to = source.encode();
						let mut value = quantity.encode();
						let mut data = Vec::new();
						data.append(&mut message_selector);
						data.append(&mut to);
						data.append(&mut value);
						// Call
						pallet_contracts::Pallet::<T>::bare_call(
							source.clone(), 			
							umi_liquidity_account,			
							contract_value,
							gas_limit,
							None,
							data,
							debug,
						).result?;
					}, 
					None => return Err(Error::<T>::NoUmiLiquidityAccount.into()),
				}
			} 

			if ticker.eq("PHPU")  {
				match PhpuLiquidityDataStore::<T>::get() {
					Some(phpu_liquidity_account) => {
						match PhpuDataStore::<T>::get() {
							Some(phpu_account) => {
								// STEP 1: Transfer PHPU to PHPU Liquidity Account
								let gas_limit:u64 = 10_000_000_000;
								let debug = false;
								let mut message_selector: Vec<u8> = [0x84, 0xA1, 0x5D, 0xA1].into();
								let contract_value: PHPUBalanceOf<T> = Default::default();
								let mut phpu_to = source.encode();
								let mut phpu_value = quantity.encode();
								let mut phpu_data = Vec::new();
								phpu_data.append(&mut message_selector);
								phpu_data.append(&mut phpu_to);
								phpu_data.append(&mut phpu_value);
								pallet_contracts::Pallet::<T>::bare_call(
									source.clone(), 			
									phpu_account.clone(),			
									contract_value.clone(),
									gas_limit.clone(),
									None,
									phpu_data,
									debug,
								).result?;
								// STEP 2: Mint lPHPU and transfer to the source
								// Contract settings
								message_selector = [0x1D, 0x2F, 0x13, 0xC5].into();
								// Message and parameters
								let mut lphpu_to = source.encode();
								let mut lphpu_value = quantity.encode();
								let mut lphpu_data = Vec::new();
								lphpu_data.append(&mut message_selector);
								lphpu_data.append(&mut lphpu_to);
								lphpu_data.append(&mut lphpu_value);
								// Call
								pallet_contracts::Pallet::<T>::bare_call(
									source.clone(), 			
									phpu_liquidity_account,			
									contract_value,
									gas_limit,
									None,
									lphpu_data,
									debug,
								).result?;
							}, None => return Err(Error::<T>::NoPhpuAccount.into()),
						}
					}, 
					None => return Err(Error::<T>::NoPhpuLiquidityAccount.into()),
				}
			} 

			Self::deposit_event(Event::StakeExecuted(source));
			Ok(())
		}

		/// Redeem the liquidity token
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn do_liquidity_redeem(origin: OriginFor<T>, source: T::AccountId, quantity:  BalanceOf<T>,source_ticker: Vec<u8>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			let ticker = scale_info::prelude::string::String::from_utf8(source_ticker).expect("Invalid");

			Self::deposit_event(Event::RedeemExecuted(source));
			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match <Something<T>>::get() {
				// Return an error if the value has not been set.
				None => return Err(Error::<T>::NoneValue.into()),
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					<Something<T>>::put(new);

					Ok(())
				},
			}
		}

	}
}
