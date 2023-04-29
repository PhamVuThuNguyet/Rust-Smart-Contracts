#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod managing_logic {
    use ink_prelude::vec::Vec;
    use openbrush::traits::{ String };

    use document_management_platform::traits::managing_logic::*;
    use document_management_platform::impls::managing_logic::data_structure::*;

    use native_token::native_token::NativeTokenRef;
    use nft_token::nft_token::{ NftTokenRef, PSP37MintableRef };

    #[ink(storage)]
    pub struct NftCollection {
        docs: Vec<NftDocument>,
        native_token_ref: NativeTokenRef,
        nft_token_ref: NftTokenRef,
    }

    impl NftCollection {
        #[ink(constructor)]
        pub fn new(
            native_token_code_hash: Hash,
            nft_token_code_hash: Hash,
            initial_supply: Balance,
            name: Option<String>,
            symbol: Option<String>,
            decimal: u8
        ) -> Self {
            let native_token_contract = NativeTokenRef::new(initial_supply, name, symbol, decimal)
                .code_hash(native_token_code_hash)
                .endowment(0)
                .salt_bytes([0xde, 0xad, 0xbe, 0xef])
                .instantiate();

            let nft_token_contract = NftTokenRef::new()
                .code_hash(nft_token_code_hash)
                .endowment(0)
                .salt_bytes([0xde, 0xad, 0xbe, 0xef])
                .instantiate();

            Self {
                docs: Vec::new(),
                native_token_ref: native_token_contract,
                nft_token_ref: nft_token_contract,
            }
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
        pub fn get_version_doc_length(&self, docId: u32) -> u8 {
            self.docs[doc_id as usize].versions.len() as u8
        }

        #[ink(message)]
        pub fn add_nft(&mut self, nft: NftDocument) {
            self.docs.push(nft);
        }

        #[ink(message)]
        pub fn get_document(&self, docId: u32) -> Option<NftDocument> {
            self.docs.get(docId as usize).cloned()
        }

        #[ink(message)]
        pub fn create_document(&mut self, title: String, ipfs_hash: String) -> NftDocument {
            let post_id = self.get_nfts_length();
            let caller = self.env().caller();

            let new_version = Version {
                version_id: 0,
                contributor: caller,
                ipfs_hash: ipfs_hash.clone(),
                state: DocumentState::Publish,
            };

            let tokenId: Id = nft_token::nft_token::Id::U32(post_id);
            let tokenBalance: u128 = 100000;
            let initVersionId: u8 = 0;
            let versionId = vec![initVersionId];
            let ids_amount: Vec<(Id, Balance)> = vec![(tokenId.clone(), tokenBalance)];

            self.nft_token._mint(caller, ids_amount);
            self.nft_token.set_attribute(tokenId, versionId, ipfs_hash.clone());

            let mut versions = Vec::new();
            versions.push(new_version);

            let vec_post_owner = Vec::new();

            let new_doc = NftDocument {
                id: post_id,
                owner: caller,
                title: title.clone(),
                ipfs_hash_doc: ipfs_hash.clone(),
                post_owner: vec_post_owner.clone(),
                versions: versions.clone(),
                version_id_publish: 0,
                number_upload: 1,
            };

            self.add_nft(new_doc);

            NftDocument {
                id: post_id,
                owner: caller,
                title,
                ipfs_hash_doc: ipfs_hash,
                post_owner: vec_post_owner,
                versions,
                version_id_publish: 0,
                number_upload: 1,
            }
        }

        #[ink(message)]
        pub fn create_version_document(&mut self, doc_id: u32, ipfs_hash: String) -> Version {
            let caller = self.env().caller();
            let next_version_id = self.get_version_doc_length(doc_id);

            let token_id: Id = nft_token::nft_token::Id::U32(doc_id);
            let version_id = vec![next_version_id];

            let new_version = Version {
                version_id: next_version_id,
                contributor: caller,
                ipfs_hash: ipfs_hash.clone(),
                state: DocumentState::Publish,
            };

            self.nft_token.set_attribute(token_id, version_id, ipfs_hash.clone());

            let publish = self.docs[doc_id as usize].version_id_publish;

            self.docs[doc_id as usize].ipfs_hash_doc = ipfs_hash.clone();
            self.docs[doc_id as usize].versions[publish as usize].state = DocumentState::Archived;
            self.docs[doc_id as usize].versions.push(new_version);
            self.docs[doc_id as usize].version_id_publish = next_version_id;
            self.docs[doc_id as usize].number_upload += 1;

            Version {
                version_id: next_version_id,
                contributor: caller,
                ipfs_hash: ipfs_hash,
                state: DocumentState::Publish,
            }
        }

        #[ink(message)]
        pub fn get_total_supply(&self, token_id: Option<Id>) -> Balance {
            return self.nft_token_ref._total_supply(token_id);
        }

        #[ink(message)]
        pub fn get_attribute_version(&self, token_id: Id, version_id: Vec<u8>) -> Option<Vec<u8>> {
            return self.nft_token_ref.get_attribute(token_id, version_id);
        }
    }
}