#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 6712usize],
    #[doc = "0x1a38 - AES hardware key select"]
    pub host_cryptokey_sel: HOST_CRYPTOKEY_SEL,
    _reserved1: [u8; 16usize],
    #[doc = "0x1a4c - This write-once register is the K_PRTL lock register. When this register is set, K_PRTL can not be used and a zeroed key will be used instead. The value of this register is saved in the CRYPTOCELL AO power domain."]
    pub host_iot_kprtl_lock: HOST_IOT_KPRTL_LOCK,
    #[doc = "0x1a50 - This register holds bits 31:0 of K_DR. The value of this register is saved in the CRYPTOCELL AO power domain. Reading from this address returns the K_DR valid status indicating if K_DR is successfully retained."]
    pub host_iot_kdr0: HOST_IOT_KDR0,
    #[doc = "0x1a54 - This register holds bits 63:32 of K_DR. The value of this register is saved in the CRYPTOCELL AO power domain."]
    pub host_iot_kdr1: HOST_IOT_KDR1,
    #[doc = "0x1a58 - This register holds bits 95:64 of K_DR. The value of this register is saved in the CRYPTOCELL AO power domain."]
    pub host_iot_kdr2: HOST_IOT_KDR2,
    #[doc = "0x1a5c - This register holds bits 127:96 of K_DR. The value of this register is saved in the CRYPTOCELL AO power domain."]
    pub host_iot_kdr3: HOST_IOT_KDR3,
    #[doc = "0x1a60 - Controls lifecycle state (LCS) for CRYPTOCELL subsystem"]
    pub host_iot_lcs: HOST_IOT_LCS,
}
#[doc = "AES hardware key select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_cryptokey_sel](host_cryptokey_sel) module"]
pub type HOST_CRYPTOKEY_SEL = crate::Reg<u32, _HOST_CRYPTOKEY_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_CRYPTOKEY_SEL;
#[doc = "`read()` method returns [host_cryptokey_sel::R](host_cryptokey_sel::R) reader structure"]
impl crate::Readable for HOST_CRYPTOKEY_SEL {}
#[doc = "`write(|w| ..)` method takes [host_cryptokey_sel::W](host_cryptokey_sel::W) writer structure"]
impl crate::Writable for HOST_CRYPTOKEY_SEL {}
#[doc = "AES hardware key select"]
pub mod host_cryptokey_sel;
#[doc = "This write-once register is the K_PRTL lock register. When this register is set, K_PRTL can not be used and a zeroed key will be used instead. The value of this register is saved in the CRYPTOCELL AO power domain.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_iot_kprtl_lock](host_iot_kprtl_lock) module"]
pub type HOST_IOT_KPRTL_LOCK = crate::Reg<u32, _HOST_IOT_KPRTL_LOCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_IOT_KPRTL_LOCK;
#[doc = "`read()` method returns [host_iot_kprtl_lock::R](host_iot_kprtl_lock::R) reader structure"]
impl crate::Readable for HOST_IOT_KPRTL_LOCK {}
#[doc = "`write(|w| ..)` method takes [host_iot_kprtl_lock::W](host_iot_kprtl_lock::W) writer structure"]
impl crate::Writable for HOST_IOT_KPRTL_LOCK {}
#[doc = "This write-once register is the K_PRTL lock register. When this register is set, K_PRTL can not be used and a zeroed key will be used instead. The value of this register is saved in the CRYPTOCELL AO power domain."]
pub mod host_iot_kprtl_lock;
#[doc = "This register holds bits 31:0 of K_DR. The value of this register is saved in the CRYPTOCELL AO power domain. Reading from this address returns the K_DR valid status indicating if K_DR is successfully retained.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_iot_kdr0](host_iot_kdr0) module"]
pub type HOST_IOT_KDR0 = crate::Reg<u32, _HOST_IOT_KDR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_IOT_KDR0;
#[doc = "`read()` method returns [host_iot_kdr0::R](host_iot_kdr0::R) reader structure"]
impl crate::Readable for HOST_IOT_KDR0 {}
#[doc = "`write(|w| ..)` method takes [host_iot_kdr0::W](host_iot_kdr0::W) writer structure"]
impl crate::Writable for HOST_IOT_KDR0 {}
#[doc = "This register holds bits 31:0 of K_DR. The value of this register is saved in the CRYPTOCELL AO power domain. Reading from this address returns the K_DR valid status indicating if K_DR is successfully retained."]
pub mod host_iot_kdr0;
#[doc = "This register holds bits 63:32 of K_DR. The value of this register is saved in the CRYPTOCELL AO power domain.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_iot_kdr1](host_iot_kdr1) module"]
pub type HOST_IOT_KDR1 = crate::Reg<u32, _HOST_IOT_KDR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_IOT_KDR1;
#[doc = "`write(|w| ..)` method takes [host_iot_kdr1::W](host_iot_kdr1::W) writer structure"]
impl crate::Writable for HOST_IOT_KDR1 {}
#[doc = "This register holds bits 63:32 of K_DR. The value of this register is saved in the CRYPTOCELL AO power domain."]
pub mod host_iot_kdr1;
#[doc = "This register holds bits 95:64 of K_DR. The value of this register is saved in the CRYPTOCELL AO power domain.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_iot_kdr2](host_iot_kdr2) module"]
pub type HOST_IOT_KDR2 = crate::Reg<u32, _HOST_IOT_KDR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_IOT_KDR2;
#[doc = "`write(|w| ..)` method takes [host_iot_kdr2::W](host_iot_kdr2::W) writer structure"]
impl crate::Writable for HOST_IOT_KDR2 {}
#[doc = "This register holds bits 95:64 of K_DR. The value of this register is saved in the CRYPTOCELL AO power domain."]
pub mod host_iot_kdr2;
#[doc = "This register holds bits 127:96 of K_DR. The value of this register is saved in the CRYPTOCELL AO power domain.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_iot_kdr3](host_iot_kdr3) module"]
pub type HOST_IOT_KDR3 = crate::Reg<u32, _HOST_IOT_KDR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_IOT_KDR3;
#[doc = "`write(|w| ..)` method takes [host_iot_kdr3::W](host_iot_kdr3::W) writer structure"]
impl crate::Writable for HOST_IOT_KDR3 {}
#[doc = "This register holds bits 127:96 of K_DR. The value of this register is saved in the CRYPTOCELL AO power domain."]
pub mod host_iot_kdr3;
#[doc = "Controls lifecycle state (LCS) for CRYPTOCELL subsystem\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_iot_lcs](host_iot_lcs) module"]
pub type HOST_IOT_LCS = crate::Reg<u32, _HOST_IOT_LCS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_IOT_LCS;
#[doc = "`read()` method returns [host_iot_lcs::R](host_iot_lcs::R) reader structure"]
impl crate::Readable for HOST_IOT_LCS {}
#[doc = "`write(|w| ..)` method takes [host_iot_lcs::W](host_iot_lcs::W) writer structure"]
impl crate::Writable for HOST_IOT_LCS {}
#[doc = "Controls lifecycle state (LCS) for CRYPTOCELL subsystem"]
pub mod host_iot_lcs;
