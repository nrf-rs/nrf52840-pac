#[doc = "Reader of register EDSAMPLE"]
pub type R = crate::R<u32, super::EDSAMPLE>;
#[doc = "Writer for register EDSAMPLE"]
pub type W = crate::W<u32, super::EDSAMPLE>;
#[doc = "Register EDSAMPLE `reset()`'s with value 0"]
impl crate::ResetValue for super::EDSAMPLE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EDLVL`"]
pub type EDLVL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EDLVL`"]
pub struct EDLVL_W<'a> {
    w: &'a mut W,
}
impl<'a> EDLVL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - IEEE 802.15.4 energy detect level"]
    #[inline(always)]
    pub fn edlvl(&self) -> EDLVL_R {
        EDLVL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - IEEE 802.15.4 energy detect level"]
    #[inline(always)]
    pub fn edlvl(&mut self) -> EDLVL_W {
        EDLVL_W { w: self }
    }
}
