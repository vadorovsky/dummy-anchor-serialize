use anchor_lang::prelude::*;

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct Dummy {
    pub foo: u64,
    pub bar: u64,
}
