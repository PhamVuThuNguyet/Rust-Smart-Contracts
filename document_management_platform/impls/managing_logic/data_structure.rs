use crate::traits::managing_logic::*;
use openbrush::traits::{ AccountId, String };

#[derive(scale::Encode, scale::Decode, Clone, Debug)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct PostOwner {
    pub user: AccountId,
    pub user_role: UserRole,
    pub point: u32,
}

#[derive(scale::Encode, scale::Decode, Clone, Debug)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct Version {
    pub version_id: u8,
    pub contributor: AccountId,
    pub ipfs_hash: String,
    pub state: DocumentState,
}

#[derive(scale::Encode, scale::Decode, Clone, Debug)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct NftDocument {
    pub id: u32,
    pub owner: AccountId,
    pub title: String,
    pub ipfs_hash_doc: String,
    pub post_owner: Vec<PostOwner>,
    pub versions: Vec<Version>,
    pub version_id_publish: u8,
    pub number_upload: u32,
}