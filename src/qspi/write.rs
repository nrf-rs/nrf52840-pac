#[doc = "Flash destination address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dst](dst) module"]
pub type DST = crate::Reg<u32, _DST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DST;
#[doc = "`read()` method returns [dst::R](dst::R) reader structure"]
impl crate::Readable for DST {}
#[doc = "`write(|w| ..)` method takes [dst::W](dst::W) writer structure"]
impl crate::Writable for DST {}
#[doc = "Flash destination address"]
pub mod dst;
#[doc = "RAM source address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [src](src) module"]
pub type SRC = crate::Reg<u32, _SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRC;
#[doc = "`read()` method returns [src::R](src::R) reader structure"]
impl crate::Readable for SRC {}
#[doc = "`write(|w| ..)` method takes [src::W](src::W) writer structure"]
impl crate::Writable for SRC {}
#[doc = "RAM source address"]
pub mod src;
#[doc = "Write transfer length\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnt](cnt) module"]
pub type CNT = crate::Reg<u32, _CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNT;
#[doc = "`read()` method returns [cnt::R](cnt::R) reader structure"]
impl crate::Readable for CNT {}
#[doc = "`write(|w| ..)` method takes [cnt::W](cnt::W) writer structure"]
impl crate::Writable for CNT {}
#[doc = "Write transfer length"]
pub mod cnt;
