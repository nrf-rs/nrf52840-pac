#[doc = "Reader of register HYST"]
pub type R = crate::R<u32, super::HYST>;
#[doc = "Writer for register HYST"]
pub type W = crate::W<u32, super::HYST>;
#[doc = "Register HYST `reset()`'s with value 0"]
impl crate::ResetValue for super::HYST {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Comparator hysteresis enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HYST_A {
    #[doc = "0: Comparator hysteresis disabled"]
    DISABLED = 0,
    #[doc = "1: Comparator hysteresis enabled"]
    ENABLED = 1,
}
impl From<HYST_A> for bool {
    #[inline(always)]
    fn from(variant: HYST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HYST`"]
pub type HYST_R = crate::R<bool, HYST_A>;
impl HYST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HYST_A {
        match self.bits {
            false => HYST_A::DISABLED,
            true => HYST_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HYST_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HYST_A::ENABLED
    }
}
#[doc = "Write proxy for field `HYST`"]
pub struct HYST_W<'a> {
    w: &'a mut W,
}
impl<'a> HYST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HYST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Comparator hysteresis disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HYST_A::DISABLED)
    }
    #[doc = "Comparator hysteresis enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HYST_A::ENABLED)
    }
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
    #[doc = "Bit 0 - Comparator hysteresis enable"]
    #[inline(always)]
    pub fn hyst(&self) -> HYST_R {
        HYST_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator hysteresis enable"]
    #[inline(always)]
    pub fn hyst(&mut self) -> HYST_W {
        HYST_W { w: self }
    }
}
