#[doc = "Reader of register TXPOWER"]
pub type R = crate::R<u32, super::TXPOWER>;
#[doc = "Writer for register TXPOWER"]
pub type W = crate::W<u32, super::TXPOWER>;
#[doc = "Register TXPOWER `reset()`'s with value 0"]
impl crate::ResetValue for super::TXPOWER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "RADIO output power\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TXPOWER_A {
    #[doc = "8: +8 dBm"]
    POS8DBM = 8,
    #[doc = "7: +7 dBm"]
    POS7DBM = 7,
    #[doc = "6: +6 dBm"]
    POS6DBM = 6,
    #[doc = "5: +5 dBm"]
    POS5DBM = 5,
    #[doc = "4: +4 dBm"]
    POS4DBM = 4,
    #[doc = "3: +3 dBm"]
    POS3DBM = 3,
    #[doc = "2: +2 dBm"]
    POS2DBM = 2,
    #[doc = "0: 0 dBm"]
    _0DBM = 0,
    #[doc = "252: -4 dBm"]
    NEG4DBM = 252,
    #[doc = "248: -8 dBm"]
    NEG8DBM = 248,
    #[doc = "244: -12 dBm"]
    NEG12DBM = 244,
    #[doc = "240: -16 dBm"]
    NEG16DBM = 240,
    #[doc = "236: -20 dBm"]
    NEG20DBM = 236,
    #[doc = "255: Deprecated enumerator -  -40 dBm"]
    NEG30DBM = 255,
    #[doc = "216: -40 dBm"]
    NEG40DBM = 216,
}
impl From<TXPOWER_A> for u8 {
    #[inline(always)]
    fn from(variant: TXPOWER_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TXPOWER`"]
pub type TXPOWER_R = crate::R<u8, TXPOWER_A>;
impl TXPOWER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TXPOWER_A> {
        use crate::Variant::*;
        match self.bits {
            8 => Val(TXPOWER_A::POS8DBM),
            7 => Val(TXPOWER_A::POS7DBM),
            6 => Val(TXPOWER_A::POS6DBM),
            5 => Val(TXPOWER_A::POS5DBM),
            4 => Val(TXPOWER_A::POS4DBM),
            3 => Val(TXPOWER_A::POS3DBM),
            2 => Val(TXPOWER_A::POS2DBM),
            0 => Val(TXPOWER_A::_0DBM),
            252 => Val(TXPOWER_A::NEG4DBM),
            248 => Val(TXPOWER_A::NEG8DBM),
            244 => Val(TXPOWER_A::NEG12DBM),
            240 => Val(TXPOWER_A::NEG16DBM),
            236 => Val(TXPOWER_A::NEG20DBM),
            255 => Val(TXPOWER_A::NEG30DBM),
            216 => Val(TXPOWER_A::NEG40DBM),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `POS8DBM`"]
    #[inline(always)]
    pub fn is_pos8d_bm(&self) -> bool {
        *self == TXPOWER_A::POS8DBM
    }
    #[doc = "Checks if the value of the field is `POS7DBM`"]
    #[inline(always)]
    pub fn is_pos7d_bm(&self) -> bool {
        *self == TXPOWER_A::POS7DBM
    }
    #[doc = "Checks if the value of the field is `POS6DBM`"]
    #[inline(always)]
    pub fn is_pos6d_bm(&self) -> bool {
        *self == TXPOWER_A::POS6DBM
    }
    #[doc = "Checks if the value of the field is `POS5DBM`"]
    #[inline(always)]
    pub fn is_pos5d_bm(&self) -> bool {
        *self == TXPOWER_A::POS5DBM
    }
    #[doc = "Checks if the value of the field is `POS4DBM`"]
    #[inline(always)]
    pub fn is_pos4d_bm(&self) -> bool {
        *self == TXPOWER_A::POS4DBM
    }
    #[doc = "Checks if the value of the field is `POS3DBM`"]
    #[inline(always)]
    pub fn is_pos3d_bm(&self) -> bool {
        *self == TXPOWER_A::POS3DBM
    }
    #[doc = "Checks if the value of the field is `POS2DBM`"]
    #[inline(always)]
    pub fn is_pos2d_bm(&self) -> bool {
        *self == TXPOWER_A::POS2DBM
    }
    #[doc = "Checks if the value of the field is `_0DBM`"]
    #[inline(always)]
    pub fn is_0d_bm(&self) -> bool {
        *self == TXPOWER_A::_0DBM
    }
    #[doc = "Checks if the value of the field is `NEG4DBM`"]
    #[inline(always)]
    pub fn is_neg4d_bm(&self) -> bool {
        *self == TXPOWER_A::NEG4DBM
    }
    #[doc = "Checks if the value of the field is `NEG8DBM`"]
    #[inline(always)]
    pub fn is_neg8d_bm(&self) -> bool {
        *self == TXPOWER_A::NEG8DBM
    }
    #[doc = "Checks if the value of the field is `NEG12DBM`"]
    #[inline(always)]
    pub fn is_neg12d_bm(&self) -> bool {
        *self == TXPOWER_A::NEG12DBM
    }
    #[doc = "Checks if the value of the field is `NEG16DBM`"]
    #[inline(always)]
    pub fn is_neg16d_bm(&self) -> bool {
        *self == TXPOWER_A::NEG16DBM
    }
    #[doc = "Checks if the value of the field is `NEG20DBM`"]
    #[inline(always)]
    pub fn is_neg20d_bm(&self) -> bool {
        *self == TXPOWER_A::NEG20DBM
    }
    #[doc = "Checks if the value of the field is `NEG30DBM`"]
    #[inline(always)]
    pub fn is_neg30d_bm(&self) -> bool {
        *self == TXPOWER_A::NEG30DBM
    }
    #[doc = "Checks if the value of the field is `NEG40DBM`"]
    #[inline(always)]
    pub fn is_neg40d_bm(&self) -> bool {
        *self == TXPOWER_A::NEG40DBM
    }
}
#[doc = "Write proxy for field `TXPOWER`"]
pub struct TXPOWER_W<'a> {
    w: &'a mut W,
}
impl<'a> TXPOWER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXPOWER_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "+8 dBm"]
    #[inline(always)]
    pub fn pos8d_bm(self) -> &'a mut W {
        self.variant(TXPOWER_A::POS8DBM)
    }
    #[doc = "+7 dBm"]
    #[inline(always)]
    pub fn pos7d_bm(self) -> &'a mut W {
        self.variant(TXPOWER_A::POS7DBM)
    }
    #[doc = "+6 dBm"]
    #[inline(always)]
    pub fn pos6d_bm(self) -> &'a mut W {
        self.variant(TXPOWER_A::POS6DBM)
    }
    #[doc = "+5 dBm"]
    #[inline(always)]
    pub fn pos5d_bm(self) -> &'a mut W {
        self.variant(TXPOWER_A::POS5DBM)
    }
    #[doc = "+4 dBm"]
    #[inline(always)]
    pub fn pos4d_bm(self) -> &'a mut W {
        self.variant(TXPOWER_A::POS4DBM)
    }
    #[doc = "+3 dBm"]
    #[inline(always)]
    pub fn pos3d_bm(self) -> &'a mut W {
        self.variant(TXPOWER_A::POS3DBM)
    }
    #[doc = "+2 dBm"]
    #[inline(always)]
    pub fn pos2d_bm(self) -> &'a mut W {
        self.variant(TXPOWER_A::POS2DBM)
    }
    #[doc = "0 dBm"]
    #[inline(always)]
    pub fn _0d_bm(self) -> &'a mut W {
        self.variant(TXPOWER_A::_0DBM)
    }
    #[doc = "-4 dBm"]
    #[inline(always)]
    pub fn neg4d_bm(self) -> &'a mut W {
        self.variant(TXPOWER_A::NEG4DBM)
    }
    #[doc = "-8 dBm"]
    #[inline(always)]
    pub fn neg8d_bm(self) -> &'a mut W {
        self.variant(TXPOWER_A::NEG8DBM)
    }
    #[doc = "-12 dBm"]
    #[inline(always)]
    pub fn neg12d_bm(self) -> &'a mut W {
        self.variant(TXPOWER_A::NEG12DBM)
    }
    #[doc = "-16 dBm"]
    #[inline(always)]
    pub fn neg16d_bm(self) -> &'a mut W {
        self.variant(TXPOWER_A::NEG16DBM)
    }
    #[doc = "-20 dBm"]
    #[inline(always)]
    pub fn neg20d_bm(self) -> &'a mut W {
        self.variant(TXPOWER_A::NEG20DBM)
    }
    #[doc = "Deprecated enumerator - -40 dBm"]
    #[inline(always)]
    pub fn neg30d_bm(self) -> &'a mut W {
        self.variant(TXPOWER_A::NEG30DBM)
    }
    #[doc = "-40 dBm"]
    #[inline(always)]
    pub fn neg40d_bm(self) -> &'a mut W {
        self.variant(TXPOWER_A::NEG40DBM)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - RADIO output power"]
    #[inline(always)]
    pub fn txpower(&self) -> TXPOWER_R {
        TXPOWER_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - RADIO output power"]
    #[inline(always)]
    pub fn txpower(&mut self) -> TXPOWER_W {
        TXPOWER_W { w: self }
    }
}
