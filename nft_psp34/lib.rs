#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]
        
#[openbrush::contract]
pub mod my_psp34 {
    
    // imports from openbrush
	use openbrush::traits::String;
	use openbrush::traits::Storage;
	use openbrush::contracts::ownable::*;
	use openbrush::contracts::psp34::extensions::burnable::*;
	use openbrush::contracts::psp34::extensions::mintable::*;
	use openbrush::contracts::psp34::extensions::enumerable::*;
	use openbrush::contracts::psp34::extensions::metadata::*;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct nft_psp34 {
    	#[storage_field]
		psp34: psp34::Data<Balances>,
		#[storage_field]
		ownable: ownable::Data,
		#[storage_field]
		metadata: metadata::Data,
    }
    
    // Section contains default implementation without any modifications
	impl PSP34 for nft_psp34 {}
	impl Ownable for nft_psp34 {}
	impl PSP34Burnable for nft_psp34 {
		#[ink(message)]
		#[openbrush::modifiers(only_owner)]
		fn burn(
            &mut self,
            account: AccountId,
			id: Id
        ) -> Result<(), PSP34Error> {
			self._burn_from(account, id)
		}
	}
	impl PSP34Mintable for nft_psp34 {
		#[ink(message)]
		#[openbrush::modifiers(only_owner)]
		fn mint(
            &mut self,
            account: AccountId,
			id: Id
        ) -> Result<(), PSP34Error> {
			self._mint_to(account, id)
		}
	}
	impl PSP34Enumerable for nft_psp34 {}
	impl PSP34Metadata for nft_psp34 {}
     
    impl nft_psp34 {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut _instance = Self::default();
			_instance._init_with_owner(_instance.env().caller());
			_instance._mint_to(_instance.env().caller(), Id::U8(1)).expect("Can mint");
			let collection_id = _instance.collection_id();
			_instance._set_attribute(collection_id.clone(), String::from("name"), String::from("MyPSP34"));
			_instance._set_attribute(collection_id, String::from("symbol"), String::from("MPSP"));
			_instance
        }
    }
}