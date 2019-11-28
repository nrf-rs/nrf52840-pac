#[doc = "Reader of register CINSTRDAT1"]
pub type R = crate::R<u32, super::CINSTRDAT1>;
#[doc = "Writer for register CINSTRDAT1"]
pub type W = crate::W<u32, super::CINSTRDAT1>;
#[doc = "Register CINSTRDAT1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CINSTRDAT1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BYTE4`"]
pub type BYTE4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BYTE4`"]
pub struct BYTE4_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTE4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `BYTE5`"]
pub type BYTE5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BYTE5`"]
pub struct BYTE5_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTE5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `BYTE6`"]
pub type BYTE6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BYTE6`"]
pub struct BYTE6_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTE6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `BYTE7`"]
pub type BYTE7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BYTE7`"]
pub struct BYTE7_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTE7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Data byte 4"]
    #[inline(always)]
    pub fn byte4(&self) -> BYTE4_R {
        BYTE4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Data byte 5"]
    #[inline(always)]
    pub fn byte5(&self) -> BYTE5_R {
        BYTE5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Data byte 6"]
    #[inline(always)]
    pub fn byte6(&self) -> BYTE6_R {
        BYTE6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Data byte 7"]
    #[inline(always)]
    pub fn byte7(&self) -> BYTE7_R {
        BYTE7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data byte 4"]
    #[inline(always)]
    pub fn byte4(&mut self) -> BYTE4_W {
        BYTE4_W { w: self }
    }
    #[doc = "Bits 8:15 - Data byte 5"]
    #[inline(always)]
    pub fn byte5(&mut self) -> BYTE5_W {
        BYTE5_W { w: self }
    }
    #[doc = "Bits 16:23 - Data byte 6"]
    #[inline(always)]
    pub fn byte6(&mut self) -> BYTE6_W {
        BYTE6_W { w: self }
    }
    #[doc = "Bits 24:31 - Data byte 7"]
    #[inline(always)]
    pub fn byte7(&mut self) -> BYTE7_W {
        BYTE7_W { w: self }
    }
}
