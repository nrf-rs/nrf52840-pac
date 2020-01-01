#[doc = "Reader of register STALLSTAT"]
pub type R = crate::R<u32, super::STALLSTAT>;
#[doc = "Writer for register STALLSTAT"]
pub type W = crate::W<u32, super::STALLSTAT>;
#[doc = "Register STALLSTAT `reset()`'s with value 0"]
impl crate::ResetValue for super::STALLSTAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Stall status for EasyDMA RAM reads\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_A {
    #[doc = "0: No stall"]
    NOSTALL = 0,
    #[doc = "1: A stall has occurred"]
    STALL = 1,
}
impl From<TX_A> for bool {
    #[inline(always)]
    fn from(variant: TX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TX`"]
pub type TX_R = crate::R<bool, TX_A>;
impl TX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_A {
        match self.bits {
            false => TX_A::NOSTALL,
            true => TX_A::STALL,
        }
    }
    #[doc = "Checks if the value of the field is `NOSTALL`"]
    #[inline(always)]
    pub fn is_nostall(&self) -> bool {
        *self == TX_A::NOSTALL
    }
    #[doc = "Checks if the value of the field is `STALL`"]
    #[inline(always)]
    pub fn is_stall(&self) -> bool {
        *self == TX_A::STALL
    }
}
#[doc = "Write proxy for field `TX`"]
pub struct TX_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No stall"]
    #[inline(always)]
    pub fn nostall(self) -> &'a mut W {
        self.variant(TX_A::NOSTALL)
    }
    #[doc = "A stall has occurred"]
    #[inline(always)]
    pub fn stall(self) -> &'a mut W {
        self.variant(TX_A::STALL)
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
#[doc = "Stall status for EasyDMA RAM writes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_A {
    #[doc = "0: No stall"]
    NOSTALL = 0,
    #[doc = "1: A stall has occurred"]
    STALL = 1,
}
impl From<RX_A> for bool {
    #[inline(always)]
    fn from(variant: RX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RX`"]
pub type RX_R = crate::R<bool, RX_A>;
impl RX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_A {
        match self.bits {
            false => RX_A::NOSTALL,
            true => RX_A::STALL,
        }
    }
    #[doc = "Checks if the value of the field is `NOSTALL`"]
    #[inline(always)]
    pub fn is_nostall(&self) -> bool {
        *self == RX_A::NOSTALL
    }
    #[doc = "Checks if the value of the field is `STALL`"]
    #[inline(always)]
    pub fn is_stall(&self) -> bool {
        *self == RX_A::STALL
    }
}
#[doc = "Write proxy for field `RX`"]
pub struct RX_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No stall"]
    #[inline(always)]
    pub fn nostall(self) -> &'a mut W {
        self.variant(RX_A::NOSTALL)
    }
    #[doc = "A stall has occurred"]
    #[inline(always)]
    pub fn stall(self) -> &'a mut W {
        self.variant(RX_A::STALL)
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
impl R {
    #[doc = "Bit 0 - Stall status for EasyDMA RAM reads"]
    #[inline(always)]
    pub fn tx(&self) -> TX_R {
        TX_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Stall status for EasyDMA RAM writes"]
    #[inline(always)]
    pub fn rx(&self) -> RX_R {
        RX_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Stall status for EasyDMA RAM reads"]
    #[inline(always)]
    pub fn tx(&mut self) -> TX_W {
        TX_W { w: self }
    }
    #[doc = "Bit 1 - Stall status for EasyDMA RAM writes"]
    #[inline(always)]
    pub fn rx(&mut self) -> RX_W {
        RX_W { w: self }
    }
}
