#[doc = "Reader of register EVENTS_USBDETECTED"]
pub type R = crate::R<u32, super::EVENTS_USBDETECTED>;
#[doc = "Writer for register EVENTS_USBDETECTED"]
pub type W = crate::W<u32, super::EVENTS_USBDETECTED>;
#[doc = "Register EVENTS_USBDETECTED `reset()`'s with value 0"]
impl crate::ResetValue for super::EVENTS_USBDETECTED {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EVENTS_USBDETECTED`"]
pub type EVENTS_USBDETECTED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EVENTS_USBDETECTED`"]
pub struct EVENTS_USBDETECTED_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENTS_USBDETECTED_W<'a> {
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
    pub fn events_usbdetected(&self) -> EVENTS_USBDETECTED_R {
        EVENTS_USBDETECTED_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn events_usbdetected(&mut self) -> EVENTS_USBDETECTED_W {
        EVENTS_USBDETECTED_W { w: self }
    }
}
