#[doc = "Description cluster[0]: Data pointer"]
pub struct PTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description cluster[0]: Data pointer"]
pub mod ptr;
#[doc = "Description cluster[0]: Maximum number of bytes to transfer"]
pub struct MAXCNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description cluster[0]: Maximum number of bytes to transfer"]
pub mod maxcnt;
#[doc = "Description cluster[0]: Number of bytes transferred in the last transaction"]
pub struct AMOUNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description cluster[0]: Number of bytes transferred in the last transaction"]
pub mod amount;
