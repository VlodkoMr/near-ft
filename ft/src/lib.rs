/*!
Fungible Token implementation with JSON serialization.
NOTES:
  - The maximum balance value is limited by U128 (2**128 - 1).
  - JSON calls should pass U128 as a base-10 string. E.g. "100".
  - The contract optimizes the inner trie structure by hashing account IDs. It will prevent some
    abuse of deep tries. Shouldn't be an issue, once NEAR clients implement full hashing of keys.
  - The contract tracks the change in storage before and after the call. If the storage increases,
    the contract requires the caller of the contract to attach enough deposit to the function call
    to cover the storage cost.
    This is done to prevent a denial of service attack on the contract by taking all available storage.
    If the storage decreases, the contract will issue a refund for the cost of the released storage.
    The unused tokens from the attached deposit are also refunded, so it's safe to
    attach more deposit than required.
  - To prevent the deployed contract from being modified or deleted, it should not have any access
    keys on its account.
 */
use near_contract_standards::fungible_token::FungibleToken;
use near_contract_standards::fungible_token::metadata::{
    FT_METADATA_SPEC, FungibleTokenMetadata, FungibleTokenMetadataProvider,
};
use near_sdk::{AccountId, Balance, env, log, near_bindgen, PanicOnDefault, Promise, PromiseOrValue};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LazyOption;
use near_sdk::json_types::{U128, ValidAccountId};

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    token: FungibleToken,
    metadata: LazyOption<FungibleTokenMetadata>,
}

const DATA_IMAGE_SVG_NEAR_ICON: &str = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAMgAAADICAYAAACtWK6eAAAAGXRFWHRTb2Z0d2FyZQBBZG9iZSBJbWFnZVJlYWR5ccllPAAAAyVpVFh0WE1MOmNvbS5hZG9iZS54bXAAAAAAADw/eHBhY2tldCBiZWdpbj0i77u/IiBpZD0iVzVNME1wQ2VoaUh6cmVTek5UY3prYzlkIj8+IDx4OnhtcG1ldGEgeG1sbnM6eD0iYWRvYmU6bnM6bWV0YS8iIHg6eG1wdGs9IkFkb2JlIFhNUCBDb3JlIDUuNi1jMTQ4IDc5LjE2NDAzNiwgMjAxOS8wOC8xMy0wMTowNjo1NyAgICAgICAgIj4gPHJkZjpSREYgeG1sbnM6cmRmPSJodHRwOi8vd3d3LnczLm9yZy8xOTk5LzAyLzIyLXJkZi1zeW50YXgtbnMjIj4gPHJkZjpEZXNjcmlwdGlvbiByZGY6YWJvdXQ9IiIgeG1sbnM6eG1wPSJodHRwOi8vbnMuYWRvYmUuY29tL3hhcC8xLjAvIiB4bWxuczp4bXBNTT0iaHR0cDovL25zLmFkb2JlLmNvbS94YXAvMS4wL21tLyIgeG1sbnM6c3RSZWY9Imh0dHA6Ly9ucy5hZG9iZS5jb20veGFwLzEuMC9zVHlwZS9SZXNvdXJjZVJlZiMiIHhtcDpDcmVhdG9yVG9vbD0iQWRvYmUgUGhvdG9zaG9wIDIxLjAgKE1hY2ludG9zaCkiIHhtcE1NOkluc3RhbmNlSUQ9InhtcC5paWQ6ODIxMjgwRjk0NTI1MTFFQzlDQkM5RTNGREFGMzFFQkIiIHhtcE1NOkRvY3VtZW50SUQ9InhtcC5kaWQ6ODIxMjgwRkE0NTI1MTFFQzlDQkM5RTNGREFGMzFFQkIiPiA8eG1wTU06RGVyaXZlZEZyb20gc3RSZWY6aW5zdGFuY2VJRD0ieG1wLmlpZDo4MjEyODBGNzQ1MjUxMUVDOUNCQzlFM0ZEQUYzMUVCQiIgc3RSZWY6ZG9jdW1lbnRJRD0ieG1wLmRpZDo4MjEyODBGODQ1MjUxMUVDOUNCQzlFM0ZEQUYzMUVCQiIvPiA8L3JkZjpEZXNjcmlwdGlvbj4gPC9yZGY6UkRGPiA8L3g6eG1wbWV0YT4gPD94cGFja2V0IGVuZD0iciI/PrCWZg4AABhgSURBVHja7F0J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";

#[near_bindgen]
impl Contract {
    /// Initializes the contract with the given total supply owned by the given `owner_id` with
    /// default metadata (for example purposes only).
    #[init]
    pub fn new_default_meta(owner_id: ValidAccountId, total_supply: U128) -> Self {
        Self::new(
            owner_id,
            total_supply,
            FungibleTokenMetadata {
                spec: FT_METADATA_SPEC.to_string(),
                name: "BlaBla Token".to_string(),
                symbol: "BLABLA".to_string(),
                icon: Some(DATA_IMAGE_SVG_NEAR_ICON.to_string()),
                reference: None,
                reference_hash: None,
                decimals: 24,
            },
        )
    }

    /// Initializes the contract with the given total supply owned by the given `owner_id` with
    /// the given fungible token metadata.
    #[init]
    pub fn new(
        owner_id: ValidAccountId,
        total_supply: U128,
        metadata: FungibleTokenMetadata,
    ) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        metadata.assert_valid();
        let mut this = Self {
            token: FungibleToken::new(b"a".to_vec()),
            metadata: LazyOption::new(b"m".to_vec(), Some(&metadata)),
        };
        this.token.internal_register_account(owner_id.as_ref());
        this.token.internal_deposit(owner_id.as_ref(), total_supply.into());
        this
    }

    fn on_account_closed(&mut self, account_id: AccountId, balance: Balance) {
        log!("Closed @{} with {}", account_id, balance);
    }

    fn on_tokens_burned(&mut self, account_id: AccountId, amount: Balance) {
        log!("Account @{} burned {}", account_id, amount);
    }

    #[payable]
    pub fn ft_mint(
        &mut self,
        receiver_id: AccountId,
        amount: U128,
    ) {
        //get initial storage usage
        assert!(
            amount.0 <= 1000,
            "Cannot mint more than 1000 tokens"
        );

        let initial_storage_usage = env::storage_usage();

        let mut amount_for_account = self.token.accounts.get(&receiver_id).unwrap_or(0);
        amount_for_account += amount.0;

        self.token.accounts.insert(&receiver_id, &amount_for_account);
        self.token.total_supply = self
            .token
            .total_supply
            .checked_add(amount.0)
            .unwrap_or_else(|| env::panic(b"Total supply overflow"));

        //refund any excess storage
        let storage_used = env::storage_usage() - initial_storage_usage;
        let required_cost = env::storage_byte_cost() * Balance::from(storage_used);
        let attached_deposit = env::attached_deposit();

        assert!(
            required_cost <= attached_deposit,
            "Must attach {} yoctoNEAR to cover storage", required_cost
        );

        let refund = attached_deposit - required_cost;
        if refund > 1 {
            Promise::new(env::predecessor_account_id()).transfer(refund);
        }
    }
}

near_contract_standards::impl_fungible_token_core!(Contract, token, on_tokens_burned);
near_contract_standards::impl_fungible_token_storage!(Contract, token, on_account_closed);

#[near_bindgen]
impl FungibleTokenMetadataProvider for Contract {
    fn ft_metadata(&self) -> FungibleTokenMetadata {
        self.metadata.get().unwrap()
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use near_sdk::{Balance, testing_env};
    use near_sdk::MockedBlockchain;
    use near_sdk::test_utils::{accounts, VMContextBuilder};

    use super::*;

    const TOTAL_SUPPLY: Balance = 1_000_000_000_000_000;

    fn get_context(predecessor_account_id: ValidAccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    #[test]
    fn test_new() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let contract = Contract::new_default_meta(accounts(1).into(), TOTAL_SUPPLY.into());
        testing_env!(context.is_view(true).build());
        assert_eq!(contract.ft_total_supply().0, TOTAL_SUPPLY);
        assert_eq!(contract.ft_balance_of(accounts(1)).0, TOTAL_SUPPLY);
    }

    #[test]
    #[should_panic(expected = "The contract is not initialized")]
    fn test_default() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let _contract = Contract::default();
    }

    #[test]
    fn test_transfer() {
        let mut context = get_context(accounts(2));
        testing_env!(context.build());
        let mut contract = Contract::new_default_meta(accounts(2).into(), TOTAL_SUPPLY.into());
        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(contract.storage_balance_bounds().min.into())
            .predecessor_account_id(accounts(1))
            .build());
        // Paying for account registration, aka storage deposit
        contract.storage_deposit(None, None);

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(1)
            .predecessor_account_id(accounts(2))
            .build());
        let transfer_amount = TOTAL_SUPPLY / 3;
        contract.ft_transfer(accounts(1), transfer_amount.into(), None);

        testing_env!(context
            .storage_usage(env::storage_usage())
            .account_balance(env::account_balance())
            .is_view(true)
            .attached_deposit(0)
            .build());
        assert_eq!(contract.ft_balance_of(accounts(2)).0, (TOTAL_SUPPLY - transfer_amount));
        assert_eq!(contract.ft_balance_of(accounts(1)).0, transfer_amount);
    }
}
