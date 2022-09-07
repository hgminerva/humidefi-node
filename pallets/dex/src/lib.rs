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
	use sp_runtime::Percent;

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
	pub type SwapFeesDataStore<T: Config> = StorageValue<_, u8>;	

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
		SwapFeesDataStored(u8),
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
		pub fn store_swap_fees(origin: OriginFor<T>, swap_fees: u8) -> DispatchResult {
			ensure_root(origin)?;

			<SwapFeesDataStore<T>>::put(swap_fees.clone());

			Self::deposit_event(Event::SwapFeesDataStored(swap_fees));

			Ok(())
		}

		/// Swap token
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn do_swap(origin: OriginFor<T>, source: T::AccountId, quantity:  BalanceOf<T>, source_ticker: Vec<u8>, destination_ticker: Vec<u8>,) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// Ticker
			let ticker1 = scale_info::prelude::string::String::from_utf8(source_ticker).expect("Invalid");
			let ticker2 = scale_info::prelude::string::String::from_utf8(destination_ticker).expect("Invalid");

			// Decimal (14)
			let decimal: Option<BalanceOf<T>> = 100_000_000_000_000u64.try_into().ok();
			let decimal_multiplier;
			match decimal { Some(multiplier) => { decimal_multiplier = multiplier; },  None => { decimal_multiplier = Default::default(); } };

			// PHPU Contract Settings
			let phpu_contract_gas_limit = 10_000_000_000;
			let phpu_contract_debug = false;
			let phpu_contract_value: PHPUBalanceOf<T> = Default::default();
			let mut phpu_contract_message_transfer_selector: Vec<u8> = [0x84, 0xA1, 0x5D, 0xA1].into();

			// UMI -> PHPU
			if ticker1.eq("UMI") && ticker2.eq("PHPU") {
				match TickerDataStore::<T>::get() {
					Some(ticker_price_data) => {

						// Conversion
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
						
						match DexAccountDataStore::<T>::get() { 
							Some(dex_account) => {

								match PhpuDataStore::<T>::get() { 
									Some(phpu_contract) => {

										match SwapFeesDataStore::<T>::get() {
											Some(swap_fees) => {

													match UmiLiquidityAccountDataStore::<T>::get() { 
														Some(umi_liquidity_account) => {
															
															match PhpuLiquidityAccountDataStore::<T>::get() { 
																Some(phpu_liquidity_account) => {

																	// STEP 1: Transfer UMI from source to Liquidity UMI Account
																	let umi = quantity * decimal_multiplier.clone();
																	<T as Config>::Currency::transfer(&source, &umi_liquidity_account, umi, ExistenceRequirement::KeepAlive)?;

																	// STEP 2: Compute for the transaction fees
																	let swap_fee_percentage = Percent::from_percent(swap_fees);
																	let factor = (phpu_multiplier.clone()*decimal_multiplier.clone()) / umi_multiplier.clone();
																	let income_quantity = swap_fee_percentage * quantity;
																	let source_quantity = quantity - income_quantity;

																	// STEP 3: Transfer PHPU from Liquidity Account to source less transaction fees
																	let mut source_data = Vec::new();
																	let mut source_phpu =(source_quantity * factor.clone()).encode();
																	let mut source_to = source.encode();
																	source_data.append(&mut phpu_contract_message_transfer_selector);
																	source_data.append(&mut source_to);
																	source_data.append(&mut source_phpu);

																	let _ = pallet_contracts::Pallet::<T>::bare_call(
																		phpu_liquidity_account.clone(), 			
																		phpu_contract.clone(),			
																		phpu_contract_value,
																		phpu_contract_gas_limit,
																		None,
																		source_data,
																		phpu_contract_debug,
																	);

																	// STEP 4: Transfer PHPU transaction fees to DEX income 
																	let mut income_data = Vec::new();
																	let mut income_phpu =(income_quantity * factor.clone()).encode();
																	let mut income_to = dex_account.encode();
																	income_data.append(&mut phpu_contract_message_transfer_selector);
																	income_data.append(&mut income_to);
																	income_data.append(&mut income_phpu);

																	let _ = pallet_contracts::Pallet::<T>::bare_call(
																		phpu_liquidity_account.clone(), 			
																		phpu_contract.clone(),			
																		phpu_contract_value,
																		phpu_contract_gas_limit,
																		None,
																		income_data,
																		phpu_contract_debug,
																	);

																}, None => return Err(Error::<T>::NoPhpuLiquidityAccount.into()),
															}

														}, None => return Err(Error::<T>::NoPhpuLiquidityAccount.into()),
													}

											}, None => return Err(Error::<T>::NoSwapFees.into()),
										}

									}, None => return Err(Error::<T>::NoPhpuContract.into()),
								}

							} , None => return Err(Error::<T>::NoDexAccount.into()),
						} 

					}, None => return Err(Error::<T>::NoTickerPriceData.into()),
				}	

			}

			// PHPU -> UMI
			if ticker1.eq("PHPU") && ticker2.eq("UMI") {
				match TickerDataStore::<T>::get() {
					Some(ticker_price_data) => {

						// Conversion
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

						match DexAccountDataStore::<T>::get() {
							Some(dex_account) => { // DEX Account

								match PhpuDataStore::<T>::get() {
									Some(phpu_contract) => { 

										match SwapFeesDataStore::<T>::get() { 
											Some(swap_fees) => {

												match UmiLiquidityAccountDataStore::<T>::get() { 
													Some(umi_liquidity_account) => {
														
														match PhpuLiquidityAccountDataStore::<T>::get() { 
															Some(phpu_liquidity_account) => {

																// STEP 1: Transfer PHPU from source to PHPU Liquidity Account
																let mut to = phpu_liquidity_account.encode();
																let mut phpu = (quantity * decimal_multiplier.clone()).encode();
																let mut data = Vec::new();
																data.append(&mut phpu_contract_message_transfer_selector);
																data.append(&mut to);
																data.append(&mut phpu);
																let _ = pallet_contracts::Pallet::<T>::bare_call(
																	source.clone(), 			
																	phpu_contract,			
																	phpu_contract_value,
																	phpu_contract_gas_limit,
																	None,
																	data,
																	phpu_contract_debug,
																);

																// STEP 2: Transaction Fees
																let swap_fee_percentage = Percent::from_percent(swap_fees);
																let factor = (umi_multiplier.clone()*decimal_multiplier.clone()) / phpu_multiplier.clone();
																let income_quantity = swap_fee_percentage * quantity;
																let source_quantity = quantity - income_quantity;

																// STEP 3: Transfer UMI less transaction fee to source from PHPU Liquidity Account
																let source_umi = source_quantity * factor.clone();
																<T as Config>::Currency::transfer(&umi_liquidity_account, &source, source_umi, ExistenceRequirement::KeepAlive)?;

																// STEP 4: Transfer transaction fees to DEX Income account
																let income_umi = income_quantity * factor.clone();
																<T as Config>::Currency::transfer(&umi_liquidity_account, &dex_account, income_umi, ExistenceRequirement::KeepAlive)?;

															}, None => return Err(Error::<T>::NoPhpuLiquidityAccount.into()),
														}

													}, None => return Err(Error::<T>::NoUmiLiquidityAccount.into()),
												}

											}, None => return Err(Error::<T>::NoSwapFees.into()),
										}

									}, None => return Err(Error::<T>::NoPhpuContract.into()),
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

			// Decimal (14)
			let decimal: Option<BalanceOf<T>> = 100_000_000_000_000u64.try_into().ok();
			let decimal_multiplier;
			match decimal { Some(multiplier) => { decimal_multiplier = multiplier; },  None => { decimal_multiplier = Default::default(); } };
			
			// Contract settings
			let default_contract_gas_limit:u64 = 10_000_000_000;
			let default_contract_debug = false;
			let default_contract_value: PHPUBalanceOf<T> = Default::default();
			let mut liquidity_contract_mint_message_selector: Vec<u8> = [0x1D, 0x2F, 0x13, 0xC5].into();
			let mut phpu_contract_transfer_message_selector: Vec<u8> = [0x84, 0xA1, 0x5D, 0xA1].into();

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
								lumi_data.append(&mut liquidity_contract_mint_message_selector);
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
										let mut phpu_data = Vec::new();
										phpu_data.append(&mut phpu_contract_transfer_message_selector);
										phpu_data.append(&mut phpu_to);
										phpu_data.append(&mut phpu_value);
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
										lphpu_data.append(&mut liquidity_contract_mint_message_selector);
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

			let mut liquidity_message_transfer_selector: Vec<u8> = [0xDB, 0x20, 0xF9, 0xF5].into();
			let mut liquidity_message_burn_selector: Vec<u8> = [0x7A, 0x9D, 0xA5, 0x10].into();
			
			let mut phpu_contract_message_transfer_selector: Vec<u8> = [0x84, 0xA1, 0x5D, 0xA1].into();

			if ticker.eq("lUMI") {
				match UmiLiquidityDataStore::<T>::get() {
					Some(umi_liquidity_contract) => {

						match UmiLiquidityAccountDataStore::<T>::get() {
							Some(umi_liquidity_account) => {

								match DexAccountDataStore::<T>::get() {
									Some(dex_account) => { 
		
										// STEP 1: Transfer the lUMI from source to the Liquidity Account
										let mut lumi_to = umi_liquidity_account.encode();
										let mut lumi_value = (quantity * decimal_multiplier.clone()).encode();
										let mut lumi_data = Vec::new();
										lumi_data.append(&mut liquidity_message_transfer_selector);
										lumi_data.append(&mut lumi_to);
										lumi_data.append(&mut lumi_value);
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
											dex_account.clone(), 			
											umi_liquidity_contract.clone(),			
											lumi_contract_value,
											default_contract_gas_limit,
											None,
											lumi_burn_data,
											default_contract_debug,
										);
		
									}, None => return Err(Error::<T>::NoDexAccount.into()),
								}

							},None => return Err(Error::<T>::NoUmiLiquidityAccount.into()),
						}

					},None => return Err(Error::<T>::NoUmiLiquidityContract.into()),
				}

			}

			if ticker.eq("lPHPU") {
				match PhpuLiquidityDataStore::<T>::get() {
					Some(phpu_liquidity_contract) => {

						match PhpuLiquidityAccountDataStore::<T>::get() {
							Some(phpu_liquidity_account) => {

								match DexAccountDataStore::<T>::get() {
									Some(dex_account) => { 
		
										match PhpuDataStore::<T>::get() {
											Some(phpu_contract) => { 
										
												// STEP 1: Transfer the lPHPU from source to the Liquidity Account
												let mut lphpu_to = phpu_liquidity_account.encode();
												let mut lphpu_value = (quantity * decimal_multiplier.clone()).encode();
												let mut lphpu_data = Vec::new();
												lphpu_data.append(&mut liquidity_message_transfer_selector);
												lphpu_data.append(&mut lphpu_to);
												lphpu_data.append(&mut lphpu_value);
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
												let mut phpu_data = Vec::new();
												phpu_data.append(&mut phpu_contract_message_transfer_selector);
												phpu_data.append(&mut phpu_to);
												phpu_data.append(&mut phpu_value);
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
										
											}, None => return Err(Error::<T>::NoPhpuContract.into()),
										}
		
									}, None => return Err(Error::<T>::NoDexAccount.into()),
								}

							},None => return Err(Error::<T>::NoUmiLiquidityAccount.into()),
						}

					},None => return Err(Error::<T>::NoUmiLiquidityContract.into()),
				}

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
