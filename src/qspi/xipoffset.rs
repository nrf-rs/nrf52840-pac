#[doc = "Reader of register XIPOFFSET"]
pub type R = crate::R<u32, super::XIPOFFSET>;
#[doc = "Writer for register XIPOFFSET"]
pub type W = crate::W<u32, super::XIPOFFSET>;
#[doc = "Register XIPOFFSET `reset()`'s with value 0"]
impl crate::ResetValue for super::XIPOFFSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `XIPOFFSET`"]
pub type XIPOFFSET_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `XIPOFFSET`"]
pub struct XIPOFFSET_W<'a> {
    w: &'a mut W,
}
impl<'a> XIPOFFSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Address offset into the external memory for Execute in Place operation. Value must be a multiple of 4."]
    #[inline(always)]
    pub fn xipoffset(&self) -> XIPOFFSET_R {
        XIPOFFSET_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Address offset into the external memory for Execute in Place operation. Value must be a multiple of 4."]
    #[inline(always)]
    pub fn xipoffset(&mut self) -> XIPOFFSET_W {
        XIPOFFSET_W { w: self }
    }
}
