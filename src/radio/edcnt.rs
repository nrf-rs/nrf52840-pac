#[doc = "Reader of register EDCNT"]
pub type R = crate::R<u32, super::EDCNT>;
#[doc = "Writer for register EDCNT"]
pub type W = crate::W<u32, super::EDCNT>;
#[doc = "Register EDCNT `reset()`'s with value 0"]
impl crate::ResetValue for super::EDCNT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EDCNT`"]
pub type EDCNT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `EDCNT`"]
pub struct EDCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> EDCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x001f_ffff) | ((value as u32) & 0x001f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:20 - IEEE 802.15.4 energy detect loop count"]
    #[inline(always)]
    pub fn edcnt(&self) -> EDCNT_R {
        EDCNT_R::new((self.bits & 0x001f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:20 - IEEE 802.15.4 energy detect loop count"]
    #[inline(always)]
    pub fn edcnt(&mut self) -> EDCNT_W {
        EDCNT_W { w: self }
    }
}
