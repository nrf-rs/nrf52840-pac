#[doc = "Reader of register HOST_IOT_LCS"]
pub type R = crate::R<u32, super::HOST_IOT_LCS>;
#[doc = "Writer for register HOST_IOT_LCS"]
pub type W = crate::W<u32, super::HOST_IOT_LCS>;
#[doc = "Register HOST_IOT_LCS `reset()`'s with value 0x02"]
impl crate::ResetValue for super::HOST_IOT_LCS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x02
    }
}
#[doc = "Lifecycle state value. This field is write-once per reset.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LCS_A {
    #[doc = "0: CC310 operates in debug mode"]
    DEBUG = 0,
    #[doc = "2: CC310 operates in secure mode"]
    SECURE = 2,
}
impl From<LCS_A> for u8 {
    #[inline(always)]
    fn from(variant: LCS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LCS`"]
pub type LCS_R = crate::R<u8, LCS_A>;
impl LCS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LCS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(LCS_A::DEBUG),
            2 => Val(LCS_A::SECURE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DEBUG`"]
    #[inline(always)]
    pub fn is_debug(&self) -> bool {
        *self == LCS_A::DEBUG
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == LCS_A::SECURE
    }
}
#[doc = "Write proxy for field `LCS`"]
pub struct LCS_W<'a> {
    w: &'a mut W,
}
impl<'a> LCS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LCS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "CC310 operates in debug mode"]
    #[inline(always)]
    pub fn debug(self) -> &'a mut W {
        self.variant(LCS_A::DEBUG)
    }
    #[doc = "CC310 operates in secure mode"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(LCS_A::SECURE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "This field is read-only and indicates if CRYPTOCELL LCS has been successfully configured since last reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LCS_IS_VALID_A {
    #[doc = "0: A valid LCS is not yet retained in the CRYPTOCELL AO power domain"]
    INVALID = 0,
    #[doc = "1: A valid LCS is successfully retained in the CRYPTOCELL AO power domain"]
    VALID = 1,
}
impl From<LCS_IS_VALID_A> for bool {
    #[inline(always)]
    fn from(variant: LCS_IS_VALID_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LCS_IS_VALID`"]
pub type LCS_IS_VALID_R = crate::R<bool, LCS_IS_VALID_A>;
impl LCS_IS_VALID_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCS_IS_VALID_A {
        match self.bits {
            false => LCS_IS_VALID_A::INVALID,
            true => LCS_IS_VALID_A::VALID,
        }
    }
    #[doc = "Checks if the value of the field is `INVALID`"]
    #[inline(always)]
    pub fn is_invalid(&self) -> bool {
        *self == LCS_IS_VALID_A::INVALID
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == LCS_IS_VALID_A::VALID
    }
}
#[doc = "Write proxy for field `LCS_IS_VALID`"]
pub struct LCS_IS_VALID_W<'a> {
    w: &'a mut W,
}
impl<'a> LCS_IS_VALID_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LCS_IS_VALID_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "A valid LCS is not yet retained in the CRYPTOCELL AO power domain"]
    #[inline(always)]
    pub fn invalid(self) -> &'a mut W {
        self.variant(LCS_IS_VALID_A::INVALID)
    }
    #[doc = "A valid LCS is successfully retained in the CRYPTOCELL AO power domain"]
    #[inline(always)]
    pub fn valid(self) -> &'a mut W {
        self.variant(LCS_IS_VALID_A::VALID)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Lifecycle state value. This field is write-once per reset."]
    #[inline(always)]
    pub fn lcs(&self) -> LCS_R {
        LCS_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 8 - This field is read-only and indicates if CRYPTOCELL LCS has been successfully configured since last reset"]
    #[inline(always)]
    pub fn lcs_is_valid(&self) -> LCS_IS_VALID_R {
        LCS_IS_VALID_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Lifecycle state value. This field is write-once per reset."]
    #[inline(always)]
    pub fn lcs(&mut self) -> LCS_W {
        LCS_W { w: self }
    }
    #[doc = "Bit 8 - This field is read-only and indicates if CRYPTOCELL LCS has been successfully configured since last reset"]
    #[inline(always)]
    pub fn lcs_is_valid(&mut self) -> LCS_IS_VALID_W {
        LCS_IS_VALID_W { w: self }
    }
}
