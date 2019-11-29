#[doc = "Reader of register LIMITH"]
pub type R = crate::R<u32, super::LIMITH>;
#[doc = "Writer for register LIMITH"]
pub type W = crate::W<u32, super::LIMITH>;
#[doc = "Register LIMITH `reset()`'s with value 0"]
impl crate::ResetValue for super::LIMITH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LIMITH`"]
pub type LIMITH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LIMITH`"]
pub struct LIMITH_W<'a> {
    w: &'a mut W,
}
impl<'a> LIMITH_W<'a> {
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
    pub fn limith(&self) -> LIMITH_R {
        LIMITH_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn limith(&mut self) -> LIMITH_W {
        LIMITH_W { w: self }
    }
}
