use openbrush::contracts::traits::{
    ownable::*,
    psp22::{ extensions::{ burnable::*, mintable::*, wrapper::*, metadata::* } },
};

#[openbrush::wrapper]
pub type NativeTokenRef = dyn PSP22 +
    PSP22Mintable +
    PSP22Burnable +
    PSP22Wrapper +
    PSP22Metadata +
    Ownable;

#[openbrush::trait_definition]
pub trait NativeToken: PSP22 +
    PSP22Mintable +
    PSP22Burnable +
    PSP22Wrapper +
    PSP22Metadata +
    Ownable {}