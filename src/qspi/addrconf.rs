#[doc = "Reader of register ADDRCONF"]
pub type R = crate::R<u32, super::ADDRCONF>;
#[doc = "Writer for register ADDRCONF"]
pub type W = crate::W<u32, super::ADDRCONF>;
#[doc = "Register ADDRCONF `reset()`'s with value 0xb7"]
impl crate::ResetValue for super::ADDRCONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xb7
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
#[doc = "Reader of field `BYTE0`"]
pub type BYTE0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BYTE0`"]
pub struct BYTE0_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTE0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `BYTE1`"]
pub type BYTE1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BYTE1`"]
pub struct BYTE1_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTE1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Extended addressing mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: Do not send any instruction."]
    NOINSTR = 0,
    #[doc = "1: Send opcode."]
    OPCODE = 1,
    #[doc = "2: Send opcode, byte0."]
    OPBYTE0 = 2,
    #[doc = "3: Send opcode, byte0, byte1."]
    ALL = 3,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<u8, MODE_A>;
impl MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            0 => MODE_A::NOINSTR,
            1 => MODE_A::OPCODE,
            2 => MODE_A::OPBYTE0,
            3 => MODE_A::ALL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NOINSTR`"]
    #[inline(always)]
    pub fn is_no_instr(&self) -> bool {
        *self == MODE_A::NOINSTR
    }
    #[doc = "Checks if the value of the field is `OPCODE`"]
    #[inline(always)]
    pub fn is_opcode(&self) -> bool {
        *self == MODE_A::OPCODE
    }
    #[doc = "Checks if the value of the field is `OPBYTE0`"]
    #[inline(always)]
    pub fn is_op_byte0(&self) -> bool {
        *self == MODE_A::OPBYTE0
    }
    #[doc = "Checks if the value of the field is `ALL`"]
    #[inline(always)]
    pub fn is_all(&self) -> bool {
        *self == MODE_A::ALL
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
            self.bits(variant.into())
        }
    }
    #[doc = "Do not send any instruction."]
    #[inline(always)]
    pub fn no_instr(self) -> &'a mut W {
        self.variant(MODE_A::NOINSTR)
    }
    #[doc = "Send opcode."]
    #[inline(always)]
    pub fn opcode(self) -> &'a mut W {
        self.variant(MODE_A::OPCODE)
    }
    #[doc = "Send opcode, byte0."]
    #[inline(always)]
    pub fn op_byte0(self) -> &'a mut W {
        self.variant(MODE_A::OPBYTE0)
    }
    #[doc = "Send opcode, byte0, byte1."]
    #[inline(always)]
    pub fn all(self) -> &'a mut W {
        self.variant(MODE_A::ALL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Opcode that enters the 32-bit addressing mode."]
    #[inline(always)]
    pub fn opcode(&self) -> OPCODE_R {
        OPCODE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Byte 0 following opcode."]
    #[inline(always)]
    pub fn byte0(&self) -> BYTE0_R {
        BYTE0_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Byte 1 following byte 0."]
    #[inline(always)]
    pub fn byte1(&self) -> BYTE1_R {
        BYTE1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:25 - Extended addressing mode."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bit 26 - Wait for write complete before sending command."]
    #[inline(always)]
    pub fn wipwait(&self) -> WIPWAIT_R {
        WIPWAIT_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Send WREN (write enable opcode 0x06) before instruction."]
    #[inline(always)]
    pub fn wren(&self) -> WREN_R {
        WREN_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Opcode that enters the 32-bit addressing mode."]
    #[inline(always)]
    pub fn opcode(&mut self) -> OPCODE_W {
        OPCODE_W { w: self }
    }
    #[doc = "Bits 8:15 - Byte 0 following opcode."]
    #[inline(always)]
    pub fn byte0(&mut self) -> BYTE0_W {
        BYTE0_W { w: self }
    }
    #[doc = "Bits 16:23 - Byte 1 following byte 0."]
    #[inline(always)]
    pub fn byte1(&mut self) -> BYTE1_W {
        BYTE1_W { w: self }
    }
    #[doc = "Bits 24:25 - Extended addressing mode."]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 26 - Wait for write complete before sending command."]
    #[inline(always)]
    pub fn wipwait(&mut self) -> WIPWAIT_W {
        WIPWAIT_W { w: self }
    }
    #[doc = "Bit 27 - Send WREN (write enable opcode 0x06) before instruction."]
    #[inline(always)]
    pub fn wren(&mut self) -> WREN_W {
        WREN_W { w: self }
    }
}
