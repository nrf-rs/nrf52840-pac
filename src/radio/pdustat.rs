#[doc = "Reader of register PDUSTAT"]
pub type R = crate::R<u32, super::PDUSTAT>;
#[doc = "Status on payload length vs. PCNF1.MAXLEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDUSTAT_A {
    #[doc = "0: Payload less than PCNF1.MAXLEN"]
    LESSTHAN = 0,
    #[doc = "1: Payload greater than PCNF1.MAXLEN"]
    GREATERTHAN = 1,
}
impl From<PDUSTAT_A> for bool {
    #[inline(always)]
    fn from(variant: PDUSTAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PDUSTAT`"]
pub type PDUSTAT_R = crate::R<bool, PDUSTAT_A>;
impl PDUSTAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDUSTAT_A {
        match self.bits {
            false => PDUSTAT_A::LESSTHAN,
            true => PDUSTAT_A::GREATERTHAN,
        }
    }
    #[doc = "Checks if the value of the field is `LESSTHAN`"]
    #[inline(always)]
    pub fn is_less_than(&self) -> bool {
        *self == PDUSTAT_A::LESSTHAN
    }
    #[doc = "Checks if the value of the field is `GREATERTHAN`"]
    #[inline(always)]
    pub fn is_greater_than(&self) -> bool {
        *self == PDUSTAT_A::GREATERTHAN
    }
}
#[doc = "Status on what rate packet is received with in Long Range\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CISTAT_A {
    #[doc = "0: Frame is received at 125kbps"]
    LR125KBIT = 0,
    #[doc = "1: Frame is received at 500kbps"]
    LR500KBIT = 1,
}
impl From<CISTAT_A> for u8 {
    #[inline(always)]
    fn from(variant: CISTAT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CISTAT`"]
pub type CISTAT_R = crate::R<u8, CISTAT_A>;
impl CISTAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CISTAT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CISTAT_A::LR125KBIT),
            1 => Val(CISTAT_A::LR500KBIT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LR125KBIT`"]
    #[inline(always)]
    pub fn is_lr125kbit(&self) -> bool {
        *self == CISTAT_A::LR125KBIT
    }
    #[doc = "Checks if the value of the field is `LR500KBIT`"]
    #[inline(always)]
    pub fn is_lr500kbit(&self) -> bool {
        *self == CISTAT_A::LR500KBIT
    }
}
impl R {
    #[doc = "Bit 0 - Status on payload length vs. PCNF1.MAXLEN"]
    #[inline(always)]
    pub fn pdustat(&self) -> PDUSTAT_R {
        PDUSTAT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - Status on what rate packet is received with in Long Range"]
    #[inline(always)]
    pub fn cistat(&self) -> CISTAT_R {
        CISTAT_R::new(((self.bits >> 1) & 0x03) as u8)
    }
}
