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
	type LUMIBalanceOf<T> = <<T as pallet_contracts::Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
	type LPHPUBalanceOf<T> = <<T as pallet_contracts::Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

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
	pub type DexAccountDataStore<T: Config> = StorageValue<_, T::AccountId>;

	/// PHPU Liquidity Pool Account
	#[pallet::storage]
	#[pallet::getter(fn phpu_liquidity_account)]
	pub type PhpuLiquidityAccountDataStore<T: Config> = StorageValue<_, T::AccountId>;	

	/// UMI Liquidity Pool Account
	#[pallet::storage]
	#[pallet::getter(fn umi_liquidity_account)]
	pub type UmiLiquidityAccountDataStore<T: Config> = StorageValue<_, T::AccountId>;	

	/// PHPU contract
	#[pallet::storage]
	#[pallet::getter(fn phpu_contract)]
	pub type PhpuDataStore<T: Config> = StorageValue<_, T::AccountId>;	

	/// PHPU Liquidity Pool Contract
	#[pallet::storage]
	#[pallet::getter(fn phpu_liquidity_contract)]
	pub type PhpuLiquidityDataStore<T: Config> = StorageValue<_, T::AccountId>;	

	/// UMI Liquidity Pool Contract
	#[pallet::storage]
	#[pallet::getter(fn umi_liquidity_contract)]
	pub type UmiLiquidityDataStore<T: Config> = StorageValue<_, T::AccountId>;	

	/// Swap fees in percentage
	#[pallet::storage]
	#[pallet::getter(fn swap_fees)]
	pub type SwapFeesDataStore<T: Config> = StorageValue<_, u64>;	

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
		/// When there is a new UMI liquidity account stored
		UmiLiquidityAccountDataStored(T::AccountId),
		/// When there is a new PHPU liquidity account stored
		PhpuLiquidityAccountDataStored(T::AccountId),
		/// When there is a new PHPU contract stored
		PhpuContractDataStored(T::AccountId),
		/// When there is a new UMI liquidity contract
		UmiLiquidityContractDataStored(T::AccountId),
		/// When there is a new PHPU liquidity contract
		PhpuLiquidityContractDataStored(T::AccountId),
		/// When there is a new income account stored
		DexIncomeAccountDataStored(T::AccountId),
		/// When there is a swap
		SwapExecuted(T::AccountId),
		/// When there is a stake
		StakeExecuted(T::AccountId),
		/// When there is a redeem
		RedeemExecuted(T::AccountId),
		/// When PHPU is sent
		SentPhpu(T::AccountId),
		/// When there is a new swap fees
		SwapFeesDataStored(u64),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		NoneValue,
		NoTickerPriceData,
		TickerPriceDataInvalid,
		TickerPriceInvalid,
		NoDexAccount,
		NoUmiLiquidityAccount,
		NoPhpuLiquidityAccount,
		StorageOverflow,
		InsufficientFunds,
		NoPhpuContract,
		NoUmiLiquidityContract,
		NoPhpuLiquidityContract,
		NoSwapFees,
		NoLiquiditySupply,
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

			<DexAccountDataStore<T>>::put(dex_account.clone());

			Self::deposit_event(Event::DexAccountDataStored(dex_account));

			Ok(())
		}

		/// Setup the UMI Liquidity Account
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn store_umi_liquidity_account(origin: OriginFor<T>, umi_liquidity_account: T::AccountId) -> DispatchResult {
			ensure_root(origin)?;

			<UmiLiquidityAccountDataStore<T>>::put(umi_liquidity_account.clone());

			Self::deposit_event(Event::UmiLiquidityAccountDataStored(umi_liquidity_account));

			Ok(())
		}	

		/// Setup the PHPU Liquidity Account
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn store_phpu_liquidity_account(origin: OriginFor<T>, phpu_liquidity_account: T::AccountId) -> DispatchResult {
			ensure_root(origin)?;

			<PhpuLiquidityAccountDataStore<T>>::put(phpu_liquidity_account.clone());

			Self::deposit_event(Event::PhpuLiquidityAccountDataStored(phpu_liquidity_account));

			Ok(())
		}	

		/// Setup the PHPU Smart Contract
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn store_phpu_contract(origin: OriginFor<T>, phpu_contract: T::AccountId) -> DispatchResult {
			ensure_root(origin)?;

			<PhpuDataStore<T>>::put(phpu_contract.clone());

			Self::deposit_event(Event::PhpuContractDataStored(phpu_contract));

			Ok(())
		}	

		/// Setup the UMI Liquidity Smart Contract
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn store_umi_liquidity_contract(origin: OriginFor<T>, umi_liquidity_contract: T::AccountId) -> DispatchResult {
			ensure_root(origin)?;

			<UmiLiquidityDataStore<T>>::put(umi_liquidity_contract.clone());

			Self::deposit_event(Event::UmiLiquidityContractDataStored(umi_liquidity_contract));

			Ok(())
		}	

		/// Setup the PHPU Liquidity Smart Contract
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn store_phpu_liquidity_contract(origin: OriginFor<T>, phpu_liquidity_contract: T::AccountId) -> DispatchResult {
			ensure_root(origin)?;

			<PhpuLiquidityDataStore<T>>::put(phpu_liquidity_contract.clone());

			Self::deposit_event(Event::PhpuLiquidityContractDataStored(phpu_liquidity_contract));

			Ok(())
		}	

		/// Setup swap fees
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn store_swap_fees(origin: OriginFor<T>, swap_fees: u64) -> DispatchResult {
			ensure_root(origin)?;

			<SwapFeesDataStore<T>>::put(swap_fees.clone());

			Self::deposit_event(Event::SwapFeesDataStored(swap_fees));

			Ok(())
		}

		// Swap ticker/token
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn do_swap(origin: OriginFor<T>, source: T::AccountId, quantity:  BalanceOf<T>, source_ticker: Vec<u8>, destination_ticker: Vec<u8>,) -> DispatchResult  {
			let _who = ensure_signed(origin)?;

			let ticker1 = scale_info::prelude::string::String::from_utf8(source_ticker).expect("Invalid");
			let ticker2 = scale_info::prelude::string::String::from_utf8(destination_ticker).expect("Invalid");

			// Decimal (14)
			let decimal: Option<BalanceOf<T>> = 100_000_000_000_000u64.try_into().ok();
			let decimal_multiplier;
			match decimal { Some(multiplier) => { decimal_multiplier = multiplier; },  None => { decimal_multiplier = Default::default(); } };

			// Contract settings
			let phpu_contract_gas_limit:u64 = 10_000_000_000;
			let phpu_contract_debug = false;
			let phpu_contract_value: PHPUBalanceOf<T> = Default::default();
			let mut phpu_contract_transfer_message_selector: Vec<u8> = [0xdb, 0x20, 0xf9, 0xf5].into();
			let mut phpu_contract_swap_message_selector: Vec<u8> = [0xc4, 0x05, 0xe9, 0x82].into();

			let dex_account;
			match DexAccountDataStore::<T>::get() { 
				Some(some_dex_account) => dex_account = some_dex_account,
				None => return Err(Error::<T>::NoDexAccount.into()),
			}

			let phpu_liquidity_account;
			match PhpuLiquidityAccountDataStore::<T>::get() { 
				Some(some_phpu_liquidity_account) => phpu_liquidity_account = some_phpu_liquidity_account,
				None => return Err(Error::<T>::NoPhpuLiquidityAccount.into()),
			}

			let umi_liquidity_account;
			match UmiLiquidityAccountDataStore::<T>::get() { 
				Some(some_umi_liquidity_account) => umi_liquidity_account = some_umi_liquidity_account,
				None => return Err(Error::<T>::NoUmiLiquidityAccount.into()),
			}

			let phpu_contract;
			match PhpuDataStore::<T>::get() {
				Some(some_phpu_contract) => phpu_contract = some_phpu_contract,
				None => return Err(Error::<T>::NoPhpuContract.into()),
			}

			let swap_fees;
			match SwapFeesDataStore::<T>::get() {
				Some(some_swap_fees) => swap_fees = some_swap_fees,
				None => return Err(Error::<T>::NoSwapFees.into()),
			}

			// Primitive quantity - used for conversion computation
			let primitive_quantity: u64;
			let try_primitive_quantity = TryInto::<u64>::try_into(quantity).ok();
			match try_primitive_quantity { 
				Some(some_primitive_quantity) => primitive_quantity = some_primitive_quantity, 
				None => primitive_quantity = 0u64,
			}			

			// Ticker prices
			let ticker_price_data;
			match TickerDataStore::<T>::get() {
				Some(some_ticker_price_data) => ticker_price_data = some_ticker_price_data,
				None => return Err(Error::<T>::NoTickerPriceData.into()),
			}
			let value_string = scale_info::prelude::string::String::from_utf8(ticker_price_data).expect("Invalid");
			let v: serde_json::Value = serde_json::from_str(&value_string).map_err(|_| <Error<T>>::TickerPriceDataInvalid)?;
			let umi_price: u64 = serde_json::from_value(v[0]["price_in_usd"].clone()).map_err(|_| <Error<T>>::TickerPriceInvalid)?;
			let phpu_price: u64 = serde_json::from_value(v[1]["price_in_usd"].clone()).map_err(|_| <Error<T>>::TickerPriceInvalid)?;

			if ticker1.eq("UMI") && ticker2.eq("PHPU") {

				// Step 1: Transfer UMI to Liquidity Account
				let umi = quantity * decimal_multiplier.clone();
				<T as Config>::Currency::transfer(&source, &umi_liquidity_account, umi, ExistenceRequirement::KeepAlive)?;

				// Step 2: Compute for conversion
				let floating_conversion = (phpu_price as f64) / (umi_price as f64);
				let floating_decimal_multiplier = 100_000_000_000_000f64;
				let floating_swap_fee = (swap_fees as f64) / 100f64;
				let floating_primitive_quantity = primitive_quantity as f64;

				let primitive_income_quantity = floating_primitive_quantity * floating_swap_fee * floating_conversion * floating_decimal_multiplier;
				let primitive_net_quantity = (floating_primitive_quantity * floating_decimal_multiplier * floating_conversion) - primitive_income_quantity;

				let income_quantity: PHPUBalanceOf<T>;
				let try_income_quantity: Option<PHPUBalanceOf<T>> = (primitive_income_quantity as u64).try_into().ok();
				match try_income_quantity { 
					Some(some_income_quantity) => { income_quantity = some_income_quantity; },  
					None => { income_quantity = Default::default(); } 
				}

				let net_quantity: PHPUBalanceOf<T>;
				let try_total_quantity: Option<PHPUBalanceOf<T>> = (primitive_net_quantity as u64).try_into().ok();
				match try_total_quantity { 
					Some(some_total_quantity) => { net_quantity = some_total_quantity; },  
					None => { net_quantity = Default::default(); } 
				}
				
				// Step 3: Transfer PHPU to Source and DEX
				let mut source_to = source.encode();
				let mut source_phpu = net_quantity.encode();
				let mut collector_to = dex_account.encode();
				let mut collector_fee_phpu = income_quantity.encode();

				let mut phpu_data = Vec::new();
				phpu_data.append(&mut phpu_contract_swap_message_selector);
				phpu_data.append(&mut source_to);
				phpu_data.append(&mut source_phpu);
				phpu_data.append(&mut collector_to);
				phpu_data.append(&mut collector_fee_phpu);
	
				let _ = pallet_contracts::Pallet::<T>::bare_call(
					phpu_liquidity_account.clone(), 			
					phpu_contract.clone(),			
					phpu_contract_value,
					phpu_contract_gas_limit,
					None,
					phpu_data,
					phpu_contract_debug,
				);

			}

			if ticker1.eq("PHPU") && ticker2.eq("UMI") {

				// Step 1: Transfer PHPU to Liquidity Account				
				let mut phpu_to = phpu_liquidity_account.encode();
				let mut phpu_value = (quantity * decimal_multiplier).encode();
				let mut phpu_remarks = "Transfer source PHPU".encode();

				let mut phpu_data = Vec::new();
				phpu_data.append(&mut phpu_contract_transfer_message_selector);
				phpu_data.append(&mut phpu_to);
				phpu_data.append(&mut phpu_value);
				phpu_data.append(&mut phpu_remarks);

				let _ = pallet_contracts::Pallet::<T>::bare_call(
					source.clone(), 			
					phpu_contract.clone(),			
					phpu_contract_value.clone(),
					phpu_contract_gas_limit.clone(),
					None,
					phpu_data,
					phpu_contract_debug,
				);

				// Step 2: Compute for conversion
				let floating_conversion = (umi_price as f64) / (phpu_price as f64);
				let floating_decimal_multiplier = 100_000_000_000_000f64;
				let floating_swap_fee = (swap_fees as f64) / 100f64;
				let floating_primitive_quantity = primitive_quantity as f64;

				let primitive_income_quantity = floating_primitive_quantity * floating_swap_fee * floating_conversion * floating_decimal_multiplier;
				let primitive_net_quantity = (floating_primitive_quantity * floating_decimal_multiplier * floating_conversion) - primitive_income_quantity;

				let income_quantity;
				let try_income_quantity: Option<BalanceOf<T>> = (primitive_income_quantity as u64).try_into().ok();
				match try_income_quantity { 
					Some(some_income_quantity) => { income_quantity = some_income_quantity; },  
					None => { income_quantity = Default::default(); } 
				}

				let net_quantity;
				let try_total_quantity: Option<BalanceOf<T>> = (primitive_net_quantity as u64).try_into().ok();
				match try_total_quantity { 
					Some(some_total_quantity) => { net_quantity = some_total_quantity; },  
					None => { net_quantity = Default::default(); } 
				}

				// Step 3: Transfer UMI to Source and DEX Account
				<T as Config>::Currency::transfer(&umi_liquidity_account, &source, net_quantity, ExistenceRequirement::KeepAlive)?;
				<T as Config>::Currency::transfer(&umi_liquidity_account, &dex_account, income_quantity, ExistenceRequirement::KeepAlive)?;

			}

			Self::deposit_event(Event::SwapExecuted(source));
			Ok(())

		}

		/// Stake token to get liquidity token
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn do_liquidity_stake(origin: OriginFor<T>, source: T::AccountId, quantity:  BalanceOf<T>,source_ticker: Vec<u8>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			let ticker = scale_info::prelude::string::String::from_utf8(source_ticker).expect("Invalid");

			// Decimal (14)
			let decimal: Option<BalanceOf<T>> = 100_000_000_000_000u64.try_into().ok();
			let decimal_multiplier;
			match decimal { Some(multiplier) => { decimal_multiplier = multiplier; },  None => { decimal_multiplier = Default::default(); } };
			
			// Contract settings
			let default_contract_gas_limit:u64 = 10_000_000_000;
			let default_contract_debug = false;
			let default_contract_value: PHPUBalanceOf<T> = Default::default();

			let mut liquidity_contract_mint_to_message_selector: Vec<u8> = [0x1d, 0x2f, 0x13, 0xc5].into();
			let mut phpu_contract_transfer_message_selector: Vec<u8> = [0xdb, 0x20, 0xf9, 0xf5].into();

			if ticker.eq("UMI")  {
				match UmiLiquidityAccountDataStore::<T>::get() {
					Some(umi_liquidity_account) => {

						match UmiLiquidityDataStore::<T>::get() {
							Some(umi_liquidity_contract) => {

								// STEP 1: Transfer UMI to UMI Liquidity Account
								let umi = quantity * decimal_multiplier;
								<T as Config>::Currency::transfer(&source, &umi_liquidity_account, umi, ExistenceRequirement::KeepAlive)?;

								// STEP 2: Mint and transfer lUMI to source
								let mut lumi_to = source.encode();
								let mut lumi_value = (quantity * decimal_multiplier).encode();
								let mut lumi_data = Vec::new();
								lumi_data.append(&mut liquidity_contract_mint_to_message_selector);
								lumi_data.append(&mut lumi_to);
								lumi_data.append(&mut lumi_value);
								let _ = pallet_contracts::Pallet::<T>::bare_call(
									source.clone(), 			
									umi_liquidity_contract,			
									default_contract_value,
									default_contract_gas_limit,
									None,
									lumi_data,
									default_contract_debug,
								);

							},  None => return Err(Error::<T>::NoUmiLiquidityContract.into()),
						}

					},  None => return Err(Error::<T>::NoUmiLiquidityAccount.into()),
				}

			} 

			if ticker.eq("PHPU")  {
				match PhpuLiquidityAccountDataStore::<T>::get() {
					Some(phpu_liquidity_account) => {

						match PhpuLiquidityDataStore::<T>::get() {
							Some(phpu_liquidity_contract) => {

								match PhpuDataStore::<T>::get() {
									Some(phpu_contract) => {
		
										// STEP 1: Transfer PHPU to PHPU Liquidity Account								
										let mut phpu_to = phpu_liquidity_account.encode();
										let mut phpu_value = (quantity * decimal_multiplier).encode();
										let mut phpu_remarks = "Transfer source PHPU".encode();
										let mut phpu_data = Vec::new();
										phpu_data.append(&mut phpu_contract_transfer_message_selector);
										phpu_data.append(&mut phpu_to);
										phpu_data.append(&mut phpu_value);
										phpu_data.append(&mut phpu_remarks);
										let _ = pallet_contracts::Pallet::<T>::bare_call(
											source.clone(), 			
											phpu_contract.clone(),			
											default_contract_value.clone(),
											default_contract_gas_limit.clone(),
											None,
											phpu_data,
											default_contract_debug,
										);
		
										// STEP 2: Mint lPHPU and transfer to source
										let mut lphpu_to = source.encode();
										let mut lphpu_value = (quantity * decimal_multiplier).encode();
										let mut lphpu_data = Vec::new();
										lphpu_data.append(&mut liquidity_contract_mint_to_message_selector);
										lphpu_data.append(&mut lphpu_to);
										lphpu_data.append(&mut lphpu_value);
										let _ = pallet_contracts::Pallet::<T>::bare_call(
											source.clone(), 			
											phpu_liquidity_contract,			
											default_contract_value,
											default_contract_gas_limit,
											None,
											lphpu_data,
											default_contract_debug,
										);
		
									}, None => return Err(Error::<T>::NoPhpuContract.into()),
								}

							}, None => return Err(Error::<T>::NoPhpuLiquidityContract.into()),
						}

					}, None => return Err(Error::<T>::NoPhpuLiquidityAccount.into()),
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

			// Decimal (14)
			let decimal: Option<BalanceOf<T>> = 100_000_000_000_000u64.try_into().ok();
			let decimal_multiplier;
			match decimal { Some(multiplier) => { decimal_multiplier = multiplier; },  None => { decimal_multiplier = Default::default(); } };

			// Liquidity Token Contract Settings
			let default_contract_gas_limit:u64 = 10_000_000_000;
			let default_contract_debug = false;

			let lumi_contract_value: LUMIBalanceOf<T> = Default::default();
			let lphpu_contract_value: LPHPUBalanceOf<T> = Default::default();
			let phpu_contract_value: PHPUBalanceOf<T> = Default::default();

			let mut liquidity_message_transfer_selector: Vec<u8> = [0xdb, 0x20, 0xf9, 0xf5].into();
			let mut liquidity_message_burn_selector: Vec<u8> = [0x7a, 0x9d, 0xa5, 0x10].into();
			let mut liquidity_message_total_supply_selector: Vec<u8> = [0x16, 0x2d, 0xf8, 0xc2].into();
			let mut phpu_contract_message_transfer_selector: Vec<u8> = [0xdb, 0x20, 0xf9, 0xf5].into();

			let dex_account;
			match DexAccountDataStore::<T>::get() {
				Some(some_dex_account) => dex_account = some_dex_account,
				None => return Err(Error::<T>::NoDexAccount.into()),
			}

			let umi_liquidity_account;
			match UmiLiquidityAccountDataStore::<T>::get() {
				Some(some_umi_liquidity_account) => umi_liquidity_account = some_umi_liquidity_account,
				None => return Err(Error::<T>::NoUmiLiquidityAccount.into()),
			}

			let phpu_liquidity_account;
			match UmiLiquidityAccountDataStore::<T>::get() {
				Some(some_phpu_liquidity_account) => phpu_liquidity_account = some_phpu_liquidity_account,
				None => return Err(Error::<T>::NoPhpuLiquidityAccount.into()),
			}

			let umi_liquidity_contract;
			match UmiLiquidityDataStore::<T>::get() {
				Some(some_umi_liquidity_contract) => umi_liquidity_contract = some_umi_liquidity_contract,
				None => return Err(Error::<T>::NoUmiLiquidityContract.into()),
			}

			let phpu_liquidity_contract;
			match PhpuLiquidityDataStore::<T>::get() {
				Some(some_umi_liquidity_contract) => phpu_liquidity_contract = some_umi_liquidity_contract,
				None => return Err(Error::<T>::NoPhpuLiquidityContract.into()),
			}

			let phpu_contract;
			match PhpuDataStore::<T>::get() {
				Some(some_umi_liquidity_contract) => phpu_contract = some_umi_liquidity_contract,
				None => return Err(Error::<T>::NoPhpuContract.into()),
			}

			let mut lumi_total_supply_data = Vec::new();
			lumi_total_supply_data.append(&mut liquidity_message_total_supply_selector);

			let response = pallet_contracts::Pallet::<T>::bare_call(
				source.clone(), 			
				umi_liquidity_contract.clone(),			
				lumi_contract_value,
				default_contract_gas_limit,
				None,
				lumi_total_supply_data,
				default_contract_debug,
			).result.unwrap();


			if ticker.eq("lUMI") {

				// STEP 1: Transfer the lUMI from source to the Liquidity Account
				let mut lumi_to = umi_liquidity_account.encode();
				let mut lumi_value = (quantity * decimal_multiplier.clone()).encode();
				let mut lumi_remarks = "Transfer lUMI".encode();

				let mut lumi_data = Vec::new();
				lumi_data.append(&mut liquidity_message_transfer_selector);
				lumi_data.append(&mut lumi_to);
				lumi_data.append(&mut lumi_value);
				lumi_data.append(&mut lumi_remarks);
				
				let _ = pallet_contracts::Pallet::<T>::bare_call(
					source.clone(), 			
					umi_liquidity_contract.clone(),			
					lumi_contract_value,
					default_contract_gas_limit,
					None,
					lumi_data,
					default_contract_debug,
				);

				// STEP 2: Return the equivalent UMI to the source
				let return_umi = quantity * decimal_multiplier.clone();
				<T as Config>::Currency::transfer(&umi_liquidity_account, &source, return_umi, ExistenceRequirement::KeepAlive)?;

				// STEP 3: Compute for the percentage - To be implemented

				// STEP 4: Transfer the UMI interest from DEX account to the source - To be implemented

				// STEP 5: Burn the lUMI in the liquidity Account
				let mut lumi_burn_to = umi_liquidity_account.encode();
				let mut lumi_burn_value = (quantity * decimal_multiplier.clone()).encode();
				
				let mut lumi_burn_data = Vec::new();
				lumi_burn_data.append(&mut liquidity_message_burn_selector);
				lumi_burn_data.append(&mut lumi_burn_to);
				lumi_burn_data.append(&mut lumi_burn_value);

				let _ = pallet_contracts::Pallet::<T>::bare_call(
					umi_liquidity_account.clone(), 			
					umi_liquidity_contract.clone(),			
					lumi_contract_value,
					default_contract_gas_limit,
					None,
					lumi_burn_data,
					default_contract_debug,
				);

			}

			if ticker.eq("lPHPU") {

				// STEP 1: Transfer the lPHPU from source to the Liquidity Account
				let mut lphpu_to = phpu_liquidity_account.encode();
				let mut lphpu_value = (quantity * decimal_multiplier.clone()).encode();
				let mut lphpu_remarks = "Transfer lPHPU".encode();

				let mut lphpu_data = Vec::new();
				lphpu_data.append(&mut liquidity_message_transfer_selector);
				lphpu_data.append(&mut lphpu_to);
				lphpu_data.append(&mut lphpu_value);
				lphpu_data.append(&mut lphpu_remarks);

				let _ = pallet_contracts::Pallet::<T>::bare_call(
					source.clone(), 			
					phpu_liquidity_contract.clone(),			
					lphpu_contract_value,
					default_contract_gas_limit,
					None,
					lphpu_data,
					default_contract_debug,
				);

				// STEP 2: Return the equivalent PHPU to the source
				let mut phpu_to = source.encode();
				let mut phpu_value = (quantity * decimal_multiplier.clone()).encode();
				let mut phpu_remarks = "Transfer PHPU to source".encode();

				let mut phpu_data = Vec::new();
				phpu_data.append(&mut phpu_contract_message_transfer_selector);
				phpu_data.append(&mut phpu_to);
				phpu_data.append(&mut phpu_value);
				phpu_data.append(&mut phpu_remarks);

				let _ = pallet_contracts::Pallet::<T>::bare_call(
					phpu_liquidity_account.clone(), 			
					phpu_contract.clone(),			
					phpu_contract_value,
					default_contract_gas_limit,
					None,
					phpu_data,
					default_contract_debug,
				);

				// STEP 3: Compute for the percentage - To be implemented

				// STEP 4: Transfer the PHP interest from DEX account to the source - To be implemented

				// STEP 5: Burn the lPHPU in the liquidity Account
				let mut lphpu_burn_to = phpu_liquidity_account.encode();
				let mut lphpu_burn_value = (quantity * decimal_multiplier.clone()).encode();

				let mut lphpu_burn_data = Vec::new();
				lphpu_burn_data.append(&mut liquidity_message_burn_selector);
				lphpu_burn_data.append(&mut lphpu_burn_to);
				lphpu_burn_data.append(&mut lphpu_burn_value);

				let _ = pallet_contracts::Pallet::<T>::bare_call(
					dex_account.clone(), 			
					phpu_liquidity_contract.clone(),			
					lphpu_contract_value,
					default_contract_gas_limit,
					None,
					lphpu_burn_data,
					default_contract_debug,
				);

			}

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
