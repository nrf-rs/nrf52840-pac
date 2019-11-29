#[doc = "Reader of register SFD"]
pub type R = crate::R<u32, super::SFD>;
#[doc = "Writer for register SFD"]
pub type W = crate::W<u32, super::SFD>;
#[doc = "Register SFD `reset()`'s with value 0xa7"]
impl crate::ResetValue for super::SFD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xa7
    }
}
#[doc = "Reader of field `SFD`"]
pub type SFD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SFD`"]
pub struct SFD_W<'a> {
    w: &'a mut W,
}
impl<'a> SFD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - IEEE 802.15.4 start of frame delimiter"]
    #[inline(always)]
    pub fn sfd(&self) -> SFD_R {
        SFD_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - IEEE 802.15.4 start of frame delimiter"]
    #[inline(always)]
    pub fn sfd(&mut self) -> SFD_W {
        SFD_W { w: self }
    }
}
