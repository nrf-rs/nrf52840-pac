#[doc = "Reader of register INTENSET"]
pub type R = crate::R<u32, super::INTENSET>;
#[doc = "Writer for register INTENSET"]
pub type W = crate::W<u32, super::INTENSET>;
#[doc = "Register INTENSET `reset()`'s with value 0"]
impl crate::ResetValue for super::INTENSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write '1' to enable interrupt for HFCLKSTARTED event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HFCLKSTARTED_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<HFCLKSTARTED_A> for bool {
    #[inline(always)]
    fn from(variant: HFCLKSTARTED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HFCLKSTARTED`"]
pub type HFCLKSTARTED_R = crate::R<bool, HFCLKSTARTED_A>;
impl HFCLKSTARTED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HFCLKSTARTED_A {
        match self.bits {
            false => HFCLKSTARTED_A::DISABLED,
            true => HFCLKSTARTED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HFCLKSTARTED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HFCLKSTARTED_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for HFCLKSTARTED event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HFCLKSTARTED_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<HFCLKSTARTED_AW> for bool {
    #[inline(always)]
    fn from(variant: HFCLKSTARTED_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `HFCLKSTARTED`"]
pub struct HFCLKSTARTED_W<'a> {
    w: &'a mut W,
}
impl<'a> HFCLKSTARTED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HFCLKSTARTED_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(HFCLKSTARTED_AW::SET)
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
#[doc = "Write '1' to enable interrupt for LFCLKSTARTED event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LFCLKSTARTED_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<LFCLKSTARTED_A> for bool {
    #[inline(always)]
    fn from(variant: LFCLKSTARTED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LFCLKSTARTED`"]
pub type LFCLKSTARTED_R = crate::R<bool, LFCLKSTARTED_A>;
impl LFCLKSTARTED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LFCLKSTARTED_A {
        match self.bits {
            false => LFCLKSTARTED_A::DISABLED,
            true => LFCLKSTARTED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LFCLKSTARTED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LFCLKSTARTED_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for LFCLKSTARTED event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LFCLKSTARTED_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<LFCLKSTARTED_AW> for bool {
    #[inline(always)]
    fn from(variant: LFCLKSTARTED_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `LFCLKSTARTED`"]
pub struct LFCLKSTARTED_W<'a> {
    w: &'a mut W,
}
impl<'a> LFCLKSTARTED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LFCLKSTARTED_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(LFCLKSTARTED_AW::SET)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Write '1' to enable interrupt for DONE event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DONE_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<DONE_A> for bool {
    #[inline(always)]
    fn from(variant: DONE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DONE`"]
pub type DONE_R = crate::R<bool, DONE_A>;
impl DONE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DONE_A {
        match self.bits {
            false => DONE_A::DISABLED,
            true => DONE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DONE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DONE_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for DONE event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DONE_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<DONE_AW> for bool {
    #[inline(always)]
    fn from(variant: DONE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `DONE`"]
pub struct DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> DONE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DONE_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(DONE_AW::SET)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Write '1' to enable interrupt for CTTO event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTTO_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<CTTO_A> for bool {
    #[inline(always)]
    fn from(variant: CTTO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CTTO`"]
pub type CTTO_R = crate::R<bool, CTTO_A>;
impl CTTO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTTO_A {
        match self.bits {
            false => CTTO_A::DISABLED,
            true => CTTO_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CTTO_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CTTO_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for CTTO event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTTO_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<CTTO_AW> for bool {
    #[inline(always)]
    fn from(variant: CTTO_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CTTO`"]
pub struct CTTO_W<'a> {
    w: &'a mut W,
}
impl<'a> CTTO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTTO_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CTTO_AW::SET)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Write '1' to enable interrupt for CTSTARTED event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTSTARTED_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<CTSTARTED_A> for bool {
    #[inline(always)]
    fn from(variant: CTSTARTED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CTSTARTED`"]
pub type CTSTARTED_R = crate::R<bool, CTSTARTED_A>;
impl CTSTARTED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTSTARTED_A {
        match self.bits {
            false => CTSTARTED_A::DISABLED,
            true => CTSTARTED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CTSTARTED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CTSTARTED_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for CTSTARTED event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTSTARTED_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<CTSTARTED_AW> for bool {
    #[inline(always)]
    fn from(variant: CTSTARTED_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CTSTARTED`"]
pub struct CTSTARTED_W<'a> {
    w: &'a mut W,
}
impl<'a> CTSTARTED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTSTARTED_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CTSTARTED_AW::SET)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Write '1' to enable interrupt for CTSTOPPED event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTSTOPPED_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<CTSTOPPED_A> for bool {
    #[inline(always)]
    fn from(variant: CTSTOPPED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CTSTOPPED`"]
pub type CTSTOPPED_R = crate::R<bool, CTSTOPPED_A>;
impl CTSTOPPED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTSTOPPED_A {
        match self.bits {
            false => CTSTOPPED_A::DISABLED,
            true => CTSTOPPED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CTSTOPPED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CTSTOPPED_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for CTSTOPPED event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTSTOPPED_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<CTSTOPPED_AW> for bool {
    #[inline(always)]
    fn from(variant: CTSTOPPED_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CTSTOPPED`"]
pub struct CTSTOPPED_W<'a> {
    w: &'a mut W,
}
impl<'a> CTSTOPPED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTSTOPPED_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CTSTOPPED_AW::SET)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Write '1' to enable interrupt for HFCLKSTARTED event"]
    #[inline(always)]
    pub fn hfclkstarted(&self) -> HFCLKSTARTED_R {
        HFCLKSTARTED_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Write '1' to enable interrupt for LFCLKSTARTED event"]
    #[inline(always)]
    pub fn lfclkstarted(&self) -> LFCLKSTARTED_R {
        LFCLKSTARTED_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Write '1' to enable interrupt for DONE event"]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Write '1' to enable interrupt for CTTO event"]
    #[inline(always)]
    pub fn ctto(&self) -> CTTO_R {
        CTTO_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Write '1' to enable interrupt for CTSTARTED event"]
    #[inline(always)]
    pub fn ctstarted(&self) -> CTSTARTED_R {
        CTSTARTED_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Write '1' to enable interrupt for CTSTOPPED event"]
    #[inline(always)]
    pub fn ctstopped(&self) -> CTSTOPPED_R {
        CTSTOPPED_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write '1' to enable interrupt for HFCLKSTARTED event"]
    #[inline(always)]
    pub fn hfclkstarted(&mut self) -> HFCLKSTARTED_W {
        HFCLKSTARTED_W { w: self }
    }
    #[doc = "Bit 1 - Write '1' to enable interrupt for LFCLKSTARTED event"]
    #[inline(always)]
    pub fn lfclkstarted(&mut self) -> LFCLKSTARTED_W {
        LFCLKSTARTED_W { w: self }
    }
    #[doc = "Bit 3 - Write '1' to enable interrupt for DONE event"]
    #[inline(always)]
    pub fn done(&mut self) -> DONE_W {
        DONE_W { w: self }
    }
    #[doc = "Bit 4 - Write '1' to enable interrupt for CTTO event"]
    #[inline(always)]
    pub fn ctto(&mut self) -> CTTO_W {
        CTTO_W { w: self }
    }
    #[doc = "Bit 10 - Write '1' to enable interrupt for CTSTARTED event"]
    #[inline(always)]
    pub fn ctstarted(&mut self) -> CTSTARTED_W {
        CTSTARTED_W { w: self }
    }
    #[doc = "Bit 11 - Write '1' to enable interrupt for CTSTOPPED event"]
    #[inline(always)]
    pub fn ctstopped(&mut self) -> CTSTOPPED_W {
        CTSTOPPED_W { w: self }
    }
}
