#[doc = "Description cluster\\[n\\]: Enable channel group n\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [en](en) module"]
pub type EN = crate::Reg<u32, _EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EN;
#[doc = "`write(|w| ..)` method takes [en::W](en::W) writer structure"]
impl crate::Writable for EN {}
#[doc = "Description cluster\\[n\\]: Enable channel group n"]
pub mod en;
#[doc = "Description cluster\\[n\\]: Disable channel group n\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dis](dis) module"]
pub type DIS = crate::Reg<u32, _DIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIS;
#[doc = "`write(|w| ..)` method takes [dis::W](dis::W) writer structure"]
impl crate::Writable for DIS {}
#[doc = "Description cluster\\[n\\]: Disable channel group n"]
pub mod dis;
