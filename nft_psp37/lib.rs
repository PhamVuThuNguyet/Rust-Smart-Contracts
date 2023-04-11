#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]
        
#[openbrush::contract]
pub mod my_psp37 {
    // imports from ink!
	use ink::prelude::vec::Vec;

    // imports from openbrush
	use openbrush::traits::String;
	use openbrush::traits::Storage;
	use openbrush::contracts::ownable::*;
	use openbrush::contracts::psp37::extensions::batch::*;
	use openbrush::contracts::psp37::extensions::burnable::*;
	use openbrush::contracts::psp37::extensions::mintable::*;
	use openbrush::contracts::psp37::extensions::enumerable::*;
	use openbrush::contracts::psp37::extensions::metadata::*;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct MyERC1155 {
    	#[storage_field]
		psp37: psp37::Data<Balances>,
		#[storage_field]
		ownable: ownable::Data,
		#[storage_field]
		metadata: metadata::Data,
    }
    
    // Section contains default implementation without any modifications
	impl PSP37 for MyERC1155 {}
	impl Ownable for MyERC1155 {}
	impl PSP37Batch for MyERC1155 {}
	impl PSP37Burnable for MyERC1155 {
		#[ink(message)]
		#[openbrush::modifiers(only_owner)]
		fn burn(
            &mut self,
            account: AccountId,
			ids_amounts: Vec<(Id, Balance)>
        ) -> Result<(), PSP37Error> {
			self._burn_from(account, ids_amounts)
		}
	}
	impl PSP37Mintable for MyERC1155 {
		#[ink(message)]
		#[openbrush::modifiers(only_owner)]
		fn mint(
            &mut self,
            account: AccountId,
			ids_amounts: Vec<(Id, Balance)>
        ) -> Result<(), PSP37Error> {
			self._mint_to(account, ids_amounts	)
		}
	}
	impl PSP37Enumerable for MyERC1155 {}
	impl PSP37Metadata for MyERC1155 {}
     
    impl MyERC1155 {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut _instance = Self::default();
			_instance._init_with_owner(_instance.env().caller());
			_instance
        }
    }
}