use std::collections::BTreeMap;

type AccountId = String;
type Balance = u128;
/// This is the Balances Module.
/// It is a simple module which keeps track of how much balance each account has in this state
/// machine.
#[derive(Debug)]
pub struct Pallet {
	// A simple storage mapping from accounts (`String`) to their balances (`u128`).
	// balances: BTreeMap<String, u128>,
	balances: BTreeMap<AccountId, Balance>,
}

impl Pallet {
	/// Create a new instance of the balances module.
	pub fn new() -> Self {
		Self { balances: BTreeMap::new() }
	}

	/// Set the balance of an account `who` to some `amount`.
	// pub fn set_balance(&mut self, who: &String, amount: u128) {
	pub fn set_balance(&mut self, who: &AccountId, amount: Balance) {
		/* Insert `amount` into the BTreeMap under `who`. */
		self.balances.insert(who.clone(), amount);
	}

	/// Get the balance of an account `who`.
	/// If the account has no stored balance, we return zero.
	// pub fn balance(&self, who: &String) -> u128 {
	pub fn balance(&self, who: &AccountId) -> Balance {
		/* Return the balance of `who`, returning zero if `None`. */
		*self.balances.get(who).unwrap_or(&0)
	}
	pub fn transfer(
		&mut self,
		// caller: String,
		// to: String,
		// amount: u128,
		caller: AccountId,
		to: AccountId,
		amount: Balance,
	) -> Result<(), &'static str> {
		/* TODO:
			- Get the balance of account `caller`.
			- Get the balance of account `to`.

			- Use safe math to calculate a `new_caller_balance`.
			- Use safe math to calculate a `new_to_balance`.

			- Insert the new balance of `caller`.
			- Insert the new balance of `to`.
		*/
		let caller_balance = self.balance(&caller);
		let to_balance = self.balance(&to);

		let new_caller_balance = caller_balance.checked_sub(amount).ok_or("Not enough funds.")?;
		let new_to_balance = to_balance.checked_add(amount).ok_or("Overflow")?;

		self.balances.insert(caller, new_caller_balance);
		self.balances.insert(to, new_to_balance);

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn init_balances() {
		/* TODO: Create a mutable variable `balances`, which is a new instance of `Pallet`. */
		let mut balances = super::Pallet::new();
		/* TODO: Assert that the balance of `alice` starts at zero. */
		assert_eq!(balances.balance(&"alice".to_string()), 0);
		/* TODO: Set the balance of `alice` to 100. */
		balances.set_balance(&"alice".to_string(), 100);
		/* TODO: Assert the balance of `alice` is now 100. */
		assert_eq!(balances.balance(&"alice".to_string()), 100);
		/* TODO: Assert the balance of `bob` has not changed and is 0. */
		assert_eq!(balances.balance(&"bob".to_string()), 0);
	}
	#[test]
	fn transfer_balance() {
		/* TODO: Create a test that checks the following:
			- That `alice` cannot transfer funds she does not have.
			- That `alice` can successfully transfer funds to `bob`.
			- That the balance of `alice` and `bob` is correctly updated.
		*/
		let mut balances = super::Pallet::new();

		assert_eq!(
			balances.transfer("alice".to_string(), "bob".to_string(), 51),
			Err("Not enough funds.")
		);

		balances.set_balance(&"alice".to_string(), 100);
		assert_eq!(balances.transfer("alice".to_string(), "bob".to_string(), 51), Ok(()));
		assert_eq!(balances.balance(&"alice".to_string()), 49);
		assert_eq!(balances.balance(&"bob".to_string()), 51);

		assert_eq!(
			balances.transfer("alice".to_string(), "bob".to_string(), 51),
			Err("Not enough funds.")
		);
	}
}
