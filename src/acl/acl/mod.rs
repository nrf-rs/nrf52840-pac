#[doc = "Description cluster[0]: Configure the word-aligned start address of region 0 to protect"]
pub struct ADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description cluster[0]: Configure the word-aligned start address of region 0 to protect"]
pub mod addr;
# [ doc = "Description cluster[0]: Size of region to protect counting from address ACL[0].ADDR. Write '0' as no effect." ]
pub struct SIZE {
    register: ::vcell::VolatileCell<u32>,
}
# [ doc = "Description cluster[0]: Size of region to protect counting from address ACL[0].ADDR. Write '0' as no effect." ]
pub mod size;
# [ doc = "Description cluster[0]: Access permissions for region 0 as defined by start address ACL[0].ADDR and size ACL[0].SIZE" ]
pub struct PERM {
    register: ::vcell::VolatileCell<u32>,
}
# [ doc = "Description cluster[0]: Access permissions for region 0 as defined by start address ACL[0].ADDR and size ACL[0].SIZE" ]
pub mod perm;
#[doc = "Unspecified"]
pub struct UNUSED0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Unspecified"]
pub mod unused0;
