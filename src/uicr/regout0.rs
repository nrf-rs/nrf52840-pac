#[doc = "Reader of register REGOUT0"]
pub type R = crate::R<u32, super::REGOUT0>;
#[doc = "Writer for register REGOUT0"]
pub type W = crate::W<u32, super::REGOUT0>;
#[doc = "Register REGOUT0 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::REGOUT0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Output voltage from of REG0 regulator stage. The maximum output voltage from this stage is given as VDDH - VEXDIF.\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum VOUT_A {
    #[doc = "0: 1.8 V"]
    _1V8 = 0,
    #[doc = "1: 2.1 V"]
    _2V1 = 1,
    #[doc = "2: 2.4 V"]
    _2V4 = 2,
    #[doc = "3: 2.7 V"]
    _2V7 = 3,
    #[doc = "4: 3.0 V"]
    _3V0 = 4,
    #[doc = "5: 3.3 V"]
    _3V3 = 5,
    #[doc = "7: Default voltage: 1.8 V"]
    DEFAULT = 7,
}
impl From<VOUT_A> for u8 {
    #[inline(always)]
    fn from(variant: VOUT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `VOUT`"]
pub type VOUT_R = crate::R<u8, VOUT_A>;
impl VOUT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, VOUT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(VOUT_A::_1V8),
            1 => Val(VOUT_A::_2V1),
            2 => Val(VOUT_A::_2V4),
            3 => Val(VOUT_A::_2V7),
            4 => Val(VOUT_A::_3V0),
            5 => Val(VOUT_A::_3V3),
            7 => Val(VOUT_A::DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1V8`"]
    #[inline(always)]
    pub fn is_1v8(&self) -> bool {
        *self == VOUT_A::_1V8
    }
    #[doc = "Checks if the value of the field is `_2V1`"]
    #[inline(always)]
    pub fn is_2v1(&self) -> bool {
        *self == VOUT_A::_2V1
    }
    #[doc = "Checks if the value of the field is `_2V4`"]
    #[inline(always)]
    pub fn is_2v4(&self) -> bool {
        *self == VOUT_A::_2V4
    }
    #[doc = "Checks if the value of the field is `_2V7`"]
    #[inline(always)]
    pub fn is_2v7(&self) -> bool {
        *self == VOUT_A::_2V7
    }
    #[doc = "Checks if the value of the field is `_3V0`"]
    #[inline(always)]
    pub fn is_3v0(&self) -> bool {
        *self == VOUT_A::_3V0
    }
    #[doc = "Checks if the value of the field is `_3V3`"]
    #[inline(always)]
    pub fn is_3v3(&self) -> bool {
        *self == VOUT_A::_3V3
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == VOUT_A::DEFAULT
    }
}
#[doc = "Write proxy for field `VOUT`"]
pub struct VOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> VOUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VOUT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1.8 V"]
    #[inline(always)]
    pub fn _1v8(self) -> &'a mut W {
        self.variant(VOUT_A::_1V8)
    }
    #[doc = "2.1 V"]
    #[inline(always)]
    pub fn _2v1(self) -> &'a mut W {
        self.variant(VOUT_A::_2V1)
    }
    #[doc = "2.4 V"]
    #[inline(always)]
    pub fn _2v4(self) -> &'a mut W {
        self.variant(VOUT_A::_2V4)
    }
    #[doc = "2.7 V"]
    #[inline(always)]
    pub fn _2v7(self) -> &'a mut W {
        self.variant(VOUT_A::_2V7)
    }
    #[doc = "3.0 V"]
    #[inline(always)]
    pub fn _3v0(self) -> &'a mut W {
        self.variant(VOUT_A::_3V0)
    }
    #[doc = "3.3 V"]
    #[inline(always)]
    pub fn _3v3(self) -> &'a mut W {
        self.variant(VOUT_A::_3V3)
    }
    #[doc = "Default voltage: 1.8 V"]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(VOUT_A::DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Output voltage from of REG0 regulator stage. The maximum output voltage from this stage is given as VDDH - VEXDIF."]
    #[inline(always)]
    pub fn vout(&self) -> VOUT_R {
        VOUT_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Output voltage from of REG0 regulator stage. The maximum output voltage from this stage is given as VDDH - VEXDIF."]
    #[inline(always)]
    pub fn vout(&mut self) -> VOUT_W {
        VOUT_W { w: self }
    }
}
