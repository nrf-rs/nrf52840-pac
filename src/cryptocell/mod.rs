#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 1280usize],
    #[doc = "0x500 - Control power and clock for ARM CryptoCell subsystem"]
    pub enable: ENABLE,
}
#[doc = "Control power and clock for ARM CryptoCell subsystem"]
pub struct ENABLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control power and clock for ARM CryptoCell subsystem"]
pub mod enable;
