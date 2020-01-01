#[doc = "Reader of register HFXODEBOUNCE"]
pub type R = crate::R<u32, super::HFXODEBOUNCE>;
#[doc = "Writer for register HFXODEBOUNCE"]
pub type W = crate::W<u32, super::HFXODEBOUNCE>;
#[doc = "Register HFXODEBOUNCE `reset()`'s with value 0x10"]
impl crate::ResetValue for super::HFXODEBOUNCE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x10
    }
}
#[doc = "HFXO debounce time. Debounce time = HFXODEBOUNCE * 16 us.\n\nValue on reset: 16"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HFXODEBOUNCE_A {
    #[doc = "16: 256 us debounce time. Recommended for TSX-3225, FA-20H and FA-128 crystals."]
    DB256US = 16,
    #[doc = "64: 1024 us debounce time. Recommended for NX1612AA and NX1210AB crystals."]
    DB1024US = 64,
}
impl From<HFXODEBOUNCE_A> for u8 {
    #[inline(always)]
    fn from(variant: HFXODEBOUNCE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `HFXODEBOUNCE`"]
pub type HFXODEBOUNCE_R = crate::R<u8, HFXODEBOUNCE_A>;
impl HFXODEBOUNCE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, HFXODEBOUNCE_A> {
        use crate::Variant::*;
        match self.bits {
            16 => Val(HFXODEBOUNCE_A::DB256US),
            64 => Val(HFXODEBOUNCE_A::DB1024US),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DB256US`"]
    #[inline(always)]
    pub fn is_db256us(&self) -> bool {
        *self == HFXODEBOUNCE_A::DB256US
    }
    #[doc = "Checks if the value of the field is `DB1024US`"]
    #[inline(always)]
    pub fn is_db1024us(&self) -> bool {
        *self == HFXODEBOUNCE_A::DB1024US
    }
}
#[doc = "Write proxy for field `HFXODEBOUNCE`"]
pub struct HFXODEBOUNCE_W<'a> {
    w: &'a mut W,
}
impl<'a> HFXODEBOUNCE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HFXODEBOUNCE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "256 us debounce time. Recommended for TSX-3225, FA-20H and FA-128 crystals."]
    #[inline(always)]
    pub fn db256us(self) -> &'a mut W {
        self.variant(HFXODEBOUNCE_A::DB256US)
    }
    #[doc = "1024 us debounce time. Recommended for NX1612AA and NX1210AB crystals."]
    #[inline(always)]
    pub fn db1024us(self) -> &'a mut W {
        self.variant(HFXODEBOUNCE_A::DB1024US)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - HFXO debounce time. Debounce time = HFXODEBOUNCE * 16 us."]
    #[inline(always)]
    pub fn hfxodebounce(&self) -> HFXODEBOUNCE_R {
        HFXODEBOUNCE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - HFXO debounce time. Debounce time = HFXODEBOUNCE * 16 us."]
    #[inline(always)]
    pub fn hfxodebounce(&mut self) -> HFXODEBOUNCE_W {
        HFXODEBOUNCE_W { w: self }
    }
}
