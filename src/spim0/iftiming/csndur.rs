#[doc = "Reader of register CSNDUR"]
pub type R = crate::R<u32, super::CSNDUR>;
#[doc = "Writer for register CSNDUR"]
pub type W = crate::W<u32, super::CSNDUR>;
#[doc = "Register CSNDUR `reset()`'s with value 0x02"]
impl crate::ResetValue for super::CSNDUR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x02
    }
}
#[doc = "Reader of field `CSNDUR`"]
pub type CSNDUR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CSNDUR`"]
pub struct CSNDUR_W<'a> {
    w: &'a mut W,
}
impl<'a> CSNDUR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Minimum duration between edge of CSN and edge of SCK and minimum duration CSN must stay high between transactions. The value is specified in number of 64 MHz clock cycles (15.625 ns)."]
    #[inline(always)]
    pub fn csndur(&self) -> CSNDUR_R {
        CSNDUR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Minimum duration between edge of CSN and edge of SCK and minimum duration CSN must stay high between transactions. The value is specified in number of 64 MHz clock cycles (15.625 ns)."]
    #[inline(always)]
    pub fn csndur(&mut self) -> CSNDUR_W {
        CSNDUR_W { w: self }
    }
}
