#[doc = "Reader of register MAXPACKETSIZE"]
pub type R = crate::R<u32, super::MAXPACKETSIZE>;
#[doc = "Writer for register MAXPACKETSIZE"]
pub type W = crate::W<u32, super::MAXPACKETSIZE>;
#[doc = "Register MAXPACKETSIZE `reset()`'s with value 0xfb"]
impl crate::ResetValue for super::MAXPACKETSIZE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xfb
    }
}
#[doc = "Reader of field `MAXPACKETSIZE`"]
pub type MAXPACKETSIZE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MAXPACKETSIZE`"]
pub struct MAXPACKETSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> MAXPACKETSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Length of key-stream generated when MODE.LENGTH = Extended. This value must be greater or equal to the subsequent packet payload to be encrypted/decrypted."]
    #[inline(always)]
    pub fn maxpacketsize(&self) -> MAXPACKETSIZE_R {
        MAXPACKETSIZE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Length of key-stream generated when MODE.LENGTH = Extended. This value must be greater or equal to the subsequent packet payload to be encrypted/decrypted."]
    #[inline(always)]
    pub fn maxpacketsize(&mut self) -> MAXPACKETSIZE_W {
        MAXPACKETSIZE_W { w: self }
    }
}
