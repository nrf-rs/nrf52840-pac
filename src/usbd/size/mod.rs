# [ doc = "Description collection[0]: Amount of bytes received last in the data stage of this OUT endpoint" ]
pub struct EPOUT {
    register: ::vcell::VolatileCell<u32>,
}
# [ doc = "Description collection[0]: Amount of bytes received last in the data stage of this OUT endpoint" ]
pub mod epout;
#[doc = "Amount of bytes received last on this iso OUT data endpoint"]
pub struct ISOOUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Amount of bytes received last on this iso OUT data endpoint"]
pub mod isoout;
