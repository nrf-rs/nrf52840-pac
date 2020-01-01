#[doc = "Reader of register LFRCMODE"]
pub type R = crate::R<u32, super::LFRCMODE>;
#[doc = "Writer for register LFRCMODE"]
pub type W = crate::W<u32, super::LFRCMODE>;
#[doc = "Register LFRCMODE `reset()`'s with value 0"]
impl crate::ResetValue for super::LFRCMODE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Set LFRC mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE_A {
    #[doc = "0: Normal mode"]
    NORMAL = 0,
    #[doc = "1: Ultra-low power mode (ULP)"]
    ULP = 1,
}
impl From<MODE_A> for bool {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<bool, MODE_A>;
impl MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            false => MODE_A::NORMAL,
            true => MODE_A::ULP,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == MODE_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `ULP`"]
    #[inline(always)]
    pub fn is_ulp(&self) -> bool {
        *self == MODE_A::ULP
    }
}
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(MODE_A::NORMAL)
    }
    #[doc = "Ultra-low power mode (ULP)"]
    #[inline(always)]
    pub fn ulp(self) -> &'a mut W {
        self.variant(MODE_A::ULP)
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
#[doc = "Active LFRC mode. This field is read only.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STATUS_A {
    #[doc = "0: Normal mode"]
    NORMAL = 0,
    #[doc = "1: Ultra-low power mode (ULP)"]
    ULP = 1,
}
impl From<STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `STATUS`"]
pub type STATUS_R = crate::R<bool, STATUS_A>;
impl STATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STATUS_A {
        match self.bits {
            false => STATUS_A::NORMAL,
            true => STATUS_A::ULP,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == STATUS_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `ULP`"]
    #[inline(always)]
    pub fn is_ulp(&self) -> bool {
        *self == STATUS_A::ULP
    }
}
#[doc = "Write proxy for field `STATUS`"]
pub struct STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> STATUS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STATUS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(STATUS_A::NORMAL)
    }
    #[doc = "Ultra-low power mode (ULP)"]
    #[inline(always)]
    pub fn ulp(self) -> &'a mut W {
        self.variant(STATUS_A::ULP)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Set LFRC mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 16 - Active LFRC mode. This field is read only."]
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set LFRC mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 16 - Active LFRC mode. This field is read only."]
    #[inline(always)]
    pub fn status(&mut self) -> STATUS_W {
        STATUS_W { w: self }
    }
}
