#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod erc721 {
    use ink::storage::Mapping;
    use scale::{ Decode, Encode };

    pub type TokenId = u32;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    #[derive(Default)]
    pub struct Erc721 {
        //Mapping from token Id to owner address
        _owners: Mapping<TokenId, AccountId>,

        //Mapping owner address to balance
        _balances: Mapping<AccountId, Balance>,

        //Mapping from token Id to approved address
        _token_approvals: Mapping<TokenId, AccountId>,

        //Mapping from owner to operator approvals,
        _operator_approvals: Mapping<(AccountId, AccountId), ()>,
    }

    #[derive(Encode, Decode, Debug, PartialEq, Eq, Copy, Clone)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        NotOwner,
        NotApproved,
        TokenExists,
        TokenNotFound,
        CannotInsert,
        CannotFetchValue,
        NotAllowed,
    }

    #[ink(event)]
    pub struct ApprovalForAll {
        #[ink(topic)]
        owner: AccountId,

        #[ink(topic)]
        operator: AccountId,

        approved: bool,
    }

    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: Option<AccountId>,

        #[ink(topic)]
        to: AccountId,

        #[ink(topic)]
        token_id: TokenId,
    }

    /// Event emitted when a token transfer occurs.
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        #[ink(topic)]
        token_id: TokenId,
    }

    impl Erc721 {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new() -> Self {
            Default::default()
        }

        // Return the balance of the owner
        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> Balance {
            self._balance_of(&owner)
        }

        fn _balance_of(&self, owner: &AccountId) -> Balance {
            self._balances.get(owner).unwrap_or(0)
        }

        // Return the owner of the token
        #[ink(message)]
        pub fn owner_of(&self, token_id: TokenId) -> Option<AccountId> {
            self._owners.get(token_id)
        }

        // Return the approved account Id for the token if any
        #[ink(message)]
        pub fn get_approved(&self, token_id: TokenId) -> Option<AccountId> {
            self._token_approvals.get(token_id)
        }

        // Check if operator is approved by the owner
        #[ink(message)]
        pub fn is_approved_for_all(&self, owner: AccountId, operator: AccountId) -> bool {
            self._is_approved_for_all(owner, operator)
        }

        fn _is_approved_for_all(&self, owner: AccountId, operator: AccountId) -> bool {
            self._operator_approvals.contains((&owner, &operator))
        }

        // Approve or disapprove the operator for all token of the caller
        #[ink(message)]
        pub fn set_approval_for_all(&mut self, to: AccountId, approved: bool) -> Result<(), Error> {
            self._set_approval_for_all(to, approved)?;
            Ok(())
        }

        fn _set_approval_for_all(&mut self, to: AccountId, approved: bool) -> Result<(), Error> {
            let caller = self.env().caller();

            if to == caller {
                return Err(Error::NotAllowed);
            }

            self.env().emit_event(ApprovalForAll {
                owner: caller,
                operator: to,
                approved,
            });

            if approved {
                self._operator_approvals.insert((&caller, &to), &());
            } else {
                self._operator_approvals.remove((&caller, &to));
            }

            Ok(())
        }

        #[ink(message)]
        pub fn approve(&mut self, to: AccountId, token_id: TokenId) -> Result<(), Error> {
            self._approve_for(&to, token_id)?;
            Ok(())
        }

        fn _approve_for(&mut self, to: &AccountId, token_id: TokenId) -> Result<(), Error> {
            let caller = self.env().caller();
            let owner = self.owner_of(token_id);

            if
                !(
                    owner == Some(caller) ||
                    self._is_approved_for_all(owner.expect("Error with AccountId"), caller)
                )
            {
                return Err(Error::NotAllowed);
            }

            if *to == AccountId::from([0x0; 32]) {
                return Err(Error::NotAllowed);
            }

            if self._token_approvals.contains(token_id) {
                return Err(Error::CannotInsert);
            } else {
                self._token_approvals.insert(token_id, to);
            }

            self.env().emit_event(Approval {
                owner: owner,
                to: *to,
                token_id,
            });

            Ok(())
        }

        #[ink(message)]
        pub fn transfer(&mut self, destination: AccountId, token_id: TokenId) -> Result<(), Error> {
            let caller = self.env().caller();

            self._transfer_from(&caller, &destination, token_id)?;
            Ok(())
        }

        #[ink(message)]
        pub fn transfer_from(
            &mut self,
            from: AccountId,
            destination: AccountId,
            token_id: TokenId
        ) -> Result<(), Error> {
            self._transfer_from(&from, &destination, token_id)?;
            Ok(())
        }

        fn _transfer_from(
            &mut self,
            from: &AccountId,
            to: &AccountId,
            token_id: TokenId
        ) -> Result<(), Error> {
            let caller = self.env().caller();

            if !self._exists(token_id) {
                return Err(Error::TokenNotFound);
            }

            if !self._is_approved_or_owner(Some(caller), token_id) {
                return Err(Error::NotApproved);
            }

            self._clear_approval(token_id);
            self._remove_token_from(from, token_id)?;
            self._add_token_to(to, token_id)?;

            self.env().emit_event(Transfer {
                from: Some(*from),
                to: Some(*to),
                token_id,
            });

            Ok(())
        }

        fn _is_approved_or_owner(&self, spender: Option<AccountId>, token_id: TokenId) -> bool {
            let owner = self.owner_of(token_id);

            spender != Some(AccountId::from([0x0; 32])) &&
                (spender == owner ||
                    spender == self._token_approvals.get(token_id) ||
                    self._is_approved_for_all(
                        owner.expect("Error with AccountId"),
                        spender.expect("Error with AccountId")
                    ))
        }

        fn _exists(&self, token_id: TokenId) -> bool {
            self._owners.contains(token_id)
        }

        /// Removes existing approval from token `id`.
        fn _clear_approval(&mut self, token_id: TokenId) {
            self._token_approvals.remove(token_id);
        }

        /// Removes token `id` from the owner.
        fn _remove_token_from(&mut self, from: &AccountId, token_id: TokenId) -> Result<(), Error> {
            let Self { _owners, _balances, .. } = self;

            if !_owners.contains(token_id) {
                return Err(Error::TokenNotFound);
            }

            let count = _balances
                .get(from)
                .map(|c| c - 1)
                .ok_or(Error::CannotFetchValue)?;

            _balances.insert(from, &count);
            _owners.remove(token_id);

            Ok(())
        }

        /// Adds the token `id` to the `to` AccountID.
        fn _add_token_to(&mut self, to: &AccountId, token_id: TokenId) -> Result<(), Error> {
            let Self { _owners, _balances, .. } = self;

            if _owners.contains(token_id) {
                return Err(Error::TokenExists);
            }

            if *to == AccountId::from([0x0; 32]) {
                return Err(Error::NotAllowed);
            }

            let count = _balances
                .get(to)
                .map(|c| c + 1)
                .unwrap_or(1);

            _balances.insert(to, &count);
            _owners.insert(token_id, to);

            Ok(())
        }

        // Creates a new token.
        #[ink(message)]
        pub fn mint(&mut self, token_id: TokenId) -> Result<(), Error> {
            let caller = self.env().caller();

            self._add_token_to(&caller, token_id)?;

            self.env().emit_event(Transfer {
                from: Some(AccountId::from([0x0; 32])),
                to: Some(caller),
                token_id,
            });

            Ok(())
        }

        /// Deletes an existing token. Only the owner can burn the token.
        #[ink(message)]
        pub fn burn(&mut self, token_id: TokenId) -> Result<(), Error> {
            let caller = self.env().caller();

            let Self { _owners, _balances, .. } = self;

            let owner = _owners.get(token_id).ok_or(Error::TokenNotFound)?;

            if owner != caller {
                return Err(Error::NotOwner);
            }

            let count = _balances
                .get(caller)
                .map(|c| c - 1)
                .ok_or(Error::CannotFetchValue)?;

            _balances.insert(caller, &count);
            _owners.remove(token_id);

            self.env().emit_event(Transfer {
                from: Some(caller),
                to: Some(AccountId::from([0x0; 32])),
                token_id,
            });

            Ok(())
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        #[ink::test]
        fn mint_works() {
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            // Create a new contract instance.
            let mut erc721 = Erc721::new();
            // Token 1 does not exists.
            assert_eq!(erc721.owner_of(1), None);
            // Alice does not owns tokens.
            assert_eq!(erc721.balance_of(accounts.alice), 0);
            // Create token Id 1.
            assert_eq!(erc721.mint(1), Ok(()));
            // Alice owns 1 token.
            assert_eq!(erc721.balance_of(accounts.alice), 1);
        }

        #[ink::test]
        fn mint_existing_should_fail() {
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            // Create a new contract instance.
            let mut erc721 = Erc721::new();
            // Create token Id 1.
            assert_eq!(erc721.mint(1), Ok(()));
            // The first Transfer event takes place
            assert_eq!(1, ink::env::test::recorded_events().count());
            // Alice owns 1 token.
            assert_eq!(erc721.balance_of(accounts.alice), 1);
            // Alice owns token Id 1.
            assert_eq!(erc721.owner_of(1), Some(accounts.alice));
            // Cannot create  token Id if it exists.
            // Bob cannot own token Id 1.
            assert_eq!(erc721.mint(1), Err(Error::TokenExists));
        }

        #[ink::test]
        fn transfer_works() {
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            // Create a new contract instance.
            let mut erc721 = Erc721::new();
            // Create token Id 1 for Alice
            assert_eq!(erc721.mint(1), Ok(()));
            // Alice owns token 1
            assert_eq!(erc721.balance_of(accounts.alice), 1);
            // Bob does not owns any token
            assert_eq!(erc721.balance_of(accounts.bob), 0);
            // The first Transfer event takes place
            assert_eq!(1, ink::env::test::recorded_events().count());
            // Alice transfers token 1 to Bob
            assert_eq!(erc721.transfer(accounts.bob, 1), Ok(()));
            // The second Transfer event takes place
            assert_eq!(2, ink::env::test::recorded_events().count());
            // Bob owns token 1
            assert_eq!(erc721.balance_of(accounts.bob), 1);
        }

        #[ink::test]
        fn invalid_transfer_should_fail() {
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            // Create a new contract instance.
            let mut erc721 = Erc721::new();
            // Transfer token fails if it does not exists.
            assert_eq!(erc721.transfer(accounts.bob, 2), Err(Error::TokenNotFound));
            // Token Id 2 does not exists.
            assert_eq!(erc721.owner_of(2), None);
            // Create token Id 2.
            assert_eq!(erc721.mint(2), Ok(()));
            // Alice owns 1 token.
            assert_eq!(erc721.balance_of(accounts.alice), 1);
            // Token Id 2 is owned by Alice.
            assert_eq!(erc721.owner_of(2), Some(accounts.alice));
            // Set Bob as caller
            set_caller(accounts.bob);
            // Bob cannot transfer not owned tokens.
            assert_eq!(erc721.transfer(accounts.eve, 2), Err(Error::NotApproved));
        }

        #[ink::test]
        fn approved_transfer_works() {
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            // Create a new contract instance.
            let mut erc721 = Erc721::new();
            // Create token Id 1.
            assert_eq!(erc721.mint(1), Ok(()));
            // Token Id 1 is owned by Alice.
            assert_eq!(erc721.owner_of(1), Some(accounts.alice));
            // Approve token Id 1 transfer for Bob on behalf of Alice.
            assert_eq!(erc721.approve(accounts.bob, 1), Ok(()));
            // Set Bob as caller
            set_caller(accounts.bob);
            // Bob transfers token Id 1 from Alice to Eve.
            assert_eq!(erc721.transfer_from(accounts.alice, accounts.eve, 1), Ok(()));
            // TokenId 3 is owned by Eve.
            assert_eq!(erc721.owner_of(1), Some(accounts.eve));
            // Alice does not owns tokens.
            assert_eq!(erc721.balance_of(accounts.alice), 0);
            // Bob does not owns tokens.
            assert_eq!(erc721.balance_of(accounts.bob), 0);
            // Eve owns 1 token.
            assert_eq!(erc721.balance_of(accounts.eve), 1);
        }

        #[ink::test]
        fn approved_for_all_works() {
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            // Create a new contract instance.
            let mut erc721 = Erc721::new();
            // Create token Id 1.
            assert_eq!(erc721.mint(1), Ok(()));
            // Create token Id 2.
            assert_eq!(erc721.mint(2), Ok(()));
            // Alice owns 2 tokens.
            assert_eq!(erc721.balance_of(accounts.alice), 2);
            // Approve token Id 1 transfer for Bob on behalf of Alice.
            assert_eq!(erc721.set_approval_for_all(accounts.bob, true), Ok(()));
            // Bob is an approved operator for Alice
            assert!(erc721.is_approved_for_all(accounts.alice, accounts.bob));
            // Set Bob as caller
            set_caller(accounts.bob);
            // Bob transfers token Id 1 from Alice to Eve.
            assert_eq!(erc721.transfer_from(accounts.alice, accounts.eve, 1), Ok(()));
            // TokenId 1 is owned by Eve.
            assert_eq!(erc721.owner_of(1), Some(accounts.eve));
            // Alice owns 1 token.
            assert_eq!(erc721.balance_of(accounts.alice), 1);
            // Bob transfers token Id 2 from Alice to Eve.
            assert_eq!(erc721.transfer_from(accounts.alice, accounts.eve, 2), Ok(()));
            // Bob does not own tokens.
            assert_eq!(erc721.balance_of(accounts.bob), 0);
            // Eve owns 2 tokens.
            assert_eq!(erc721.balance_of(accounts.eve), 2);
            // Remove operator approval for Bob on behalf of Alice.
            set_caller(accounts.alice);
            assert_eq!(erc721.set_approval_for_all(accounts.bob, false), Ok(()));
            // Bob is not an approved operator for Alice.
            assert!(!erc721.is_approved_for_all(accounts.alice, accounts.bob));
        }

        #[ink::test]
        fn not_approved_transfer_should_fail() {
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            // Create a new contract instance.
            let mut erc721 = Erc721::new();
            // Create token Id 1.
            assert_eq!(erc721.mint(1), Ok(()));
            // Alice owns 1 token.
            assert_eq!(erc721.balance_of(accounts.alice), 1);
            // Bob does not owns tokens.
            assert_eq!(erc721.balance_of(accounts.bob), 0);
            // Eve does not owns tokens.
            assert_eq!(erc721.balance_of(accounts.eve), 0);
            // Set Eve as caller
            set_caller(accounts.eve);
            // Eve is not an approved operator by Alice.
            assert_eq!(
                erc721.transfer_from(accounts.alice, accounts.frank, 1),
                Err(Error::NotApproved)
            );
            // Alice owns 1 token.
            assert_eq!(erc721.balance_of(accounts.alice), 1);
            // Bob does not owns tokens.
            assert_eq!(erc721.balance_of(accounts.bob), 0);
            // Eve does not owns tokens.
            assert_eq!(erc721.balance_of(accounts.eve), 0);
        }

        #[ink::test]
        fn burn_works() {
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            // Create a new contract instance.
            let mut erc721 = Erc721::new();
            // Create token Id 1 for Alice
            assert_eq!(erc721.mint(1), Ok(()));
            // Alice owns 1 token.
            assert_eq!(erc721.balance_of(accounts.alice), 1);
            // Alice owns token Id 1.
            assert_eq!(erc721.owner_of(1), Some(accounts.alice));
            // Destroy token Id 1.
            assert_eq!(erc721.burn(1), Ok(()));
            // Alice does not owns tokens.
            assert_eq!(erc721.balance_of(accounts.alice), 0);
            // Token Id 1 does not exists
            assert_eq!(erc721.owner_of(1), None);
        }

        #[ink::test]
        fn burn_fails_token_not_found() {
            // Create a new contract instance.
            let mut erc721 = Erc721::new();
            // Try burning a non existent token
            assert_eq!(erc721.burn(1), Err(Error::TokenNotFound));
        }

        #[ink::test]
        fn burn_fails_not_owner() {
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            // Create a new contract instance.
            let mut erc721 = Erc721::new();
            // Create token Id 1 for Alice
            assert_eq!(erc721.mint(1), Ok(()));
            // Try burning this token with a different account
            set_caller(accounts.eve);
            assert_eq!(erc721.burn(1), Err(Error::NotOwner));
        }

        fn set_caller(sender: AccountId) {
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(sender);
        }
    }
}