#[doc = "Pin select for serial clock SCK\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sck](sck) module"]
pub type SCK = crate::Reg<u32, _SCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCK;
#[doc = "`read()` method returns [sck::R](sck::R) reader structure"]
impl crate::Readable for SCK {}
#[doc = "`write(|w| ..)` method takes [sck::W](sck::W) writer structure"]
impl crate::Writable for SCK {}
#[doc = "Pin select for serial clock SCK"]
pub mod sck;
#[doc = "Pin select for chip select signal CSN.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csn](csn) module"]
pub type CSN = crate::Reg<u32, _CSN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSN;
#[doc = "`read()` method returns [csn::R](csn::R) reader structure"]
impl crate::Readable for CSN {}
#[doc = "`write(|w| ..)` method takes [csn::W](csn::W) writer structure"]
impl crate::Writable for CSN {}
#[doc = "Pin select for chip select signal CSN."]
pub mod csn;
#[doc = "Pin select for serial data MOSI/IO0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [io0](io0) module"]
pub type IO0 = crate::Reg<u32, _IO0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IO0;
#[doc = "`read()` method returns [io0::R](io0::R) reader structure"]
impl crate::Readable for IO0 {}
#[doc = "`write(|w| ..)` method takes [io0::W](io0::W) writer structure"]
impl crate::Writable for IO0 {}
#[doc = "Pin select for serial data MOSI/IO0."]
pub mod io0;
#[doc = "Pin select for serial data MISO/IO1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [io1](io1) module"]
pub type IO1 = crate::Reg<u32, _IO1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IO1;
#[doc = "`read()` method returns [io1::R](io1::R) reader structure"]
impl crate::Readable for IO1 {}
#[doc = "`write(|w| ..)` method takes [io1::W](io1::W) writer structure"]
impl crate::Writable for IO1 {}
#[doc = "Pin select for serial data MISO/IO1."]
pub mod io1;
#[doc = "Pin select for serial data IO2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [io2](io2) module"]
pub type IO2 = crate::Reg<u32, _IO2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IO2;
#[doc = "`read()` method returns [io2::R](io2::R) reader structure"]
impl crate::Readable for IO2 {}
#[doc = "`write(|w| ..)` method takes [io2::W](io2::W) writer structure"]
impl crate::Writable for IO2 {}
#[doc = "Pin select for serial data IO2."]
pub mod io2;
#[doc = "Pin select for serial data IO3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [io3](io3) module"]
pub type IO3 = crate::Reg<u32, _IO3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IO3;
#[doc = "`read()` method returns [io3::R](io3::R) reader structure"]
impl crate::Readable for IO3 {}
#[doc = "`write(|w| ..)` method takes [io3::W](io3::W) writer structure"]
impl crate::Writable for IO3 {}
#[doc = "Pin select for serial data IO3."]
pub mod io3;
