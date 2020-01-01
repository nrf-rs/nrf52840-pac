#[doc = "Reader of register LEN"]
pub type R = crate::R<u32, super::LEN>;
#[doc = "Writer for register LEN"]
pub type W = crate::W<u32, super::LEN>;
#[doc = "Register LEN `reset()`'s with value 0"]
impl crate::ResetValue for super::LEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "LEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LEN_A {
    #[doc = "0: Erase 4 kB block (flash command 0x20)"]
    _4KB = 0,
    #[doc = "1: Erase 64 kB block (flash command 0xD8)"]
    _64KB = 1,
    #[doc = "2: Erase all (flash command 0xC7)"]
    ALL = 2,
}
impl From<LEN_A> for u8 {
    #[inline(always)]
    fn from(variant: LEN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LEN`"]
pub type LEN_R = crate::R<u8, LEN_A>;
impl LEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LEN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(LEN_A::_4KB),
            1 => Val(LEN_A::_64KB),
            2 => Val(LEN_A::ALL),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_4KB`"]
    #[inline(always)]
    pub fn is_4kb(&self) -> bool {
        *self == LEN_A::_4KB
    }
    #[doc = "Checks if the value of the field is `_64KB`"]
    #[inline(always)]
    pub fn is_64kb(&self) -> bool {
        *self == LEN_A::_64KB
    }
    #[doc = "Checks if the value of the field is `ALL`"]
    #[inline(always)]
    pub fn is_all(&self) -> bool {
        *self == LEN_A::ALL
    }
}
#[doc = "Write proxy for field `LEN`"]
pub struct LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Erase 4 kB block (flash command 0x20)"]
    #[inline(always)]
    pub fn _4kb(self) -> &'a mut W {
        self.variant(LEN_A::_4KB)
    }
    #[doc = "Erase 64 kB block (flash command 0xD8)"]
    #[inline(always)]
    pub fn _64kb(self) -> &'a mut W {
        self.variant(LEN_A::_64KB)
    }
    #[doc = "Erase all (flash command 0xC7)"]
    #[inline(always)]
    pub fn all(self) -> &'a mut W {
        self.variant(LEN_A::ALL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - LEN"]
    #[inline(always)]
    pub fn len(&self) -> LEN_R {
        LEN_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - LEN"]
    #[inline(always)]
    pub fn len(&mut self) -> LEN_W {
        LEN_W { w: self }
    }
}
