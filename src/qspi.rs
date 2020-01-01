#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Activate QSPI interface"]
    pub tasks_activate: TASKS_ACTIVATE,
    #[doc = "0x04 - Start transfer from external flash memory to internal RAM"]
    pub tasks_readstart: TASKS_READSTART,
    #[doc = "0x08 - Start transfer from internal RAM to external flash memory"]
    pub tasks_writestart: TASKS_WRITESTART,
    #[doc = "0x0c - Start external flash memory erase operation"]
    pub tasks_erasestart: TASKS_ERASESTART,
    #[doc = "0x10 - Deactivate QSPI interface"]
    pub tasks_deactivate: TASKS_DEACTIVATE,
    _reserved5: [u8; 236usize],
    #[doc = "0x100 - QSPI peripheral is ready. This event will be generated as a response to any QSPI task."]
    pub events_ready: EVENTS_READY,
    _reserved6: [u8; 508usize],
    #[doc = "0x300 - Enable or disable interrupt"]
    pub inten: INTEN,
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved9: [u8; 500usize],
    #[doc = "0x500 - Enable QSPI peripheral and acquire the pins selected in PSELn registers"]
    pub enable: ENABLE,
    #[doc = "0x504 - Unspecified"]
    pub read: READ,
    #[doc = "0x510 - Unspecified"]
    pub write: WRITE,
    #[doc = "0x51c - Unspecified"]
    pub erase: ERASE,
    #[doc = "0x524 - Unspecified"]
    pub psel: PSEL,
    #[doc = "0x540 - Address offset into the external memory for Execute in Place operation."]
    pub xipoffset: XIPOFFSET,
    #[doc = "0x544 - Interface configuration."]
    pub ifconfig0: IFCONFIG0,
    _reserved16: [u8; 184usize],
    #[doc = "0x600 - Interface configuration."]
    pub ifconfig1: IFCONFIG1,
    #[doc = "0x604 - Status register."]
    pub status: STATUS,
    _reserved18: [u8; 12usize],
    #[doc = "0x614 - Set the duration required to enter/exit deep power-down mode (DPM)."]
    pub dpmdur: DPMDUR,
    _reserved19: [u8; 12usize],
    #[doc = "0x624 - Extended address configuration."]
    pub addrconf: ADDRCONF,
    _reserved20: [u8; 12usize],
    #[doc = "0x634 - Custom instruction configuration register."]
    pub cinstrconf: CINSTRCONF,
    #[doc = "0x638 - Custom instruction data register 0."]
    pub cinstrdat0: CINSTRDAT0,
    #[doc = "0x63c - Custom instruction data register 1."]
    pub cinstrdat1: CINSTRDAT1,
    #[doc = "0x640 - SPI interface timing."]
    pub iftiming: IFTIMING,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct READ {
    #[doc = "0x00 - Flash memory source address"]
    pub src: self::read::SRC,
    #[doc = "0x04 - RAM destination address"]
    pub dst: self::read::DST,
    #[doc = "0x08 - Read transfer length"]
    pub cnt: self::read::CNT,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod read;
#[doc = r"Register block"]
#[repr(C)]
pub struct WRITE {
    #[doc = "0x00 - Flash destination address"]
    pub dst: self::write::DST,
    #[doc = "0x04 - RAM source address"]
    pub src: self::write::SRC,
    #[doc = "0x08 - Write transfer length"]
    pub cnt: self::write::CNT,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod write;
#[doc = r"Register block"]
#[repr(C)]
pub struct ERASE {
    #[doc = "0x00 - Start address of flash block to be erased"]
    pub ptr: self::erase::PTR,
    #[doc = "0x04 - Size of block to be erased."]
    pub len: self::erase::LEN,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod erase;
#[doc = r"Register block"]
#[repr(C)]
pub struct PSEL {
    #[doc = "0x00 - Pin select for serial clock SCK"]
    pub sck: self::psel::SCK,
    #[doc = "0x04 - Pin select for chip select signal CSN."]
    pub csn: self::psel::CSN,
    _reserved2: [u8; 4usize],
    #[doc = "0x0c - Pin select for serial data MOSI/IO0."]
    pub io0: self::psel::IO0,
    #[doc = "0x10 - Pin select for serial data MISO/IO1."]
    pub io1: self::psel::IO1,
    #[doc = "0x14 - Pin select for serial data IO2."]
    pub io2: self::psel::IO2,
    #[doc = "0x18 - Pin select for serial data IO3."]
    pub io3: self::psel::IO3,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod psel;
#[doc = "Activate QSPI interface\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_activate](tasks_activate) module"]
pub type TASKS_ACTIVATE = crate::Reg<u32, _TASKS_ACTIVATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_ACTIVATE;
#[doc = "`write(|w| ..)` method takes [tasks_activate::W](tasks_activate::W) writer structure"]
impl crate::Writable for TASKS_ACTIVATE {}
#[doc = "Activate QSPI interface"]
pub mod tasks_activate;
#[doc = "Start transfer from external flash memory to internal RAM\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_readstart](tasks_readstart) module"]
pub type TASKS_READSTART = crate::Reg<u32, _TASKS_READSTART>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_READSTART;
#[doc = "`write(|w| ..)` method takes [tasks_readstart::W](tasks_readstart::W) writer structure"]
impl crate::Writable for TASKS_READSTART {}
#[doc = "Start transfer from external flash memory to internal RAM"]
pub mod tasks_readstart;
#[doc = "Start transfer from internal RAM to external flash memory\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_writestart](tasks_writestart) module"]
pub type TASKS_WRITESTART = crate::Reg<u32, _TASKS_WRITESTART>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_WRITESTART;
#[doc = "`write(|w| ..)` method takes [tasks_writestart::W](tasks_writestart::W) writer structure"]
impl crate::Writable for TASKS_WRITESTART {}
#[doc = "Start transfer from internal RAM to external flash memory"]
pub mod tasks_writestart;
#[doc = "Start external flash memory erase operation\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_erasestart](tasks_erasestart) module"]
pub type TASKS_ERASESTART = crate::Reg<u32, _TASKS_ERASESTART>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_ERASESTART;
#[doc = "`write(|w| ..)` method takes [tasks_erasestart::W](tasks_erasestart::W) writer structure"]
impl crate::Writable for TASKS_ERASESTART {}
#[doc = "Start external flash memory erase operation"]
pub mod tasks_erasestart;
#[doc = "Deactivate QSPI interface\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_deactivate](tasks_deactivate) module"]
pub type TASKS_DEACTIVATE = crate::Reg<u32, _TASKS_DEACTIVATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_DEACTIVATE;
#[doc = "`write(|w| ..)` method takes [tasks_deactivate::W](tasks_deactivate::W) writer structure"]
impl crate::Writable for TASKS_DEACTIVATE {}
#[doc = "Deactivate QSPI interface"]
pub mod tasks_deactivate;
#[doc = "QSPI peripheral is ready. This event will be generated as a response to any QSPI task.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_ready](events_ready) module"]
pub type EVENTS_READY = crate::Reg<u32, _EVENTS_READY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_READY;
#[doc = "`read()` method returns [events_ready::R](events_ready::R) reader structure"]
impl crate::Readable for EVENTS_READY {}
#[doc = "`write(|w| ..)` method takes [events_ready::W](events_ready::W) writer structure"]
impl crate::Writable for EVENTS_READY {}
#[doc = "QSPI peripheral is ready. This event will be generated as a response to any QSPI task."]
pub mod events_ready;
#[doc = "Enable or disable interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](inten) module"]
pub type INTEN = crate::Reg<u32, _INTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTEN;
#[doc = "`read()` method returns [inten::R](inten::R) reader structure"]
impl crate::Readable for INTEN {}
#[doc = "`write(|w| ..)` method takes [inten::W](inten::W) writer structure"]
impl crate::Writable for INTEN {}
#[doc = "Enable or disable interrupt"]
pub mod inten;
#[doc = "Enable interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenset](intenset) module"]
pub type INTENSET = crate::Reg<u32, _INTENSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTENSET;
#[doc = "`read()` method returns [intenset::R](intenset::R) reader structure"]
impl crate::Readable for INTENSET {}
#[doc = "`write(|w| ..)` method takes [intenset::W](intenset::W) writer structure"]
impl crate::Writable for INTENSET {}
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "Disable interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenclr](intenclr) module"]
pub type INTENCLR = crate::Reg<u32, _INTENCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTENCLR;
#[doc = "`read()` method returns [intenclr::R](intenclr::R) reader structure"]
impl crate::Readable for INTENCLR {}
#[doc = "`write(|w| ..)` method takes [intenclr::W](intenclr::W) writer structure"]
impl crate::Writable for INTENCLR {}
#[doc = "Disable interrupt"]
pub mod intenclr;
#[doc = "Enable QSPI peripheral and acquire the pins selected in PSELn registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enable](enable) module"]
pub type ENABLE = crate::Reg<u32, _ENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENABLE;
#[doc = "`read()` method returns [enable::R](enable::R) reader structure"]
impl crate::Readable for ENABLE {}
#[doc = "`write(|w| ..)` method takes [enable::W](enable::W) writer structure"]
impl crate::Writable for ENABLE {}
#[doc = "Enable QSPI peripheral and acquire the pins selected in PSELn registers"]
pub mod enable;
#[doc = "Address offset into the external memory for Execute in Place operation.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xipoffset](xipoffset) module"]
pub type XIPOFFSET = crate::Reg<u32, _XIPOFFSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XIPOFFSET;
#[doc = "`read()` method returns [xipoffset::R](xipoffset::R) reader structure"]
impl crate::Readable for XIPOFFSET {}
#[doc = "`write(|w| ..)` method takes [xipoffset::W](xipoffset::W) writer structure"]
impl crate::Writable for XIPOFFSET {}
#[doc = "Address offset into the external memory for Execute in Place operation."]
pub mod xipoffset;
#[doc = "Interface configuration.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifconfig0](ifconfig0) module"]
pub type IFCONFIG0 = crate::Reg<u32, _IFCONFIG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IFCONFIG0;
#[doc = "`read()` method returns [ifconfig0::R](ifconfig0::R) reader structure"]
impl crate::Readable for IFCONFIG0 {}
#[doc = "`write(|w| ..)` method takes [ifconfig0::W](ifconfig0::W) writer structure"]
impl crate::Writable for IFCONFIG0 {}
#[doc = "Interface configuration."]
pub mod ifconfig0;
#[doc = "Interface configuration.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifconfig1](ifconfig1) module"]
pub type IFCONFIG1 = crate::Reg<u32, _IFCONFIG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IFCONFIG1;
#[doc = "`read()` method returns [ifconfig1::R](ifconfig1::R) reader structure"]
impl crate::Readable for IFCONFIG1 {}
#[doc = "`write(|w| ..)` method takes [ifconfig1::W](ifconfig1::W) writer structure"]
impl crate::Writable for IFCONFIG1 {}
#[doc = "Interface configuration."]
pub mod ifconfig1;
#[doc = "Status register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "Status register."]
pub mod status;
#[doc = "Set the duration required to enter/exit deep power-down mode (DPM).\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dpmdur](dpmdur) module"]
pub type DPMDUR = crate::Reg<u32, _DPMDUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPMDUR;
#[doc = "`read()` method returns [dpmdur::R](dpmdur::R) reader structure"]
impl crate::Readable for DPMDUR {}
#[doc = "`write(|w| ..)` method takes [dpmdur::W](dpmdur::W) writer structure"]
impl crate::Writable for DPMDUR {}
#[doc = "Set the duration required to enter/exit deep power-down mode (DPM)."]
pub mod dpmdur;
#[doc = "Extended address configuration.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addrconf](addrconf) module"]
pub type ADDRCONF = crate::Reg<u32, _ADDRCONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDRCONF;
#[doc = "`read()` method returns [addrconf::R](addrconf::R) reader structure"]
impl crate::Readable for ADDRCONF {}
#[doc = "`write(|w| ..)` method takes [addrconf::W](addrconf::W) writer structure"]
impl crate::Writable for ADDRCONF {}
#[doc = "Extended address configuration."]
pub mod addrconf;
#[doc = "Custom instruction configuration register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cinstrconf](cinstrconf) module"]
pub type CINSTRCONF = crate::Reg<u32, _CINSTRCONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CINSTRCONF;
#[doc = "`read()` method returns [cinstrconf::R](cinstrconf::R) reader structure"]
impl crate::Readable for CINSTRCONF {}
#[doc = "`write(|w| ..)` method takes [cinstrconf::W](cinstrconf::W) writer structure"]
impl crate::Writable for CINSTRCONF {}
#[doc = "Custom instruction configuration register."]
pub mod cinstrconf;
#[doc = "Custom instruction data register 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cinstrdat0](cinstrdat0) module"]
pub type CINSTRDAT0 = crate::Reg<u32, _CINSTRDAT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CINSTRDAT0;
#[doc = "`read()` method returns [cinstrdat0::R](cinstrdat0::R) reader structure"]
impl crate::Readable for CINSTRDAT0 {}
#[doc = "`write(|w| ..)` method takes [cinstrdat0::W](cinstrdat0::W) writer structure"]
impl crate::Writable for CINSTRDAT0 {}
#[doc = "Custom instruction data register 0."]
pub mod cinstrdat0;
#[doc = "Custom instruction data register 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cinstrdat1](cinstrdat1) module"]
pub type CINSTRDAT1 = crate::Reg<u32, _CINSTRDAT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CINSTRDAT1;
#[doc = "`read()` method returns [cinstrdat1::R](cinstrdat1::R) reader structure"]
impl crate::Readable for CINSTRDAT1 {}
#[doc = "`write(|w| ..)` method takes [cinstrdat1::W](cinstrdat1::W) writer structure"]
impl crate::Writable for CINSTRDAT1 {}
#[doc = "Custom instruction data register 1."]
pub mod cinstrdat1;
#[doc = "SPI interface timing.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iftiming](iftiming) module"]
pub type IFTIMING = crate::Reg<u32, _IFTIMING>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IFTIMING;
#[doc = "`read()` method returns [iftiming::R](iftiming::R) reader structure"]
impl crate::Readable for IFTIMING {}
#[doc = "`write(|w| ..)` method takes [iftiming::W](iftiming::W) writer structure"]
impl crate::Writable for IFTIMING {}
#[doc = "SPI interface timing."]
pub mod iftiming;
