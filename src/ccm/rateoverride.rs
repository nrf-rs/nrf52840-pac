#[doc = "Reader of register RATEOVERRIDE"]
pub type R = crate::R<u32, super::RATEOVERRIDE>;
#[doc = "Writer for register RATEOVERRIDE"]
pub type W = crate::W<u32, super::RATEOVERRIDE>;
#[doc = "Register RATEOVERRIDE `reset()`'s with value 0"]
impl crate::ResetValue for super::RATEOVERRIDE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Data rate override setting.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RATEOVERRIDE_A {
    #[doc = "0: 1 Mbps"]
    _1MBIT = 0,
    #[doc = "1: 2 Mbps"]
    _2MBIT = 1,
    #[doc = "2: 125 Kbps"]
    _125KBPS = 2,
    #[doc = "3: 500 Kbps"]
    _500KBPS = 3,
}
impl From<RATEOVERRIDE_A> for u8 {
    #[inline(always)]
    fn from(variant: RATEOVERRIDE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RATEOVERRIDE`"]
pub type RATEOVERRIDE_R = crate::R<u8, RATEOVERRIDE_A>;
impl RATEOVERRIDE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RATEOVERRIDE_A {
        match self.bits {
            0 => RATEOVERRIDE_A::_1MBIT,
            1 => RATEOVERRIDE_A::_2MBIT,
            2 => RATEOVERRIDE_A::_125KBPS,
            3 => RATEOVERRIDE_A::_500KBPS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_1MBIT`"]
    #[inline(always)]
    pub fn is_1mbit(&self) -> bool {
        *self == RATEOVERRIDE_A::_1MBIT
    }
    #[doc = "Checks if the value of the field is `_2MBIT`"]
    #[inline(always)]
    pub fn is_2mbit(&self) -> bool {
        *self == RATEOVERRIDE_A::_2MBIT
    }
    #[doc = "Checks if the value of the field is `_125KBPS`"]
    #[inline(always)]
    pub fn is_125kbps(&self) -> bool {
        *self == RATEOVERRIDE_A::_125KBPS
    }
    #[doc = "Checks if the value of the field is `_500KBPS`"]
    #[inline(always)]
    pub fn is_500kbps(&self) -> bool {
        *self == RATEOVERRIDE_A::_500KBPS
    }
}
#[doc = "Write proxy for field `RATEOVERRIDE`"]
pub struct RATEOVERRIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> RATEOVERRIDE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RATEOVERRIDE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "1 Mbps"]
    #[inline(always)]
    pub fn _1mbit(self) -> &'a mut W {
        self.variant(RATEOVERRIDE_A::_1MBIT)
    }
    #[doc = "2 Mbps"]
    #[inline(always)]
    pub fn _2mbit(self) -> &'a mut W {
        self.variant(RATEOVERRIDE_A::_2MBIT)
    }
    #[doc = "125 Kbps"]
    #[inline(always)]
    pub fn _125kbps(self) -> &'a mut W {
        self.variant(RATEOVERRIDE_A::_125KBPS)
    }
    #[doc = "500 Kbps"]
    #[inline(always)]
    pub fn _500kbps(self) -> &'a mut W {
        self.variant(RATEOVERRIDE_A::_500KBPS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Data rate override setting."]
    #[inline(always)]
    pub fn rateoverride(&self) -> RATEOVERRIDE_R {
        RATEOVERRIDE_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Data rate override setting."]
    #[inline(always)]
    pub fn rateoverride(&mut self) -> RATEOVERRIDE_W {
        RATEOVERRIDE_W { w: self }
    }
}
