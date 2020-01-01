#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 1024usize],
    #[doc = "0x400 - Ready flag"]
    pub ready: READY,
    _reserved1: [u8; 4usize],
    #[doc = "0x408 - Ready flag"]
    pub readynext: READYNEXT,
    _reserved2: [u8; 248usize],
    #[doc = "0x504 - Configuration register"]
    pub config: CONFIG,
    _reserved_3_erasepage: [u8; 4usize],
    #[doc = "0x50c - Register for erasing all non-volatile user memory"]
    pub eraseall: ERASEALL,
    #[doc = "0x510 - Deprecated register - Register for erasing a page in code area. Equivalent to ERASEPAGE."]
    pub erasepcr0: ERASEPCR0,
    #[doc = "0x514 - Register for erasing user information configuration registers"]
    pub eraseuicr: ERASEUICR,
    #[doc = "0x518 - Register for partial erase of a page in code area"]
    pub erasepagepartial: ERASEPAGEPARTIAL,
    #[doc = "0x51c - Register for partial erase configuration"]
    pub erasepagepartialcfg: ERASEPAGEPARTIALCFG,
    _reserved9: [u8; 32usize],
    #[doc = "0x540 - I-code cache configuration register."]
    pub icachecnf: ICACHECNF,
    _reserved10: [u8; 4usize],
    #[doc = "0x548 - I-code cache hit counter."]
    pub ihit: IHIT,
    #[doc = "0x54c - I-code cache miss counter."]
    pub imiss: IMISS,
}
impl RegisterBlock {
    #[doc = "0x508 - Deprecated register - Register for erasing a page in code area. Equivalent to ERASEPAGE."]
    #[inline(always)]
    pub fn erasepcr1(&self) -> &ERASEPCR1 {
        unsafe { &*(((self as *const Self) as *const u8).add(1288usize) as *const ERASEPCR1) }
    }
    #[doc = "0x508 - Deprecated register - Register for erasing a page in code area. Equivalent to ERASEPAGE."]
    #[inline(always)]
    pub fn erasepcr1_mut(&self) -> &mut ERASEPCR1 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1288usize) as *mut ERASEPCR1) }
    }
    #[doc = "0x508 - Register for erasing a page in code area"]
    #[inline(always)]
    pub fn erasepage(&self) -> &ERASEPAGE {
        unsafe { &*(((self as *const Self) as *const u8).add(1288usize) as *const ERASEPAGE) }
    }
    #[doc = "0x508 - Register for erasing a page in code area"]
    #[inline(always)]
    pub fn erasepage_mut(&self) -> &mut ERASEPAGE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1288usize) as *mut ERASEPAGE) }
    }
}
#[doc = "Ready flag\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ready](ready) module"]
pub type READY = crate::Reg<u32, _READY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _READY;
#[doc = "`read()` method returns [ready::R](ready::R) reader structure"]
impl crate::Readable for READY {}
#[doc = "Ready flag"]
pub mod ready;
#[doc = "Ready flag\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [readynext](readynext) module"]
pub type READYNEXT = crate::Reg<u32, _READYNEXT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _READYNEXT;
#[doc = "`read()` method returns [readynext::R](readynext::R) reader structure"]
impl crate::Readable for READYNEXT {}
#[doc = "Ready flag"]
pub mod readynext;
#[doc = "Configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config](config) module"]
pub type CONFIG = crate::Reg<u32, _CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIG;
#[doc = "`read()` method returns [config::R](config::R) reader structure"]
impl crate::Readable for CONFIG {}
#[doc = "`write(|w| ..)` method takes [config::W](config::W) writer structure"]
impl crate::Writable for CONFIG {}
#[doc = "Configuration register"]
pub mod config;
#[doc = "Register for erasing a page in code area\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [erasepage](erasepage) module"]
pub type ERASEPAGE = crate::Reg<u32, _ERASEPAGE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ERASEPAGE;
#[doc = "`read()` method returns [erasepage::R](erasepage::R) reader structure"]
impl crate::Readable for ERASEPAGE {}
#[doc = "`write(|w| ..)` method takes [erasepage::W](erasepage::W) writer structure"]
impl crate::Writable for ERASEPAGE {}
#[doc = "Register for erasing a page in code area"]
pub mod erasepage;
#[doc = "Deprecated register - Register for erasing a page in code area. Equivalent to ERASEPAGE.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [erasepcr1](erasepcr1) module"]
pub type ERASEPCR1 = crate::Reg<u32, _ERASEPCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ERASEPCR1;
#[doc = "`read()` method returns [erasepcr1::R](erasepcr1::R) reader structure"]
impl crate::Readable for ERASEPCR1 {}
#[doc = "`write(|w| ..)` method takes [erasepcr1::W](erasepcr1::W) writer structure"]
impl crate::Writable for ERASEPCR1 {}
#[doc = "Deprecated register - Register for erasing a page in code area. Equivalent to ERASEPAGE."]
pub mod erasepcr1;
#[doc = "Register for erasing all non-volatile user memory\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eraseall](eraseall) module"]
pub type ERASEALL = crate::Reg<u32, _ERASEALL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ERASEALL;
#[doc = "`read()` method returns [eraseall::R](eraseall::R) reader structure"]
impl crate::Readable for ERASEALL {}
#[doc = "`write(|w| ..)` method takes [eraseall::W](eraseall::W) writer structure"]
impl crate::Writable for ERASEALL {}
#[doc = "Register for erasing all non-volatile user memory"]
pub mod eraseall;
#[doc = "Deprecated register - Register for erasing a page in code area. Equivalent to ERASEPAGE.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [erasepcr0](erasepcr0) module"]
pub type ERASEPCR0 = crate::Reg<u32, _ERASEPCR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ERASEPCR0;
#[doc = "`read()` method returns [erasepcr0::R](erasepcr0::R) reader structure"]
impl crate::Readable for ERASEPCR0 {}
#[doc = "`write(|w| ..)` method takes [erasepcr0::W](erasepcr0::W) writer structure"]
impl crate::Writable for ERASEPCR0 {}
#[doc = "Deprecated register - Register for erasing a page in code area. Equivalent to ERASEPAGE."]
pub mod erasepcr0;
#[doc = "Register for erasing user information configuration registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eraseuicr](eraseuicr) module"]
pub type ERASEUICR = crate::Reg<u32, _ERASEUICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ERASEUICR;
#[doc = "`read()` method returns [eraseuicr::R](eraseuicr::R) reader structure"]
impl crate::Readable for ERASEUICR {}
#[doc = "`write(|w| ..)` method takes [eraseuicr::W](eraseuicr::W) writer structure"]
impl crate::Writable for ERASEUICR {}
#[doc = "Register for erasing user information configuration registers"]
pub mod eraseuicr;
#[doc = "Register for partial erase of a page in code area\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [erasepagepartial](erasepagepartial) module"]
pub type ERASEPAGEPARTIAL = crate::Reg<u32, _ERASEPAGEPARTIAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ERASEPAGEPARTIAL;
#[doc = "`read()` method returns [erasepagepartial::R](erasepagepartial::R) reader structure"]
impl crate::Readable for ERASEPAGEPARTIAL {}
#[doc = "`write(|w| ..)` method takes [erasepagepartial::W](erasepagepartial::W) writer structure"]
impl crate::Writable for ERASEPAGEPARTIAL {}
#[doc = "Register for partial erase of a page in code area"]
pub mod erasepagepartial;
#[doc = "Register for partial erase configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [erasepagepartialcfg](erasepagepartialcfg) module"]
pub type ERASEPAGEPARTIALCFG = crate::Reg<u32, _ERASEPAGEPARTIALCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ERASEPAGEPARTIALCFG;
#[doc = "`read()` method returns [erasepagepartialcfg::R](erasepagepartialcfg::R) reader structure"]
impl crate::Readable for ERASEPAGEPARTIALCFG {}
#[doc = "`write(|w| ..)` method takes [erasepagepartialcfg::W](erasepagepartialcfg::W) writer structure"]
impl crate::Writable for ERASEPAGEPARTIALCFG {}
#[doc = "Register for partial erase configuration"]
pub mod erasepagepartialcfg;
#[doc = "I-code cache configuration register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icachecnf](icachecnf) module"]
pub type ICACHECNF = crate::Reg<u32, _ICACHECNF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICACHECNF;
#[doc = "`read()` method returns [icachecnf::R](icachecnf::R) reader structure"]
impl crate::Readable for ICACHECNF {}
#[doc = "`write(|w| ..)` method takes [icachecnf::W](icachecnf::W) writer structure"]
impl crate::Writable for ICACHECNF {}
#[doc = "I-code cache configuration register."]
pub mod icachecnf;
#[doc = "I-code cache hit counter.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ihit](ihit) module"]
pub type IHIT = crate::Reg<u32, _IHIT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IHIT;
#[doc = "`read()` method returns [ihit::R](ihit::R) reader structure"]
impl crate::Readable for IHIT {}
#[doc = "`write(|w| ..)` method takes [ihit::W](ihit::W) writer structure"]
impl crate::Writable for IHIT {}
#[doc = "I-code cache hit counter."]
pub mod ihit;
#[doc = "I-code cache miss counter.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imiss](imiss) module"]
pub type IMISS = crate::Reg<u32, _IMISS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMISS;
#[doc = "`read()` method returns [imiss::R](imiss::R) reader structure"]
impl crate::Readable for IMISS {}
#[doc = "`write(|w| ..)` method takes [imiss::W](imiss::W) writer structure"]
impl crate::Writable for IMISS {}
#[doc = "I-code cache miss counter."]
pub mod imiss;
