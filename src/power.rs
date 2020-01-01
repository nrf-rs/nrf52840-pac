#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 120usize],
    #[doc = "0x78 - Enable constant latency mode"]
    pub tasks_constlat: TASKS_CONSTLAT,
    #[doc = "0x7c - Enable low power mode (variable latency)"]
    pub tasks_lowpwr: TASKS_LOWPWR,
    _reserved2: [u8; 136usize],
    #[doc = "0x108 - Power failure warning"]
    pub events_pofwarn: EVENTS_POFWARN,
    _reserved3: [u8; 8usize],
    #[doc = "0x114 - CPU entered WFI/WFE sleep"]
    pub events_sleepenter: EVENTS_SLEEPENTER,
    #[doc = "0x118 - CPU exited WFI/WFE sleep"]
    pub events_sleepexit: EVENTS_SLEEPEXIT,
    #[doc = "0x11c - Voltage supply detected on VBUS"]
    pub events_usbdetected: EVENTS_USBDETECTED,
    #[doc = "0x120 - Voltage supply removed from VBUS"]
    pub events_usbremoved: EVENTS_USBREMOVED,
    #[doc = "0x124 - USB 3.3 V supply ready"]
    pub events_usbpwrrdy: EVENTS_USBPWRRDY,
    _reserved8: [u8; 476usize],
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved10: [u8; 244usize],
    #[doc = "0x400 - Reset reason"]
    pub resetreas: RESETREAS,
    _reserved11: [u8; 36usize],
    #[doc = "0x428 - Deprecated register - RAM status register"]
    pub ramstatus: RAMSTATUS,
    _reserved12: [u8; 12usize],
    #[doc = "0x438 - USB supply status"]
    pub usbregstatus: USBREGSTATUS,
    _reserved13: [u8; 196usize],
    #[doc = "0x500 - System OFF register"]
    pub systemoff: SYSTEMOFF,
    _reserved14: [u8; 12usize],
    #[doc = "0x510 - Power-fail comparator configuration"]
    pub pofcon: POFCON,
    _reserved15: [u8; 8usize],
    #[doc = "0x51c - General purpose retention register"]
    pub gpregret: GPREGRET,
    #[doc = "0x520 - General purpose retention register"]
    pub gpregret2: GPREGRET2,
    _reserved17: [u8; 84usize],
    #[doc = "0x578 - Enable DC/DC converter for REG1 stage."]
    pub dcdcen: DCDCEN,
    _reserved18: [u8; 4usize],
    #[doc = "0x580 - Enable DC/DC converter for REG0 stage."]
    pub dcdcen0: DCDCEN0,
    _reserved19: [u8; 188usize],
    #[doc = "0x640 - Main supply status"]
    pub mainregstatus: MAINREGSTATUS,
    _reserved20: [u8; 700usize],
    #[doc = "0x900 - Unspecified"]
    pub ram0: RAM,
    _reserved21: [u8; 4usize],
    #[doc = "0x910 - Unspecified"]
    pub ram1: RAM,
    _reserved22: [u8; 4usize],
    #[doc = "0x920 - Unspecified"]
    pub ram2: RAM,
    _reserved23: [u8; 4usize],
    #[doc = "0x930 - Unspecified"]
    pub ram3: RAM,
    _reserved24: [u8; 4usize],
    #[doc = "0x940 - Unspecified"]
    pub ram4: RAM,
    _reserved25: [u8; 4usize],
    #[doc = "0x950 - Unspecified"]
    pub ram5: RAM,
    _reserved26: [u8; 4usize],
    #[doc = "0x960 - Unspecified"]
    pub ram6: RAM,
    _reserved27: [u8; 4usize],
    #[doc = "0x970 - Unspecified"]
    pub ram7: RAM,
    _reserved28: [u8; 4usize],
    #[doc = "0x980 - Unspecified"]
    pub ram8: RAM,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct RAM {
    #[doc = "0x00 - Description cluster\\[n\\]: RAMn power control register"]
    pub power: self::ram::POWER,
    #[doc = "0x04 - Description cluster\\[n\\]: RAMn power control set register"]
    pub powerset: self::ram::POWERSET,
    #[doc = "0x08 - Description cluster\\[n\\]: RAMn power control clear register"]
    pub powerclr: self::ram::POWERCLR,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod ram;
#[doc = "Enable constant latency mode\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_constlat](tasks_constlat) module"]
pub type TASKS_CONSTLAT = crate::Reg<u32, _TASKS_CONSTLAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_CONSTLAT;
#[doc = "`write(|w| ..)` method takes [tasks_constlat::W](tasks_constlat::W) writer structure"]
impl crate::Writable for TASKS_CONSTLAT {}
#[doc = "Enable constant latency mode"]
pub mod tasks_constlat;
#[doc = "Enable low power mode (variable latency)\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_lowpwr](tasks_lowpwr) module"]
pub type TASKS_LOWPWR = crate::Reg<u32, _TASKS_LOWPWR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_LOWPWR;
#[doc = "`write(|w| ..)` method takes [tasks_lowpwr::W](tasks_lowpwr::W) writer structure"]
impl crate::Writable for TASKS_LOWPWR {}
#[doc = "Enable low power mode (variable latency)"]
pub mod tasks_lowpwr;
#[doc = "Power failure warning\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_pofwarn](events_pofwarn) module"]
pub type EVENTS_POFWARN = crate::Reg<u32, _EVENTS_POFWARN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_POFWARN;
#[doc = "`read()` method returns [events_pofwarn::R](events_pofwarn::R) reader structure"]
impl crate::Readable for EVENTS_POFWARN {}
#[doc = "`write(|w| ..)` method takes [events_pofwarn::W](events_pofwarn::W) writer structure"]
impl crate::Writable for EVENTS_POFWARN {}
#[doc = "Power failure warning"]
pub mod events_pofwarn;
#[doc = "CPU entered WFI/WFE sleep\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_sleepenter](events_sleepenter) module"]
pub type EVENTS_SLEEPENTER = crate::Reg<u32, _EVENTS_SLEEPENTER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_SLEEPENTER;
#[doc = "`read()` method returns [events_sleepenter::R](events_sleepenter::R) reader structure"]
impl crate::Readable for EVENTS_SLEEPENTER {}
#[doc = "`write(|w| ..)` method takes [events_sleepenter::W](events_sleepenter::W) writer structure"]
impl crate::Writable for EVENTS_SLEEPENTER {}
#[doc = "CPU entered WFI/WFE sleep"]
pub mod events_sleepenter;
#[doc = "CPU exited WFI/WFE sleep\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_sleepexit](events_sleepexit) module"]
pub type EVENTS_SLEEPEXIT = crate::Reg<u32, _EVENTS_SLEEPEXIT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_SLEEPEXIT;
#[doc = "`read()` method returns [events_sleepexit::R](events_sleepexit::R) reader structure"]
impl crate::Readable for EVENTS_SLEEPEXIT {}
#[doc = "`write(|w| ..)` method takes [events_sleepexit::W](events_sleepexit::W) writer structure"]
impl crate::Writable for EVENTS_SLEEPEXIT {}
#[doc = "CPU exited WFI/WFE sleep"]
pub mod events_sleepexit;
#[doc = "Voltage supply detected on VBUS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_usbdetected](events_usbdetected) module"]
pub type EVENTS_USBDETECTED = crate::Reg<u32, _EVENTS_USBDETECTED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_USBDETECTED;
#[doc = "`read()` method returns [events_usbdetected::R](events_usbdetected::R) reader structure"]
impl crate::Readable for EVENTS_USBDETECTED {}
#[doc = "`write(|w| ..)` method takes [events_usbdetected::W](events_usbdetected::W) writer structure"]
impl crate::Writable for EVENTS_USBDETECTED {}
#[doc = "Voltage supply detected on VBUS"]
pub mod events_usbdetected;
#[doc = "Voltage supply removed from VBUS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_usbremoved](events_usbremoved) module"]
pub type EVENTS_USBREMOVED = crate::Reg<u32, _EVENTS_USBREMOVED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_USBREMOVED;
#[doc = "`read()` method returns [events_usbremoved::R](events_usbremoved::R) reader structure"]
impl crate::Readable for EVENTS_USBREMOVED {}
#[doc = "`write(|w| ..)` method takes [events_usbremoved::W](events_usbremoved::W) writer structure"]
impl crate::Writable for EVENTS_USBREMOVED {}
#[doc = "Voltage supply removed from VBUS"]
pub mod events_usbremoved;
#[doc = "USB 3.3 V supply ready\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_usbpwrrdy](events_usbpwrrdy) module"]
pub type EVENTS_USBPWRRDY = crate::Reg<u32, _EVENTS_USBPWRRDY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_USBPWRRDY;
#[doc = "`read()` method returns [events_usbpwrrdy::R](events_usbpwrrdy::R) reader structure"]
impl crate::Readable for EVENTS_USBPWRRDY {}
#[doc = "`write(|w| ..)` method takes [events_usbpwrrdy::W](events_usbpwrrdy::W) writer structure"]
impl crate::Writable for EVENTS_USBPWRRDY {}
#[doc = "USB 3.3 V supply ready"]
pub mod events_usbpwrrdy;
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
#[doc = "Reset reason\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resetreas](resetreas) module"]
pub type RESETREAS = crate::Reg<u32, _RESETREAS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESETREAS;
#[doc = "`read()` method returns [resetreas::R](resetreas::R) reader structure"]
impl crate::Readable for RESETREAS {}
#[doc = "`write(|w| ..)` method takes [resetreas::W](resetreas::W) writer structure"]
impl crate::Writable for RESETREAS {}
#[doc = "Reset reason"]
pub mod resetreas;
#[doc = "Deprecated register - RAM status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ramstatus](ramstatus) module"]
pub type RAMSTATUS = crate::Reg<u32, _RAMSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RAMSTATUS;
#[doc = "`read()` method returns [ramstatus::R](ramstatus::R) reader structure"]
impl crate::Readable for RAMSTATUS {}
#[doc = "Deprecated register - RAM status register"]
pub mod ramstatus;
#[doc = "USB supply status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbregstatus](usbregstatus) module"]
pub type USBREGSTATUS = crate::Reg<u32, _USBREGSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBREGSTATUS;
#[doc = "`read()` method returns [usbregstatus::R](usbregstatus::R) reader structure"]
impl crate::Readable for USBREGSTATUS {}
#[doc = "USB supply status"]
pub mod usbregstatus;
#[doc = "System OFF register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [systemoff](systemoff) module"]
pub type SYSTEMOFF = crate::Reg<u32, _SYSTEMOFF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSTEMOFF;
#[doc = "`write(|w| ..)` method takes [systemoff::W](systemoff::W) writer structure"]
impl crate::Writable for SYSTEMOFF {}
#[doc = "System OFF register"]
pub mod systemoff;
#[doc = "Power-fail comparator configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pofcon](pofcon) module"]
pub type POFCON = crate::Reg<u32, _POFCON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _POFCON;
#[doc = "`read()` method returns [pofcon::R](pofcon::R) reader structure"]
impl crate::Readable for POFCON {}
#[doc = "`write(|w| ..)` method takes [pofcon::W](pofcon::W) writer structure"]
impl crate::Writable for POFCON {}
#[doc = "Power-fail comparator configuration"]
pub mod pofcon;
#[doc = "General purpose retention register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpregret](gpregret) module"]
pub type GPREGRET = crate::Reg<u32, _GPREGRET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPREGRET;
#[doc = "`read()` method returns [gpregret::R](gpregret::R) reader structure"]
impl crate::Readable for GPREGRET {}
#[doc = "`write(|w| ..)` method takes [gpregret::W](gpregret::W) writer structure"]
impl crate::Writable for GPREGRET {}
#[doc = "General purpose retention register"]
pub mod gpregret;
#[doc = "General purpose retention register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpregret2](gpregret2) module"]
pub type GPREGRET2 = crate::Reg<u32, _GPREGRET2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPREGRET2;
#[doc = "`read()` method returns [gpregret2::R](gpregret2::R) reader structure"]
impl crate::Readable for GPREGRET2 {}
#[doc = "`write(|w| ..)` method takes [gpregret2::W](gpregret2::W) writer structure"]
impl crate::Writable for GPREGRET2 {}
#[doc = "General purpose retention register"]
pub mod gpregret2;
#[doc = "Enable DC/DC converter for REG1 stage.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcdcen](dcdcen) module"]
pub type DCDCEN = crate::Reg<u32, _DCDCEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCDCEN;
#[doc = "`read()` method returns [dcdcen::R](dcdcen::R) reader structure"]
impl crate::Readable for DCDCEN {}
#[doc = "`write(|w| ..)` method takes [dcdcen::W](dcdcen::W) writer structure"]
impl crate::Writable for DCDCEN {}
#[doc = "Enable DC/DC converter for REG1 stage."]
pub mod dcdcen;
#[doc = "Enable DC/DC converter for REG0 stage.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcdcen0](dcdcen0) module"]
pub type DCDCEN0 = crate::Reg<u32, _DCDCEN0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCDCEN0;
#[doc = "`read()` method returns [dcdcen0::R](dcdcen0::R) reader structure"]
impl crate::Readable for DCDCEN0 {}
#[doc = "`write(|w| ..)` method takes [dcdcen0::W](dcdcen0::W) writer structure"]
impl crate::Writable for DCDCEN0 {}
#[doc = "Enable DC/DC converter for REG0 stage."]
pub mod dcdcen0;
#[doc = "Main supply status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mainregstatus](mainregstatus) module"]
pub type MAINREGSTATUS = crate::Reg<u32, _MAINREGSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAINREGSTATUS;
#[doc = "`read()` method returns [mainregstatus::R](mainregstatus::R) reader structure"]
impl crate::Readable for MAINREGSTATUS {}
#[doc = "Main supply status"]
pub mod mainregstatus;
