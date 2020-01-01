#[doc = "Reader of register CSNPOL"]
pub type R = crate::R<u32, super::CSNPOL>;
#[doc = "Writer for register CSNPOL"]
pub type W = crate::W<u32, super::CSNPOL>;
#[doc = "Register CSNPOL `reset()`'s with value 0"]
impl crate::ResetValue for super::CSNPOL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Polarity of CSN output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSNPOL_A {
    #[doc = "0: Active low (idle state high)"]
    LOW = 0,
    #[doc = "1: Active high (idle state low)"]
    HIGH = 1,
}
impl From<CSNPOL_A> for bool {
    #[inline(always)]
    fn from(variant: CSNPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CSNPOL`"]
pub type CSNPOL_R = crate::R<bool, CSNPOL_A>;
impl CSNPOL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSNPOL_A {
        match self.bits {
            false => CSNPOL_A::LOW,
            true => CSNPOL_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CSNPOL_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CSNPOL_A::HIGH
    }
}
#[doc = "Write proxy for field `CSNPOL`"]
pub struct CSNPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> CSNPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSNPOL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Active low (idle state high)"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(CSNPOL_A::LOW)
    }
    #[doc = "Active high (idle state low)"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CSNPOL_A::HIGH)
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
    #[doc = "Bit 0 - Polarity of CSN output"]
    #[inline(always)]
    pub fn csnpol(&self) -> CSNPOL_R {
        CSNPOL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Polarity of CSN output"]
    #[inline(always)]
    pub fn csnpol(&mut self) -> CSNPOL_W {
        CSNPOL_W { w: self }
    }
}
