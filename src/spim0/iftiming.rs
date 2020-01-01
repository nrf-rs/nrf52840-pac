#[doc = "Sample delay for input serial data on MISO\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxdelay](rxdelay) module"]
pub type RXDELAY = crate::Reg<u32, _RXDELAY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXDELAY;
#[doc = "`read()` method returns [rxdelay::R](rxdelay::R) reader structure"]
impl crate::Readable for RXDELAY {}
#[doc = "`write(|w| ..)` method takes [rxdelay::W](rxdelay::W) writer structure"]
impl crate::Writable for RXDELAY {}
#[doc = "Sample delay for input serial data on MISO"]
pub mod rxdelay;
#[doc = "Minimum duration between edge of CSN and edge of SCK and minimum duration CSN must stay high between transactions\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csndur](csndur) module"]
pub type CSNDUR = crate::Reg<u32, _CSNDUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSNDUR;
#[doc = "`read()` method returns [csndur::R](csndur::R) reader structure"]
impl crate::Readable for CSNDUR {}
#[doc = "`write(|w| ..)` method takes [csndur::W](csndur::W) writer structure"]
impl crate::Writable for CSNDUR {}
#[doc = "Minimum duration between edge of CSN and edge of SCK and minimum duration CSN must stay high between transactions"]
pub mod csndur;
