#[doc = "Reader of register DCXCNT"]
pub type R = crate::R<u32, super::DCXCNT>;
#[doc = "Writer for register DCXCNT"]
pub type W = crate::W<u32, super::DCXCNT>;
#[doc = "Register DCXCNT `reset()`'s with value 0"]
impl crate::ResetValue for super::DCXCNT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DCXCNT`"]
pub type DCXCNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DCXCNT`"]
pub struct DCXCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> DCXCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - This register specifies the number of command bytes preceding the data bytes. The PSEL.DCX line will be low during transmission of command bytes and high during transmission of data bytes. Value 0xF indicates that all bytes are command bytes."]
    #[inline(always)]
    pub fn dcxcnt(&self) -> DCXCNT_R {
        DCXCNT_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - This register specifies the number of command bytes preceding the data bytes. The PSEL.DCX line will be low during transmission of command bytes and high during transmission of data bytes. Value 0xF indicates that all bytes are command bytes."]
    #[inline(always)]
    pub fn dcxcnt(&mut self) -> DCXCNT_W {
        DCXCNT_W { w: self }
    }
}
