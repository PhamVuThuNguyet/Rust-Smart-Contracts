#![cfg_attr(not(feature = "std"), no_std)]
#[ink::contract]
mod document_management {
    pub type DocId = u32;

    use ink_prelude::string::String;
    use ink_prelude::vec::Vec;
    use scale::{ Decode, Encode };

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

    #[derive(scale::Encode, scale::Decode, Clone, Debug)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Version {
        pub contributor: AccountId,
        pub ipfs_hash: String,
    }

    #[derive(scale::Encode, scale::Decode, Clone, Debug)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct NftDocument {
        pub id: DocId,
        pub owner: AccountId,
        pub title: String,
        pub versions: Vec<Version>,
    }

    #[ink(storage)]
    pub struct NftCollection {
        docs: Vec<NftDocument>,
    }

    impl NftCollection {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { docs: Vec::new() }
        }

        #[ink(message)]
        pub fn get_nfts(&self) -> Vec<NftDocument> {
            self.docs.clone()
        }

        #[ink(message)]
        pub fn get_nfts_length(&self) -> u32 {
            self.docs.len() as u32
        }

        #[ink(message)]
        pub fn add_nft(&mut self, nft: NftDocument) {
            self.docs.push(nft);
        }

        #[ink(message)]
        pub fn get_document(&self, doc_id: DocId) -> Option<NftDocument> {
            self.docs.get(doc_id as usize).cloned()
        }

        #[ink(message)]
        pub fn create_document(&mut self, title: String, ipfs_hash: String) -> DocId {
            // Create a new NFT document with the given attributes and return its post_id
            let post_id = self.get_nfts_length();
            let caller = self.env().caller();
            let new_version = Version {
                contributor: caller,
                ipfs_hash: ipfs_hash,
            };
            let mut versions = Vec::new();
            versions.push(new_version);
            let new_doc = NftDocument {
                id: post_id,
                owner: caller,
                title,
                versions,
            };
            self.add_nft(new_doc);
            post_id
        }

        #[ink(message)]
        pub fn update_document(&mut self, doc_id: DocId, ipfs_hash: String) -> Result<(), Error> {
            // Create a new NFT document with the given attributes and return its post_id
            let caller = self.env().caller();
            let new_version = Version {
                contributor: caller,
                ipfs_hash: ipfs_hash,
            };
            self.get_document(doc_id).unwrap().versions.push(new_version);
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

        /// We test if the create function does its job.
        #[ink::test]
        fn create_document_test() {
            let mut nft_collection = NftCollection::new();
            assert_eq!(
                nft_collection.create_document(
                    "New Doc".to_string(),
                    "https://www.facebook.com/".to_string()
                ),
                0
            )
        }

        /// We test if the update function does its job.
        #[ink::test]
        fn update_document_test() {
            let mut nft_collection = NftCollection::new();
            assert_eq!(
                nft_collection.create_document(
                    "New Doc".to_string(),
                    "https://www.facebook.com/".to_string()
                ),
                0
            );
            assert_eq!(
                nft_collection.update_document(0, "https://www.facebook.com.vn/".to_string()),
                Ok(())
            )
        }
    }
}