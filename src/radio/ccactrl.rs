#[doc = "Reader of register CCACTRL"]
pub type R = crate::R<u32, super::CCACTRL>;
#[doc = "Writer for register CCACTRL"]
pub type W = crate::W<u32, super::CCACTRL>;
#[doc = "Register CCACTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CCACTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "CCA mode of operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CCAMODE_A {
    #[doc = "0: Energy above threshold"]
    EDMODE = 0,
    #[doc = "1: Carrier seen"]
    CARRIERMODE = 1,
    #[doc = "2: Energy above threshold AND carrier seen"]
    CARRIERANDEDMODE = 2,
    #[doc = "3: Energy above threshold OR carrier seen"]
    CARRIEROREDMODE = 3,
    #[doc = "4: Energy above threshold test mode that will abort when first ED measurement over threshold is seen. No averaging."]
    EDMODETEST1 = 4,
}
impl From<CCAMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: CCAMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CCAMODE`"]
pub type CCAMODE_R = crate::R<u8, CCAMODE_A>;
impl CCAMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CCAMODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CCAMODE_A::EDMODE),
            1 => Val(CCAMODE_A::CARRIERMODE),
            2 => Val(CCAMODE_A::CARRIERANDEDMODE),
            3 => Val(CCAMODE_A::CARRIEROREDMODE),
            4 => Val(CCAMODE_A::EDMODETEST1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `EDMODE`"]
    #[inline(always)]
    pub fn is_ed_mode(&self) -> bool {
        *self == CCAMODE_A::EDMODE
    }
    #[doc = "Checks if the value of the field is `CARRIERMODE`"]
    #[inline(always)]
    pub fn is_carrier_mode(&self) -> bool {
        *self == CCAMODE_A::CARRIERMODE
    }
    #[doc = "Checks if the value of the field is `CARRIERANDEDMODE`"]
    #[inline(always)]
    pub fn is_carrier_and_ed_mode(&self) -> bool {
        *self == CCAMODE_A::CARRIERANDEDMODE
    }
    #[doc = "Checks if the value of the field is `CARRIEROREDMODE`"]
    #[inline(always)]
    pub fn is_carrier_or_ed_mode(&self) -> bool {
        *self == CCAMODE_A::CARRIEROREDMODE
    }
    #[doc = "Checks if the value of the field is `EDMODETEST1`"]
    #[inline(always)]
    pub fn is_ed_mode_test1(&self) -> bool {
        *self == CCAMODE_A::EDMODETEST1
    }
}
#[doc = "Write proxy for field `CCAMODE`"]
pub struct CCAMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CCAMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCAMODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Energy above threshold"]
    #[inline(always)]
    pub fn ed_mode(self) -> &'a mut W {
        self.variant(CCAMODE_A::EDMODE)
    }
    #[doc = "Carrier seen"]
    #[inline(always)]
    pub fn carrier_mode(self) -> &'a mut W {
        self.variant(CCAMODE_A::CARRIERMODE)
    }
    #[doc = "Energy above threshold AND carrier seen"]
    #[inline(always)]
    pub fn carrier_and_ed_mode(self) -> &'a mut W {
        self.variant(CCAMODE_A::CARRIERANDEDMODE)
    }
    #[doc = "Energy above threshold OR carrier seen"]
    #[inline(always)]
    pub fn carrier_or_ed_mode(self) -> &'a mut W {
        self.variant(CCAMODE_A::CARRIEROREDMODE)
    }
    #[doc = "Energy above threshold test mode that will abort when first ED measurement over threshold is seen. No averaging."]
    #[inline(always)]
    pub fn ed_mode_test1(self) -> &'a mut W {
        self.variant(CCAMODE_A::EDMODETEST1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `CCAEDTHRES`"]
pub type CCAEDTHRES_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CCAEDTHRES`"]
pub struct CCAEDTHRES_W<'a> {
    w: &'a mut W,
}
impl<'a> CCAEDTHRES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `CCACORRTHRES`"]
pub type CCACORRTHRES_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CCACORRTHRES`"]
pub struct CCACORRTHRES_W<'a> {
    w: &'a mut W,
}
impl<'a> CCACORRTHRES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `CCACORRCNT`"]
pub type CCACORRCNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CCACORRCNT`"]
pub struct CCACORRCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> CCACORRCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - CCA mode of operation"]
    #[inline(always)]
    pub fn ccamode(&self) -> CCAMODE_R {
        CCAMODE_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 8:15 - CCA energy busy threshold. Used in all the CCA modes except CarrierMode."]
    #[inline(always)]
    pub fn ccaedthres(&self) -> CCAEDTHRES_R {
        CCAEDTHRES_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - CCA correlator busy threshold. Only relevant to CarrierMode, CarrierAndEdMode and CarrierOrEdMode."]
    #[inline(always)]
    pub fn ccacorrthres(&self) -> CCACORRTHRES_R {
        CCACORRTHRES_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Limit for occurances above CCACORRTHRES. When not equal to zero the corrolator based signal detect is enabled."]
    #[inline(always)]
    pub fn ccacorrcnt(&self) -> CCACORRCNT_R {
        CCACORRCNT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - CCA mode of operation"]
    #[inline(always)]
    pub fn ccamode(&mut self) -> CCAMODE_W {
        CCAMODE_W { w: self }
    }
    #[doc = "Bits 8:15 - CCA energy busy threshold. Used in all the CCA modes except CarrierMode."]
    #[inline(always)]
    pub fn ccaedthres(&mut self) -> CCAEDTHRES_W {
        CCAEDTHRES_W { w: self }
    }
    #[doc = "Bits 16:23 - CCA correlator busy threshold. Only relevant to CarrierMode, CarrierAndEdMode and CarrierOrEdMode."]
    #[inline(always)]
    pub fn ccacorrthres(&mut self) -> CCACORRTHRES_W {
        CCACORRTHRES_W { w: self }
    }
    #[doc = "Bits 24:31 - Limit for occurances above CCACORRTHRES. When not equal to zero the corrolator based signal detect is enabled."]
    #[inline(always)]
    pub fn ccacorrcnt(&mut self) -> CCACORRCNT_W {
        CCACORRCNT_W { w: self }
    }
}
