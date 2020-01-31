#[doc = "Reader of register IFCONFIG0"]
pub type R = crate::R<u32, super::IFCONFIG0>;
#[doc = "Writer for register IFCONFIG0"]
pub type W = crate::W<u32, super::IFCONFIG0>;
#[doc = "Register IFCONFIG0 `reset()`'s with value 0"]
impl crate::ResetValue for super::IFCONFIG0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Configure number of data lines and opcode used for reading.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum READOC_A {
    #[doc = "0: Single data line SPI. FAST_READ (opcode 0x0B)."]
    FASTREAD = 0,
    #[doc = "1: Dual data line SPI. READ2O (opcode 0x3B)."]
    READ2O = 1,
    #[doc = "2: Dual data line SPI. READ2IO (opcode 0xBB)."]
    READ2IO = 2,
    #[doc = "3: Quad data line SPI. READ4O (opcode 0x6B)."]
    READ4O = 3,
    #[doc = "4: Quad data line SPI. READ4IO (opcode 0xEB)."]
    READ4IO = 4,
}
impl From<READOC_A> for u8 {
    #[inline(always)]
    fn from(variant: READOC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `READOC`"]
pub type READOC_R = crate::R<u8, READOC_A>;
impl READOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, READOC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(READOC_A::FASTREAD),
            1 => Val(READOC_A::READ2O),
            2 => Val(READOC_A::READ2IO),
            3 => Val(READOC_A::READ4O),
            4 => Val(READOC_A::READ4IO),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FASTREAD`"]
    #[inline(always)]
    pub fn is_fastread(&self) -> bool {
        *self == READOC_A::FASTREAD
    }
    #[doc = "Checks if the value of the field is `READ2O`"]
    #[inline(always)]
    pub fn is_read2o(&self) -> bool {
        *self == READOC_A::READ2O
    }
    #[doc = "Checks if the value of the field is `READ2IO`"]
    #[inline(always)]
    pub fn is_read2io(&self) -> bool {
        *self == READOC_A::READ2IO
    }
    #[doc = "Checks if the value of the field is `READ4O`"]
    #[inline(always)]
    pub fn is_read4o(&self) -> bool {
        *self == READOC_A::READ4O
    }
    #[doc = "Checks if the value of the field is `READ4IO`"]
    #[inline(always)]
    pub fn is_read4io(&self) -> bool {
        *self == READOC_A::READ4IO
    }
}
#[doc = "Write proxy for field `READOC`"]
pub struct READOC_W<'a> {
    w: &'a mut W,
}
impl<'a> READOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: READOC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Single data line SPI. FAST_READ (opcode 0x0B)."]
    #[inline(always)]
    pub fn fastread(self) -> &'a mut W {
        self.variant(READOC_A::FASTREAD)
    }
    #[doc = "Dual data line SPI. READ2O (opcode 0x3B)."]
    #[inline(always)]
    pub fn read2o(self) -> &'a mut W {
        self.variant(READOC_A::READ2O)
    }
    #[doc = "Dual data line SPI. READ2IO (opcode 0xBB)."]
    #[inline(always)]
    pub fn read2io(self) -> &'a mut W {
        self.variant(READOC_A::READ2IO)
    }
    #[doc = "Quad data line SPI. READ4O (opcode 0x6B)."]
    #[inline(always)]
    pub fn read4o(self) -> &'a mut W {
        self.variant(READOC_A::READ4O)
    }
    #[doc = "Quad data line SPI. READ4IO (opcode 0xEB)."]
    #[inline(always)]
    pub fn read4io(self) -> &'a mut W {
        self.variant(READOC_A::READ4IO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Configure number of data lines and opcode used for writing.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WRITEOC_A {
    #[doc = "0: Single data line SPI. PP (opcode 0x02)."]
    PP = 0,
    #[doc = "1: Dual data line SPI. PP2O (opcode 0xA2)."]
    PP2O = 1,
    #[doc = "2: Quad data line SPI. PP4O (opcode 0x32)."]
    PP4O = 2,
    #[doc = "3: Quad data line SPI. PP4IO (opcode 0x38)."]
    PP4IO = 3,
}
impl From<WRITEOC_A> for u8 {
    #[inline(always)]
    fn from(variant: WRITEOC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `WRITEOC`"]
pub type WRITEOC_R = crate::R<u8, WRITEOC_A>;
impl WRITEOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, WRITEOC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(WRITEOC_A::PP),
            1 => Val(WRITEOC_A::PP2O),
            2 => Val(WRITEOC_A::PP4O),
            3 => Val(WRITEOC_A::PP4IO),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PP`"]
    #[inline(always)]
    pub fn is_pp(&self) -> bool {
        *self == WRITEOC_A::PP
    }
    #[doc = "Checks if the value of the field is `PP2O`"]
    #[inline(always)]
    pub fn is_pp2o(&self) -> bool {
        *self == WRITEOC_A::PP2O
    }
    #[doc = "Checks if the value of the field is `PP4O`"]
    #[inline(always)]
    pub fn is_pp4o(&self) -> bool {
        *self == WRITEOC_A::PP4O
    }
    #[doc = "Checks if the value of the field is `PP4IO`"]
    #[inline(always)]
    pub fn is_pp4io(&self) -> bool {
        *self == WRITEOC_A::PP4IO
    }
}
#[doc = "Write proxy for field `WRITEOC`"]
pub struct WRITEOC_W<'a> {
    w: &'a mut W,
}
impl<'a> WRITEOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WRITEOC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Single data line SPI. PP (opcode 0x02)."]
    #[inline(always)]
    pub fn pp(self) -> &'a mut W {
        self.variant(WRITEOC_A::PP)
    }
    #[doc = "Dual data line SPI. PP2O (opcode 0xA2)."]
    #[inline(always)]
    pub fn pp2o(self) -> &'a mut W {
        self.variant(WRITEOC_A::PP2O)
    }
    #[doc = "Quad data line SPI. PP4O (opcode 0x32)."]
    #[inline(always)]
    pub fn pp4o(self) -> &'a mut W {
        self.variant(WRITEOC_A::PP4O)
    }
    #[doc = "Quad data line SPI. PP4IO (opcode 0x38)."]
    #[inline(always)]
    pub fn pp4io(self) -> &'a mut W {
        self.variant(WRITEOC_A::PP4IO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u32) & 0x07) << 3);
        self.w
    }
}
#[doc = "Addressing mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDRMODE_A {
    #[doc = "0: 24-bit addressing."]
    _24BIT = 0,
    #[doc = "1: 32-bit addressing."]
    _32BIT = 1,
}
impl From<ADDRMODE_A> for bool {
    #[inline(always)]
    fn from(variant: ADDRMODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADDRMODE`"]
pub type ADDRMODE_R = crate::R<bool, ADDRMODE_A>;
impl ADDRMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDRMODE_A {
        match self.bits {
            false => ADDRMODE_A::_24BIT,
            true => ADDRMODE_A::_32BIT,
        }
    }
    #[doc = "Checks if the value of the field is `_24BIT`"]
    #[inline(always)]
    pub fn is_24bit(&self) -> bool {
        *self == ADDRMODE_A::_24BIT
    }
    #[doc = "Checks if the value of the field is `_32BIT`"]
    #[inline(always)]
    pub fn is_32bit(&self) -> bool {
        *self == ADDRMODE_A::_32BIT
    }
}
#[doc = "Write proxy for field `ADDRMODE`"]
pub struct ADDRMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADDRMODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "24-bit addressing."]
    #[inline(always)]
    pub fn _24bit(self) -> &'a mut W {
        self.variant(ADDRMODE_A::_24BIT)
    }
    #[doc = "32-bit addressing."]
    #[inline(always)]
    pub fn _32bit(self) -> &'a mut W {
        self.variant(ADDRMODE_A::_32BIT)
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
#[doc = "Enable deep power-down mode (DPM) feature.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPMENABLE_A {
    #[doc = "0: Disable DPM feature."]
    DISABLE = 0,
    #[doc = "1: Enable DPM feature."]
    ENABLE = 1,
}
impl From<DPMENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: DPMENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DPMENABLE`"]
pub type DPMENABLE_R = crate::R<bool, DPMENABLE_A>;
impl DPMENABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPMENABLE_A {
        match self.bits {
            false => DPMENABLE_A::DISABLE,
            true => DPMENABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DPMENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DPMENABLE_A::ENABLE
    }
}
#[doc = "Write proxy for field `DPMENABLE`"]
pub struct DPMENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> DPMENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DPMENABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable DPM feature."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DPMENABLE_A::DISABLE)
    }
    #[doc = "Enable DPM feature."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DPMENABLE_A::ENABLE)
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
#[doc = "Page size for commands PP, PP2O, PP4O and PP4IO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PPSIZE_A {
    #[doc = "0: 256 bytes."]
    _256BYTES = 0,
    #[doc = "1: 512 bytes."]
    _512BYTES = 1,
}
impl From<PPSIZE_A> for bool {
    #[inline(always)]
    fn from(variant: PPSIZE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PPSIZE`"]
pub type PPSIZE_R = crate::R<bool, PPSIZE_A>;
impl PPSIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PPSIZE_A {
        match self.bits {
            false => PPSIZE_A::_256BYTES,
            true => PPSIZE_A::_512BYTES,
        }
    }
    #[doc = "Checks if the value of the field is `_256BYTES`"]
    #[inline(always)]
    pub fn is_256bytes(&self) -> bool {
        *self == PPSIZE_A::_256BYTES
    }
    #[doc = "Checks if the value of the field is `_512BYTES`"]
    #[inline(always)]
    pub fn is_512bytes(&self) -> bool {
        *self == PPSIZE_A::_512BYTES
    }
}
#[doc = "Write proxy for field `PPSIZE`"]
pub struct PPSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> PPSIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PPSIZE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "256 bytes."]
    #[inline(always)]
    pub fn _256bytes(self) -> &'a mut W {
        self.variant(PPSIZE_A::_256BYTES)
    }
    #[doc = "512 bytes."]
    #[inline(always)]
    pub fn _512bytes(self) -> &'a mut W {
        self.variant(PPSIZE_A::_512BYTES)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Configure number of data lines and opcode used for reading."]
    #[inline(always)]
    pub fn readoc(&self) -> READOC_R {
        READOC_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:5 - Configure number of data lines and opcode used for writing."]
    #[inline(always)]
    pub fn writeoc(&self) -> WRITEOC_R {
        WRITEOC_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bit 6 - Addressing mode."]
    #[inline(always)]
    pub fn addrmode(&self) -> ADDRMODE_R {
        ADDRMODE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable deep power-down mode (DPM) feature."]
    #[inline(always)]
    pub fn dpmenable(&self) -> DPMENABLE_R {
        DPMENABLE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Page size for commands PP, PP2O, PP4O and PP4IO."]
    #[inline(always)]
    pub fn ppsize(&self) -> PPSIZE_R {
        PPSIZE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Configure number of data lines and opcode used for reading."]
    #[inline(always)]
    pub fn readoc(&mut self) -> READOC_W {
        READOC_W { w: self }
    }
    #[doc = "Bits 3:5 - Configure number of data lines and opcode used for writing."]
    #[inline(always)]
    pub fn writeoc(&mut self) -> WRITEOC_W {
        WRITEOC_W { w: self }
    }
    #[doc = "Bit 6 - Addressing mode."]
    #[inline(always)]
    pub fn addrmode(&mut self) -> ADDRMODE_W {
        ADDRMODE_W { w: self }
    }
    #[doc = "Bit 7 - Enable deep power-down mode (DPM) feature."]
    #[inline(always)]
    pub fn dpmenable(&mut self) -> DPMENABLE_W {
        DPMENABLE_W { w: self }
    }
    #[doc = "Bit 12 - Page size for commands PP, PP2O, PP4O and PP4IO."]
    #[inline(always)]
    pub fn ppsize(&mut self) -> PPSIZE_W {
        PPSIZE_W { w: self }
    }
}
