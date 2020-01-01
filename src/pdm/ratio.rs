#[doc = "Reader of register RATIO"]
pub type R = crate::R<u32, super::RATIO>;
#[doc = "Writer for register RATIO"]
pub type W = crate::W<u32, super::RATIO>;
#[doc = "Register RATIO `reset()`'s with value 0"]
impl crate::ResetValue for super::RATIO {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Selects the ratio between PDM_CLK and output sample rate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RATIO_A {
    #[doc = "0: Ratio of 64"]
    RATIO64 = 0,
    #[doc = "1: Ratio of 80"]
    RATIO80 = 1,
}
impl From<RATIO_A> for bool {
    #[inline(always)]
    fn from(variant: RATIO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RATIO`"]
pub type RATIO_R = crate::R<bool, RATIO_A>;
impl RATIO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RATIO_A {
        match self.bits {
            false => RATIO_A::RATIO64,
            true => RATIO_A::RATIO80,
        }
    }
    #[doc = "Checks if the value of the field is `RATIO64`"]
    #[inline(always)]
    pub fn is_ratio64(&self) -> bool {
        *self == RATIO_A::RATIO64
    }
    #[doc = "Checks if the value of the field is `RATIO80`"]
    #[inline(always)]
    pub fn is_ratio80(&self) -> bool {
        *self == RATIO_A::RATIO80
    }
}
#[doc = "Write proxy for field `RATIO`"]
pub struct RATIO_W<'a> {
    w: &'a mut W,
}
impl<'a> RATIO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RATIO_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Ratio of 64"]
    #[inline(always)]
    pub fn ratio64(self) -> &'a mut W {
        self.variant(RATIO_A::RATIO64)
    }
    #[doc = "Ratio of 80"]
    #[inline(always)]
    pub fn ratio80(self) -> &'a mut W {
        self.variant(RATIO_A::RATIO80)
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
    #[doc = "Bit 0 - Selects the ratio between PDM_CLK and output sample rate"]
    #[inline(always)]
    pub fn ratio(&self) -> RATIO_R {
        RATIO_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selects the ratio between PDM_CLK and output sample rate"]
    #[inline(always)]
    pub fn ratio(&mut self) -> RATIO_W {
        RATIO_W { w: self }
    }
}
