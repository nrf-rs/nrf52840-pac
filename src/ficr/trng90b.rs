#[doc = "Amount of bytes for the required entropy bits\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bytes](bytes) module"]
pub type BYTES = crate::Reg<u32, _BYTES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BYTES;
#[doc = "`read()` method returns [bytes::R](bytes::R) reader structure"]
impl crate::Readable for BYTES {}
#[doc = "Amount of bytes for the required entropy bits"]
pub mod bytes;
#[doc = "Repetition counter cutoff\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rccutoff](rccutoff) module"]
pub type RCCUTOFF = crate::Reg<u32, _RCCUTOFF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCCUTOFF;
#[doc = "`read()` method returns [rccutoff::R](rccutoff::R) reader structure"]
impl crate::Readable for RCCUTOFF {}
#[doc = "Repetition counter cutoff"]
pub mod rccutoff;
#[doc = "Adaptive proportion cutoff\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apcutoff](apcutoff) module"]
pub type APCUTOFF = crate::Reg<u32, _APCUTOFF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APCUTOFF;
#[doc = "`read()` method returns [apcutoff::R](apcutoff::R) reader structure"]
impl crate::Readable for APCUTOFF {}
#[doc = "Adaptive proportion cutoff"]
pub mod apcutoff;
#[doc = "Amount of bytes for the startup tests\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [startup](startup) module"]
pub type STARTUP = crate::Reg<u32, _STARTUP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STARTUP;
#[doc = "`read()` method returns [startup::R](startup::R) reader structure"]
impl crate::Readable for STARTUP {}
#[doc = "Amount of bytes for the startup tests"]
pub mod startup;
#[doc = "Sample count for ring oscillator 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rosc1](rosc1) module"]
pub type ROSC1 = crate::Reg<u32, _ROSC1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ROSC1;
#[doc = "`read()` method returns [rosc1::R](rosc1::R) reader structure"]
impl crate::Readable for ROSC1 {}
#[doc = "Sample count for ring oscillator 1"]
pub mod rosc1;
#[doc = "Sample count for ring oscillator 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rosc2](rosc2) module"]
pub type ROSC2 = crate::Reg<u32, _ROSC2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ROSC2;
#[doc = "`read()` method returns [rosc2::R](rosc2::R) reader structure"]
impl crate::Readable for ROSC2 {}
#[doc = "Sample count for ring oscillator 2"]
pub mod rosc2;
#[doc = "Sample count for ring oscillator 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rosc3](rosc3) module"]
pub type ROSC3 = crate::Reg<u32, _ROSC3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ROSC3;
#[doc = "`read()` method returns [rosc3::R](rosc3::R) reader structure"]
impl crate::Readable for ROSC3 {}
#[doc = "Sample count for ring oscillator 3"]
pub mod rosc3;
#[doc = "Sample count for ring oscillator 4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rosc4](rosc4) module"]
pub type ROSC4 = crate::Reg<u32, _ROSC4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ROSC4;
#[doc = "`read()` method returns [rosc4::R](rosc4::R) reader structure"]
impl crate::Readable for ROSC4 {}
#[doc = "Sample count for ring oscillator 4"]
pub mod rosc4;
