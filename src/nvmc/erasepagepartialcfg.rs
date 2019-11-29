#[doc = "Reader of register ERASEPAGEPARTIALCFG"]
pub type R = crate::R<u32, super::ERASEPAGEPARTIALCFG>;
#[doc = "Writer for register ERASEPAGEPARTIALCFG"]
pub type W = crate::W<u32, super::ERASEPAGEPARTIALCFG>;
#[doc = "Register ERASEPAGEPARTIALCFG `reset()`'s with value 0x0a"]
impl crate::ResetValue for super::ERASEPAGEPARTIALCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0a
    }
}
#[doc = "Reader of field `DURATION`"]
pub type DURATION_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DURATION`"]
pub struct DURATION_W<'a> {
    w: &'a mut W,
}
impl<'a> DURATION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Duration of the partial erase in milliseconds"]
    #[inline(always)]
    pub fn duration(&self) -> DURATION_R {
        DURATION_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Duration of the partial erase in milliseconds"]
    #[inline(always)]
    pub fn duration(&mut self) -> DURATION_W {
        DURATION_W { w: self }
    }
}
