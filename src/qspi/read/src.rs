#[doc = "Reader of register SRC"]
pub type R = crate::R<u32, super::SRC>;
#[doc = "Writer for register SRC"]
pub type W = crate::W<u32, super::SRC>;
#[doc = "Register SRC `reset()`'s with value 0"]
impl crate::ResetValue for super::SRC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SRC`"]
pub type SRC_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SRC`"]
pub struct SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Word-aligned flash memory source address."]
    #[inline(always)]
    pub fn src(&self) -> SRC_R {
        SRC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Word-aligned flash memory source address."]
    #[inline(always)]
    pub fn src(&mut self) -> SRC_W {
        SRC_W { w: self }
    }
}
