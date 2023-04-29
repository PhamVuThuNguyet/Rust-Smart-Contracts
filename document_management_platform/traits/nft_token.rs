use openbrush::contracts::traits::{
    ownable::*,
    psp37::{ extensions::{ burnable::*, mintable::*, metadata::*, enumerable::*, batch::* } },
};

#[openbrush::wrapper]
pub type NftTokenRef = dyn PSP37 +
    PSP37Mintable +
    PSP37Burnable +
    PSP37Enumerable +
    PSP37Metadata +
    PSP37Batch +
    Ownable;

#[openbrush::trait_definition]
pub trait NftToken: PSP37 +
    PSP37Mintable +
    PSP37Burnable +
    PSP37Enumerable +
    PSP37Metadata +
    PSP37Batch +
    Ownable {}