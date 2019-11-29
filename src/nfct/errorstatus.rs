#[doc = "Reader of register ERRORSTATUS"]
pub type R = crate::R<u32, super::ERRORSTATUS>;
#[doc = "Writer for register ERRORSTATUS"]
pub type W = crate::W<u32, super::ERRORSTATUS>;
#[doc = "Register ERRORSTATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::ERRORSTATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FRAMEDELAYTIMEOUT`"]
pub type FRAMEDELAYTIMEOUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FRAMEDELAYTIMEOUT`"]
pub struct FRAMEDELAYTIMEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAMEDELAYTIMEOUT_W<'a> {
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
    #[doc = "Bit 0 - No STARTTX task triggered before expiration of the time set in FRAMEDELAYMAX"]
    #[inline(always)]
    pub fn framedelaytimeout(&self) -> FRAMEDELAYTIMEOUT_R {
        FRAMEDELAYTIMEOUT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - No STARTTX task triggered before expiration of the time set in FRAMEDELAYMAX"]
    #[inline(always)]
    pub fn framedelaytimeout(&mut self) -> FRAMEDELAYTIMEOUT_W {
        FRAMEDELAYTIMEOUT_W { w: self }
    }
}
