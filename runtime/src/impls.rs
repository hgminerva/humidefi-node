// Humidefi block authorship fees, impls.rs included
// By: HGMinerva - June 20, 2022
// Reference
// 1. Github: https://github.com/substrate-developer-hub/substrate-node-template/issues/51
// 2. Stackexchange: https://substrate.stackexchange.com/questions/3298/how-could-i-configure-that-reward-amount-or-value-for-my-aura-validators/3346#3346
// -------------------------------
use crate::{Authorship, Balances};
use frame_support::traits::{Imbalance, OnUnbalanced};
use crate::sp_api_hidden_includes_construct_runtime::hidden_include::traits::Currency;
use crate::AccountId;

type NegativeImbalance = <Balances as Currency<AccountId>>::NegativeImbalance;

pub struct Author;
impl OnUnbalanced<NegativeImbalance> for Author {
	fn on_nonzero_unbalanced(amount: NegativeImbalance) {
		if let Some(author) = Authorship::author() {
			Balances::resolve_creating(&author, amount);
		}
	}
}

pub struct DealWithFees;
impl OnUnbalanced<NegativeImbalance> for DealWithFees {
	fn on_unbalanceds<B>(mut fees_then_tips: impl Iterator<Item = NegativeImbalance>) {
		if let Some(fees) = fees_then_tips.next() {
			let mut split = fees.ration(0, 100);
			frame_support::debug(&split);
			if let Some(tips) = fees_then_tips.next() {
				// for tips, if any, 80% to treasury, 20% to block author (though this can be anything)
				frame_support::debug(&tips);
				tips.ration_merge_into(0, 100, &mut split);
			}
			//Treasury::on_unbalanced(split.0);
			Author::on_unbalanced(split.1);
		}
	}
} 
// =========================