#[doc = "Reader of register IFTIMING"]
pub type R = crate::R<u32, super::IFTIMING>;
#[doc = "Writer for register IFTIMING"]
pub type W = crate::W<u32, super::IFTIMING>;
#[doc = "Register IFTIMING `reset()`'s with value 0x0200"]
impl crate::ResetValue for super::IFTIMING {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0200
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
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:10 - Timing related to sampling of the input serial data. The value of RXDELAY specifies the number of 64 MHz cycles (15.625 ns) delay from the the rising edge of the SPI Clock (SCK) until the input serial data is sampled. As en example, if set to 0 the input serial data is sampled on the rising edge of SCK."]
    #[inline(always)]
    pub fn rxdelay(&self) -> RXDELAY_R {
        RXDELAY_R::new(((self.bits >> 8) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 8:10 - Timing related to sampling of the input serial data. The value of RXDELAY specifies the number of 64 MHz cycles (15.625 ns) delay from the the rising edge of the SPI Clock (SCK) until the input serial data is sampled. As en example, if set to 0 the input serial data is sampled on the rising edge of SCK."]
    #[inline(always)]
    pub fn rxdelay(&mut self) -> RXDELAY_W {
        RXDELAY_W { w: self }
    }
}
