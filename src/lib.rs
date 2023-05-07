use anchor_lang::prelude::*;

#[derive(AnchorDeserialize, AnchorSerialize, Copy, Clone, PartialEq, Eq)]
pub struct Dummy {
    pub foo: u64,
    pub bar: u64,
}

#[derive(AnchorDeserialize, AnchorSerialize, Copy, Clone, PartialEq, Eq)]
pub enum DummyEnum {
    Foo,
    Bar,
}
