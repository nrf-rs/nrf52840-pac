#[doc = "Reader of register RXDELAY"]
pub type R = crate::R<u32, super::RXDELAY>;
#[doc = "Writer for register RXDELAY"]
pub type W = crate::W<u32, super::RXDELAY>;
#[doc = "Register RXDELAY `reset()`'s with value 0x02"]
impl crate::ResetValue for super::RXDELAY {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x02
    }
}
#[doc = "Reader of field `RXDELAY`"]
pub type RXDELAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RXDELAY`"]
pub struct RXDELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> RXDELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Sample delay for input serial data on MISO. The value specifies the number of 64 MHz clock cycles (15.625 ns) delay from the the sampling edge of SCK (leading edge for CONFIG.CPHA = 0, trailing edge for CONFIG.CPHA = 1) until the input serial data is sampled. As en example, if RXDELAY = 0 and CONFIG.CPHA = 0, the input serial data is sampled on the rising edge of SCK."]
    #[inline(always)]
    pub fn rxdelay(&self) -> RXDELAY_R {
        RXDELAY_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Sample delay for input serial data on MISO. The value specifies the number of 64 MHz clock cycles (15.625 ns) delay from the the sampling edge of SCK (leading edge for CONFIG.CPHA = 0, trailing edge for CONFIG.CPHA = 1) until the input serial data is sampled. As en example, if RXDELAY = 0 and CONFIG.CPHA = 0, the input serial data is sampled on the rising edge of SCK."]
    #[inline(always)]
    pub fn rxdelay(&mut self) -> RXDELAY_W {
        RXDELAY_W { w: self }
    }
}
