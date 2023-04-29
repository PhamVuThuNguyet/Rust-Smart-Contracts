use openbrush::traits::{ String, Balance };
use ink::prelude::vec::Vec;
use crate::impls::managing_logic::data_structure::*;
use openbrush::contracts::traits::psp37::{ extensions::{ metadata::* } };

#[derive(scale::Encode, scale::Decode, Debug, PartialEq, Eq, Copy, Clone)]
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

#[derive(scale::Encode, scale::Decode, Clone, Debug)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum UserRole {
    Developer,
    CoAuthor,
    Partner,
}

#[derive(scale::Encode, scale::Decode, Clone, Debug)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum DocumentState {
    Publish,
    Archived,
}

#[openbrush::wrapper]
pub type ManagingLogicRef = dyn ManagingLogic;

#[openbrush::trait_definition]
pub trait ManagingLogic {
    #[ink(message)]
    fn get_nfts(&self) -> Vec<NftDocument>;

    #[ink(message)]
    fn get_nfts_length(&self) -> u32;

    #[ink(message)]
    fn get_version_doc_length(&self, doc_id: u32) -> u8;

    #[ink(message)]
    fn add_nft(&mut self, nft: NftDocument);

    #[ink(message)]
    fn get_document(&self, doc_id: u32) -> Option<NftDocument>;

    #[ink(message)]
    fn create_document(&mut self, title: String, ipfs_hash: String) -> NftDocument;

    #[ink(message)]
    fn create_version_document(&mut self, doc_id: u32, ipfs_hash: String) -> Version;

    #[ink(message)]
    fn get_total_supply(&self, token_id: Option<Id>) -> Balance;

    #[ink(message)]
    fn get_attribute_version(&self, token_id: Id, version_id: Vec<u8>) -> Option<Vec<u8>>;
}