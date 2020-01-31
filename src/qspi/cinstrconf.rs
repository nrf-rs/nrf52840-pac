#[doc = "Reader of register CINSTRCONF"]
pub type R = crate::R<u32, super::CINSTRCONF>;
#[doc = "Writer for register CINSTRCONF"]
pub type W = crate::W<u32, super::CINSTRCONF>;
#[doc = "Register CINSTRCONF `reset()`'s with value 0x2000"]
impl crate::ResetValue for super::CINSTRCONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x2000
    }
}
#[doc = "Reader of field `OPCODE`"]
pub type OPCODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OPCODE`"]
pub struct OPCODE_W<'a> {
    w: &'a mut W,
}
impl<'a> OPCODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Length of custom instruction in number of bytes.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LENGTH_A {
    #[doc = "1: Send opcode only."]
    _1B = 1,
    #[doc = "2: Send opcode, CINSTRDAT0.BYTE0."]
    _2B = 2,
    #[doc = "3: Send opcode, CINSTRDAT0.BYTE0 -&gt; CINSTRDAT0.BYTE1."]
    _3B = 3,
    #[doc = "4: Send opcode, CINSTRDAT0.BYTE0 -&gt; CINSTRDAT0.BYTE2."]
    _4B = 4,
    #[doc = "5: Send opcode, CINSTRDAT0.BYTE0 -&gt; CINSTRDAT0.BYTE3."]
    _5B = 5,
    #[doc = "6: Send opcode, CINSTRDAT0.BYTE0 -&gt; CINSTRDAT1.BYTE4."]
    _6B = 6,
    #[doc = "7: Send opcode, CINSTRDAT0.BYTE0 -&gt; CINSTRDAT1.BYTE5."]
    _7B = 7,
    #[doc = "8: Send opcode, CINSTRDAT0.BYTE0 -&gt; CINSTRDAT1.BYTE6."]
    _8B = 8,
    #[doc = "9: Send opcode, CINSTRDAT0.BYTE0 -&gt; CINSTRDAT1.BYTE7."]
    _9B = 9,
}
impl From<LENGTH_A> for u8 {
    #[inline(always)]
    fn from(variant: LENGTH_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LENGTH`"]
pub type LENGTH_R = crate::R<u8, LENGTH_A>;
impl LENGTH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LENGTH_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(LENGTH_A::_1B),
            2 => Val(LENGTH_A::_2B),
            3 => Val(LENGTH_A::_3B),
            4 => Val(LENGTH_A::_4B),
            5 => Val(LENGTH_A::_5B),
            6 => Val(LENGTH_A::_6B),
            7 => Val(LENGTH_A::_7B),
            8 => Val(LENGTH_A::_8B),
            9 => Val(LENGTH_A::_9B),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1B`"]
    #[inline(always)]
    pub fn is_1b(&self) -> bool {
        *self == LENGTH_A::_1B
    }
    #[doc = "Checks if the value of the field is `_2B`"]
    #[inline(always)]
    pub fn is_2b(&self) -> bool {
        *self == LENGTH_A::_2B
    }
    #[doc = "Checks if the value of the field is `_3B`"]
    #[inline(always)]
    pub fn is_3b(&self) -> bool {
        *self == LENGTH_A::_3B
    }
    #[doc = "Checks if the value of the field is `_4B`"]
    #[inline(always)]
    pub fn is_4b(&self) -> bool {
        *self == LENGTH_A::_4B
    }
    #[doc = "Checks if the value of the field is `_5B`"]
    #[inline(always)]
    pub fn is_5b(&self) -> bool {
        *self == LENGTH_A::_5B
    }
    #[doc = "Checks if the value of the field is `_6B`"]
    #[inline(always)]
    pub fn is_6b(&self) -> bool {
        *self == LENGTH_A::_6B
    }
    #[doc = "Checks if the value of the field is `_7B`"]
    #[inline(always)]
    pub fn is_7b(&self) -> bool {
        *self == LENGTH_A::_7B
    }
    #[doc = "Checks if the value of the field is `_8B`"]
    #[inline(always)]
    pub fn is_8b(&self) -> bool {
        *self == LENGTH_A::_8B
    }
    #[doc = "Checks if the value of the field is `_9B`"]
    #[inline(always)]
    pub fn is_9b(&self) -> bool {
        *self == LENGTH_A::_9B
    }
}
#[doc = "Write proxy for field `LENGTH`"]
pub struct LENGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> LENGTH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LENGTH_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Send opcode only."]
    #[inline(always)]
    pub fn _1b(self) -> &'a mut W {
        self.variant(LENGTH_A::_1B)
    }
    #[doc = "Send opcode, CINSTRDAT0.BYTE0."]
    #[inline(always)]
    pub fn _2b(self) -> &'a mut W {
        self.variant(LENGTH_A::_2B)
    }
    #[doc = "Send opcode, CINSTRDAT0.BYTE0 -&gt; CINSTRDAT0.BYTE1."]
    #[inline(always)]
    pub fn _3b(self) -> &'a mut W {
        self.variant(LENGTH_A::_3B)
    }
    #[doc = "Send opcode, CINSTRDAT0.BYTE0 -&gt; CINSTRDAT0.BYTE2."]
    #[inline(always)]
    pub fn _4b(self) -> &'a mut W {
        self.variant(LENGTH_A::_4B)
    }
    #[doc = "Send opcode, CINSTRDAT0.BYTE0 -&gt; CINSTRDAT0.BYTE3."]
    #[inline(always)]
    pub fn _5b(self) -> &'a mut W {
        self.variant(LENGTH_A::_5B)
    }
    #[doc = "Send opcode, CINSTRDAT0.BYTE0 -&gt; CINSTRDAT1.BYTE4."]
    #[inline(always)]
    pub fn _6b(self) -> &'a mut W {
        self.variant(LENGTH_A::_6B)
    }
    #[doc = "Send opcode, CINSTRDAT0.BYTE0 -&gt; CINSTRDAT1.BYTE5."]
    #[inline(always)]
    pub fn _7b(self) -> &'a mut W {
        self.variant(LENGTH_A::_7B)
    }
    #[doc = "Send opcode, CINSTRDAT0.BYTE0 -&gt; CINSTRDAT1.BYTE6."]
    #[inline(always)]
    pub fn _8b(self) -> &'a mut W {
        self.variant(LENGTH_A::_8B)
    }
    #[doc = "Send opcode, CINSTRDAT0.BYTE0 -&gt; CINSTRDAT1.BYTE7."]
    #[inline(always)]
    pub fn _9b(self) -> &'a mut W {
        self.variant(LENGTH_A::_9B)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `LIO2`"]
pub type LIO2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LIO2`"]
pub struct LIO2_W<'a> {
    w: &'a mut W,
}
impl<'a> LIO2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `LIO3`"]
pub type LIO3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LIO3`"]
pub struct LIO3_W<'a> {
    w: &'a mut W,
}
impl<'a> LIO3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Wait for write complete before sending command.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WIPWAIT_A {
    #[doc = "0: No wait."]
    DISABLE = 0,
    #[doc = "1: Wait."]
    ENABLE = 1,
}
impl From<WIPWAIT_A> for bool {
    #[inline(always)]
    fn from(variant: WIPWAIT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WIPWAIT`"]
pub type WIPWAIT_R = crate::R<bool, WIPWAIT_A>;
impl WIPWAIT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WIPWAIT_A {
        match self.bits {
            false => WIPWAIT_A::DISABLE,
            true => WIPWAIT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WIPWAIT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WIPWAIT_A::ENABLE
    }
}
#[doc = "Write proxy for field `WIPWAIT`"]
pub struct WIPWAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> WIPWAIT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WIPWAIT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No wait."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WIPWAIT_A::DISABLE)
    }
    #[doc = "Wait."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WIPWAIT_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Send WREN (write enable opcode 0x06) before instruction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WREN_A {
    #[doc = "0: Do not send WREN."]
    DISABLE = 0,
    #[doc = "1: Send WREN."]
    ENABLE = 1,
}
impl From<WREN_A> for bool {
    #[inline(always)]
    fn from(variant: WREN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WREN`"]
pub type WREN_R = crate::R<bool, WREN_A>;
impl WREN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WREN_A {
        match self.bits {
            false => WREN_A::DISABLE,
            true => WREN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WREN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WREN_A::ENABLE
    }
}
#[doc = "Write proxy for field `WREN`"]
pub struct WREN_W<'a> {
    w: &'a mut W,
}
impl<'a> WREN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WREN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Do not send WREN."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WREN_A::DISABLE)
    }
    #[doc = "Send WREN."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WREN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Enable long frame mode. When enabled, a custom instruction transaction has to be ended by writing the LFSTOP field.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LFEN_A {
    #[doc = "0: Long frame mode disabled"]
    DISABLE = 0,
    #[doc = "1: Long frame mode enabled"]
    ENABLE = 1,
}
impl From<LFEN_A> for bool {
    #[inline(always)]
    fn from(variant: LFEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LFEN`"]
pub type LFEN_R = crate::R<bool, LFEN_A>;
impl LFEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LFEN_A {
        match self.bits {
            false => LFEN_A::DISABLE,
            true => LFEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == LFEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LFEN_A::ENABLE
    }
}
#[doc = "Write proxy for field `LFEN`"]
pub struct LFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LFEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LFEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Long frame mode disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(LFEN_A::DISABLE)
    }
    #[doc = "Long frame mode enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(LFEN_A::ENABLE)
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
#[doc = "Stop (finalize) long frame transaction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LFSTOP_A {
    #[doc = "1: Stop"]
    STOP = 1,
}
impl From<LFSTOP_A> for bool {
    #[inline(always)]
    fn from(variant: LFSTOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LFSTOP`"]
pub type LFSTOP_R = crate::R<bool, LFSTOP_A>;
impl LFSTOP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, LFSTOP_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(LFSTOP_A::STOP),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == LFSTOP_A::STOP
    }
}
#[doc = "Write proxy for field `LFSTOP`"]
pub struct LFSTOP_W<'a> {
    w: &'a mut W,
}
impl<'a> LFSTOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LFSTOP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Stop"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(LFSTOP_A::STOP)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Opcode of Custom instruction."]
    #[inline(always)]
    pub fn opcode(&self) -> OPCODE_R {
        OPCODE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - Length of custom instruction in number of bytes."]
    #[inline(always)]
    pub fn length(&self) -> LENGTH_R {
        LENGTH_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Level of the IO2 pin (if connected) during transmission of custom instruction."]
    #[inline(always)]
    pub fn lio2(&self) -> LIO2_R {
        LIO2_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Level of the IO3 pin (if connected) during transmission of custom instruction."]
    #[inline(always)]
    pub fn lio3(&self) -> LIO3_R {
        LIO3_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Wait for write complete before sending command."]
    #[inline(always)]
    pub fn wipwait(&self) -> WIPWAIT_R {
        WIPWAIT_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Send WREN (write enable opcode 0x06) before instruction."]
    #[inline(always)]
    pub fn wren(&self) -> WREN_R {
        WREN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Enable long frame mode. When enabled, a custom instruction transaction has to be ended by writing the LFSTOP field."]
    #[inline(always)]
    pub fn lfen(&self) -> LFEN_R {
        LFEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Stop (finalize) long frame transaction"]
    #[inline(always)]
    pub fn lfstop(&self) -> LFSTOP_R {
        LFSTOP_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Opcode of Custom instruction."]
    #[inline(always)]
    pub fn opcode(&mut self) -> OPCODE_W {
        OPCODE_W { w: self }
    }
    #[doc = "Bits 8:11 - Length of custom instruction in number of bytes."]
    #[inline(always)]
    pub fn length(&mut self) -> LENGTH_W {
        LENGTH_W { w: self }
    }
    #[doc = "Bit 12 - Level of the IO2 pin (if connected) during transmission of custom instruction."]
    #[inline(always)]
    pub fn lio2(&mut self) -> LIO2_W {
        LIO2_W { w: self }
    }
    #[doc = "Bit 13 - Level of the IO3 pin (if connected) during transmission of custom instruction."]
    #[inline(always)]
    pub fn lio3(&mut self) -> LIO3_W {
        LIO3_W { w: self }
    }
    #[doc = "Bit 14 - Wait for write complete before sending command."]
    #[inline(always)]
    pub fn wipwait(&mut self) -> WIPWAIT_W {
        WIPWAIT_W { w: self }
    }
    #[doc = "Bit 15 - Send WREN (write enable opcode 0x06) before instruction."]
    #[inline(always)]
    pub fn wren(&mut self) -> WREN_W {
        WREN_W { w: self }
    }
    #[doc = "Bit 16 - Enable long frame mode. When enabled, a custom instruction transaction has to be ended by writing the LFSTOP field."]
    #[inline(always)]
    pub fn lfen(&mut self) -> LFEN_W {
        LFEN_W { w: self }
    }
    #[doc = "Bit 17 - Stop (finalize) long frame transaction"]
    #[inline(always)]
    pub fn lfstop(&mut self) -> LFSTOP_W {
        LFSTOP_W { w: self }
    }
}
