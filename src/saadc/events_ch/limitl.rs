#[doc = "Reader of register LIMITL"]
pub type R = crate::R<u32, super::LIMITL>;
#[doc = "Writer for register LIMITL"]
pub type W = crate::W<u32, super::LIMITL>;
#[doc = "Register LIMITL `reset()`'s with value 0"]
impl crate::ResetValue for super::LIMITL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LIMITL`"]
pub type LIMITL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LIMITL`"]
pub struct LIMITL_W<'a> {
    w: &'a mut W,
}
impl<'a> LIMITL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn limitl(&self) -> LIMITL_R {
        LIMITL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn limitl(&mut self) -> LIMITL_W {
        LIMITL_W { w: self }
    }
}
