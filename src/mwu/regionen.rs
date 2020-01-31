#[doc = "Reader of register REGIONEN"]
pub type R = crate::R<u32, super::REGIONEN>;
#[doc = "Writer for register REGIONEN"]
pub type W = crate::W<u32, super::REGIONEN>;
#[doc = "Register REGIONEN `reset()`'s with value 0"]
impl crate::ResetValue for super::REGIONEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Enable/disable write access watch in region\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RGN0WA_A {
    #[doc = "0: Disable write access watch in this region"]
    DISABLE = 0,
    #[doc = "1: Enable write access watch in this region"]
    ENABLE = 1,
}
impl From<RGN0WA_A> for bool {
    #[inline(always)]
    fn from(variant: RGN0WA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RGN0WA`"]
pub type RGN0WA_R = crate::R<bool, RGN0WA_A>;
impl RGN0WA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RGN0WA_A {
        match self.bits {
            false => RGN0WA_A::DISABLE,
            true => RGN0WA_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RGN0WA_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RGN0WA_A::ENABLE
    }
}
#[doc = "Write proxy for field `RGN0WA`"]
pub struct RGN0WA_W<'a> {
    w: &'a mut W,
}
impl<'a> RGN0WA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RGN0WA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable write access watch in this region"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RGN0WA_A::DISABLE)
    }
    #[doc = "Enable write access watch in this region"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RGN0WA_A::ENABLE)
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
#[doc = "Enable/disable read access watch in region\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RGN0RA_A {
    #[doc = "0: Disable read access watch in this region"]
    DISABLE = 0,
    #[doc = "1: Enable read access watch in this region"]
    ENABLE = 1,
}
impl From<RGN0RA_A> for bool {
    #[inline(always)]
    fn from(variant: RGN0RA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RGN0RA`"]
pub type RGN0RA_R = crate::R<bool, RGN0RA_A>;
impl RGN0RA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RGN0RA_A {
        match self.bits {
            false => RGN0RA_A::DISABLE,
            true => RGN0RA_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RGN0RA_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RGN0RA_A::ENABLE
    }
}
#[doc = "Write proxy for field `RGN0RA`"]
pub struct RGN0RA_W<'a> {
    w: &'a mut W,
}
impl<'a> RGN0RA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RGN0RA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable read access watch in this region"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RGN0RA_A::DISABLE)
    }
    #[doc = "Enable read access watch in this region"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RGN0RA_A::ENABLE)
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
#[doc = "Enable/disable write access watch in region\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RGN1WA_A {
    #[doc = "0: Disable write access watch in this region"]
    DISABLE = 0,
    #[doc = "1: Enable write access watch in this region"]
    ENABLE = 1,
}
impl From<RGN1WA_A> for bool {
    #[inline(always)]
    fn from(variant: RGN1WA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RGN1WA`"]
pub type RGN1WA_R = crate::R<bool, RGN1WA_A>;
impl RGN1WA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RGN1WA_A {
        match self.bits {
            false => RGN1WA_A::DISABLE,
            true => RGN1WA_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RGN1WA_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RGN1WA_A::ENABLE
    }
}
#[doc = "Write proxy for field `RGN1WA`"]
pub struct RGN1WA_W<'a> {
    w: &'a mut W,
}
impl<'a> RGN1WA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RGN1WA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable write access watch in this region"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RGN1WA_A::DISABLE)
    }
    #[doc = "Enable write access watch in this region"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RGN1WA_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Enable/disable read access watch in region\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RGN1RA_A {
    #[doc = "0: Disable read access watch in this region"]
    DISABLE = 0,
    #[doc = "1: Enable read access watch in this region"]
    ENABLE = 1,
}
impl From<RGN1RA_A> for bool {
    #[inline(always)]
    fn from(variant: RGN1RA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RGN1RA`"]
pub type RGN1RA_R = crate::R<bool, RGN1RA_A>;
impl RGN1RA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RGN1RA_A {
        match self.bits {
            false => RGN1RA_A::DISABLE,
            true => RGN1RA_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RGN1RA_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RGN1RA_A::ENABLE
    }
}
#[doc = "Write proxy for field `RGN1RA`"]
pub struct RGN1RA_W<'a> {
    w: &'a mut W,
}
impl<'a> RGN1RA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RGN1RA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable read access watch in this region"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RGN1RA_A::DISABLE)
    }
    #[doc = "Enable read access watch in this region"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RGN1RA_A::ENABLE)
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
#[doc = "Enable/disable write access watch in region\\[2\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RGN2WA_A {
    #[doc = "0: Disable write access watch in this region"]
    DISABLE = 0,
    #[doc = "1: Enable write access watch in this region"]
    ENABLE = 1,
}
impl From<RGN2WA_A> for bool {
    #[inline(always)]
    fn from(variant: RGN2WA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RGN2WA`"]
pub type RGN2WA_R = crate::R<bool, RGN2WA_A>;
impl RGN2WA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RGN2WA_A {
        match self.bits {
            false => RGN2WA_A::DISABLE,
            true => RGN2WA_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RGN2WA_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RGN2WA_A::ENABLE
    }
}
#[doc = "Write proxy for field `RGN2WA`"]
pub struct RGN2WA_W<'a> {
    w: &'a mut W,
}
impl<'a> RGN2WA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RGN2WA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable write access watch in this region"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RGN2WA_A::DISABLE)
    }
    #[doc = "Enable write access watch in this region"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RGN2WA_A::ENABLE)
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
#[doc = "Enable/disable read access watch in region\\[2\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RGN2RA_A {
    #[doc = "0: Disable read access watch in this region"]
    DISABLE = 0,
    #[doc = "1: Enable read access watch in this region"]
    ENABLE = 1,
}
impl From<RGN2RA_A> for bool {
    #[inline(always)]
    fn from(variant: RGN2RA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RGN2RA`"]
pub type RGN2RA_R = crate::R<bool, RGN2RA_A>;
impl RGN2RA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RGN2RA_A {
        match self.bits {
            false => RGN2RA_A::DISABLE,
            true => RGN2RA_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RGN2RA_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RGN2RA_A::ENABLE
    }
}
#[doc = "Write proxy for field `RGN2RA`"]
pub struct RGN2RA_W<'a> {
    w: &'a mut W,
}
impl<'a> RGN2RA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RGN2RA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable read access watch in this region"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RGN2RA_A::DISABLE)
    }
    #[doc = "Enable read access watch in this region"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RGN2RA_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Enable/disable write access watch in region\\[3\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RGN3WA_A {
    #[doc = "0: Disable write access watch in this region"]
    DISABLE = 0,
    #[doc = "1: Enable write access watch in this region"]
    ENABLE = 1,
}
impl From<RGN3WA_A> for bool {
    #[inline(always)]
    fn from(variant: RGN3WA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RGN3WA`"]
pub type RGN3WA_R = crate::R<bool, RGN3WA_A>;
impl RGN3WA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RGN3WA_A {
        match self.bits {
            false => RGN3WA_A::DISABLE,
            true => RGN3WA_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RGN3WA_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RGN3WA_A::ENABLE
    }
}
#[doc = "Write proxy for field `RGN3WA`"]
pub struct RGN3WA_W<'a> {
    w: &'a mut W,
}
impl<'a> RGN3WA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RGN3WA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable write access watch in this region"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RGN3WA_A::DISABLE)
    }
    #[doc = "Enable write access watch in this region"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RGN3WA_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Enable/disable read access watch in region\\[3\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RGN3RA_A {
    #[doc = "0: Disable read access watch in this region"]
    DISABLE = 0,
    #[doc = "1: Enable read access watch in this region"]
    ENABLE = 1,
}
impl From<RGN3RA_A> for bool {
    #[inline(always)]
    fn from(variant: RGN3RA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RGN3RA`"]
pub type RGN3RA_R = crate::R<bool, RGN3RA_A>;
impl RGN3RA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RGN3RA_A {
        match self.bits {
            false => RGN3RA_A::DISABLE,
            true => RGN3RA_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RGN3RA_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RGN3RA_A::ENABLE
    }
}
#[doc = "Write proxy for field `RGN3RA`"]
pub struct RGN3RA_W<'a> {
    w: &'a mut W,
}
impl<'a> RGN3RA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RGN3RA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable read access watch in this region"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RGN3RA_A::DISABLE)
    }
    #[doc = "Enable read access watch in this region"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RGN3RA_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Enable/disable write access watch in PREGION\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRGN0WA_A {
    #[doc = "0: Disable write access watch in this PREGION"]
    DISABLE = 0,
    #[doc = "1: Enable write access watch in this PREGION"]
    ENABLE = 1,
}
impl From<PRGN0WA_A> for bool {
    #[inline(always)]
    fn from(variant: PRGN0WA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PRGN0WA`"]
pub type PRGN0WA_R = crate::R<bool, PRGN0WA_A>;
impl PRGN0WA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRGN0WA_A {
        match self.bits {
            false => PRGN0WA_A::DISABLE,
            true => PRGN0WA_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PRGN0WA_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PRGN0WA_A::ENABLE
    }
}
#[doc = "Write proxy for field `PRGN0WA`"]
pub struct PRGN0WA_W<'a> {
    w: &'a mut W,
}
impl<'a> PRGN0WA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRGN0WA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable write access watch in this PREGION"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PRGN0WA_A::DISABLE)
    }
    #[doc = "Enable write access watch in this PREGION"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PRGN0WA_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Enable/disable read access watch in PREGION\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRGN0RA_A {
    #[doc = "0: Disable read access watch in this PREGION"]
    DISABLE = 0,
    #[doc = "1: Enable read access watch in this PREGION"]
    ENABLE = 1,
}
impl From<PRGN0RA_A> for bool {
    #[inline(always)]
    fn from(variant: PRGN0RA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PRGN0RA`"]
pub type PRGN0RA_R = crate::R<bool, PRGN0RA_A>;
impl PRGN0RA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRGN0RA_A {
        match self.bits {
            false => PRGN0RA_A::DISABLE,
            true => PRGN0RA_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PRGN0RA_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PRGN0RA_A::ENABLE
    }
}
#[doc = "Write proxy for field `PRGN0RA`"]
pub struct PRGN0RA_W<'a> {
    w: &'a mut W,
}
impl<'a> PRGN0RA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRGN0RA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable read access watch in this PREGION"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PRGN0RA_A::DISABLE)
    }
    #[doc = "Enable read access watch in this PREGION"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PRGN0RA_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Enable/disable write access watch in PREGION\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRGN1WA_A {
    #[doc = "0: Disable write access watch in this PREGION"]
    DISABLE = 0,
    #[doc = "1: Enable write access watch in this PREGION"]
    ENABLE = 1,
}
impl From<PRGN1WA_A> for bool {
    #[inline(always)]
    fn from(variant: PRGN1WA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PRGN1WA`"]
pub type PRGN1WA_R = crate::R<bool, PRGN1WA_A>;
impl PRGN1WA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRGN1WA_A {
        match self.bits {
            false => PRGN1WA_A::DISABLE,
            true => PRGN1WA_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PRGN1WA_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PRGN1WA_A::ENABLE
    }
}
#[doc = "Write proxy for field `PRGN1WA`"]
pub struct PRGN1WA_W<'a> {
    w: &'a mut W,
}
impl<'a> PRGN1WA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRGN1WA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable write access watch in this PREGION"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PRGN1WA_A::DISABLE)
    }
    #[doc = "Enable write access watch in this PREGION"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PRGN1WA_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Enable/disable read access watch in PREGION\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRGN1RA_A {
    #[doc = "0: Disable read access watch in this PREGION"]
    DISABLE = 0,
    #[doc = "1: Enable read access watch in this PREGION"]
    ENABLE = 1,
}
impl From<PRGN1RA_A> for bool {
    #[inline(always)]
    fn from(variant: PRGN1RA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PRGN1RA`"]
pub type PRGN1RA_R = crate::R<bool, PRGN1RA_A>;
impl PRGN1RA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRGN1RA_A {
        match self.bits {
            false => PRGN1RA_A::DISABLE,
            true => PRGN1RA_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PRGN1RA_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PRGN1RA_A::ENABLE
    }
}
#[doc = "Write proxy for field `PRGN1RA`"]
pub struct PRGN1RA_W<'a> {
    w: &'a mut W,
}
impl<'a> PRGN1RA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRGN1RA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable read access watch in this PREGION"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PRGN1RA_A::DISABLE)
    }
    #[doc = "Enable read access watch in this PREGION"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PRGN1RA_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable/disable write access watch in region\\[0\\]"]
    #[inline(always)]
    pub fn rgn0wa(&self) -> RGN0WA_R {
        RGN0WA_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable/disable read access watch in region\\[0\\]"]
    #[inline(always)]
    pub fn rgn0ra(&self) -> RGN0RA_R {
        RGN0RA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable/disable write access watch in region\\[1\\]"]
    #[inline(always)]
    pub fn rgn1wa(&self) -> RGN1WA_R {
        RGN1WA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable/disable read access watch in region\\[1\\]"]
    #[inline(always)]
    pub fn rgn1ra(&self) -> RGN1RA_R {
        RGN1RA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable/disable write access watch in region\\[2\\]"]
    #[inline(always)]
    pub fn rgn2wa(&self) -> RGN2WA_R {
        RGN2WA_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable/disable read access watch in region\\[2\\]"]
    #[inline(always)]
    pub fn rgn2ra(&self) -> RGN2RA_R {
        RGN2RA_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable/disable write access watch in region\\[3\\]"]
    #[inline(always)]
    pub fn rgn3wa(&self) -> RGN3WA_R {
        RGN3WA_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable/disable read access watch in region\\[3\\]"]
    #[inline(always)]
    pub fn rgn3ra(&self) -> RGN3RA_R {
        RGN3RA_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Enable/disable write access watch in PREGION\\[0\\]"]
    #[inline(always)]
    pub fn prgn0wa(&self) -> PRGN0WA_R {
        PRGN0WA_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Enable/disable read access watch in PREGION\\[0\\]"]
    #[inline(always)]
    pub fn prgn0ra(&self) -> PRGN0RA_R {
        PRGN0RA_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Enable/disable write access watch in PREGION\\[1\\]"]
    #[inline(always)]
    pub fn prgn1wa(&self) -> PRGN1WA_R {
        PRGN1WA_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Enable/disable read access watch in PREGION\\[1\\]"]
    #[inline(always)]
    pub fn prgn1ra(&self) -> PRGN1RA_R {
        PRGN1RA_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable/disable write access watch in region\\[0\\]"]
    #[inline(always)]
    pub fn rgn0wa(&mut self) -> RGN0WA_W {
        RGN0WA_W { w: self }
    }
    #[doc = "Bit 1 - Enable/disable read access watch in region\\[0\\]"]
    #[inline(always)]
    pub fn rgn0ra(&mut self) -> RGN0RA_W {
        RGN0RA_W { w: self }
    }
    #[doc = "Bit 2 - Enable/disable write access watch in region\\[1\\]"]
    #[inline(always)]
    pub fn rgn1wa(&mut self) -> RGN1WA_W {
        RGN1WA_W { w: self }
    }
    #[doc = "Bit 3 - Enable/disable read access watch in region\\[1\\]"]
    #[inline(always)]
    pub fn rgn1ra(&mut self) -> RGN1RA_W {
        RGN1RA_W { w: self }
    }
    #[doc = "Bit 4 - Enable/disable write access watch in region\\[2\\]"]
    #[inline(always)]
    pub fn rgn2wa(&mut self) -> RGN2WA_W {
        RGN2WA_W { w: self }
    }
    #[doc = "Bit 5 - Enable/disable read access watch in region\\[2\\]"]
    #[inline(always)]
    pub fn rgn2ra(&mut self) -> RGN2RA_W {
        RGN2RA_W { w: self }
    }
    #[doc = "Bit 6 - Enable/disable write access watch in region\\[3\\]"]
    #[inline(always)]
    pub fn rgn3wa(&mut self) -> RGN3WA_W {
        RGN3WA_W { w: self }
    }
    #[doc = "Bit 7 - Enable/disable read access watch in region\\[3\\]"]
    #[inline(always)]
    pub fn rgn3ra(&mut self) -> RGN3RA_W {
        RGN3RA_W { w: self }
    }
    #[doc = "Bit 24 - Enable/disable write access watch in PREGION\\[0\\]"]
    #[inline(always)]
    pub fn prgn0wa(&mut self) -> PRGN0WA_W {
        PRGN0WA_W { w: self }
    }
    #[doc = "Bit 25 - Enable/disable read access watch in PREGION\\[0\\]"]
    #[inline(always)]
    pub fn prgn0ra(&mut self) -> PRGN0RA_W {
        PRGN0RA_W { w: self }
    }
    #[doc = "Bit 26 - Enable/disable write access watch in PREGION\\[1\\]"]
    #[inline(always)]
    pub fn prgn1wa(&mut self) -> PRGN1WA_W {
        PRGN1WA_W { w: self }
    }
    #[doc = "Bit 27 - Enable/disable read access watch in PREGION\\[1\\]"]
    #[inline(always)]
    pub fn prgn1ra(&mut self) -> PRGN1RA_W {
        PRGN1RA_W { w: self }
    }
}
