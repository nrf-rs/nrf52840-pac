#[doc = "Reader of register HOST_CRYPTOKEY_SEL"]
pub type R = crate::R<u32, super::HOST_CRYPTOKEY_SEL>;
#[doc = "Writer for register HOST_CRYPTOKEY_SEL"]
pub type W = crate::W<u32, super::HOST_CRYPTOKEY_SEL>;
#[doc = "Register HOST_CRYPTOKEY_SEL `reset()`'s with value 0"]
impl crate::ResetValue for super::HOST_CRYPTOKEY_SEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Select the source of the HW key that is used by the AES engine\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HOST_CRYPTOKEY_SEL_A {
    #[doc = "0: Use device root key K_DR from CRYPTOCELL AO power domain"]
    K_DR = 0,
    #[doc = "1: Use hard-coded RTL key K_PRTL"]
    K_PRTL = 1,
    #[doc = "2: Use provided session key"]
    SESSION = 2,
}
impl From<HOST_CRYPTOKEY_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: HOST_CRYPTOKEY_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `HOST_CRYPTOKEY_SEL`"]
pub type HOST_CRYPTOKEY_SEL_R = crate::R<u8, HOST_CRYPTOKEY_SEL_A>;
impl HOST_CRYPTOKEY_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, HOST_CRYPTOKEY_SEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(HOST_CRYPTOKEY_SEL_A::K_DR),
            1 => Val(HOST_CRYPTOKEY_SEL_A::K_PRTL),
            2 => Val(HOST_CRYPTOKEY_SEL_A::SESSION),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `K_DR`"]
    #[inline(always)]
    pub fn is_k_dr(&self) -> bool {
        *self == HOST_CRYPTOKEY_SEL_A::K_DR
    }
    #[doc = "Checks if the value of the field is `K_PRTL`"]
    #[inline(always)]
    pub fn is_k_prtl(&self) -> bool {
        *self == HOST_CRYPTOKEY_SEL_A::K_PRTL
    }
    #[doc = "Checks if the value of the field is `SESSION`"]
    #[inline(always)]
    pub fn is_session(&self) -> bool {
        *self == HOST_CRYPTOKEY_SEL_A::SESSION
    }
}
#[doc = "Write proxy for field `HOST_CRYPTOKEY_SEL`"]
pub struct HOST_CRYPTOKEY_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_CRYPTOKEY_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HOST_CRYPTOKEY_SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Use device root key K_DR from CRYPTOCELL AO power domain"]
    #[inline(always)]
    pub fn k_dr(self) -> &'a mut W {
        self.variant(HOST_CRYPTOKEY_SEL_A::K_DR)
    }
    #[doc = "Use hard-coded RTL key K_PRTL"]
    #[inline(always)]
    pub fn k_prtl(self) -> &'a mut W {
        self.variant(HOST_CRYPTOKEY_SEL_A::K_PRTL)
    }
    #[doc = "Use provided session key"]
    #[inline(always)]
    pub fn session(self) -> &'a mut W {
        self.variant(HOST_CRYPTOKEY_SEL_A::SESSION)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Select the source of the HW key that is used by the AES engine"]
    #[inline(always)]
    pub fn host_cryptokey_sel(&self) -> HOST_CRYPTOKEY_SEL_R {
        HOST_CRYPTOKEY_SEL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Select the source of the HW key that is used by the AES engine"]
    #[inline(always)]
    pub fn host_cryptokey_sel(&mut self) -> HOST_CRYPTOKEY_SEL_W {
        HOST_CRYPTOKEY_SEL_W { w: self }
    }
}
