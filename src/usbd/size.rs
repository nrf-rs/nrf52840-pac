#[doc = "Description collection\\[n\\]: Number of bytes received last in the data stage of this OUT endpoint\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epout](epout) module"]
pub type EPOUT = crate::Reg<u32, _EPOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EPOUT;
#[doc = "`read()` method returns [epout::R](epout::R) reader structure"]
impl crate::Readable for EPOUT {}
#[doc = "`write(|w| ..)` method takes [epout::W](epout::W) writer structure"]
impl crate::Writable for EPOUT {}
#[doc = "Description collection\\[n\\]: Number of bytes received last in the data stage of this OUT endpoint"]
pub mod epout;
#[doc = "Number of bytes received last on this ISO OUT data endpoint\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isoout](isoout) module"]
pub type ISOOUT = crate::Reg<u32, _ISOOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISOOUT;
#[doc = "`read()` method returns [isoout::R](isoout::R) reader structure"]
impl crate::Readable for ISOOUT {}
#[doc = "Number of bytes received last on this ISO OUT data endpoint"]
pub mod isoout;
