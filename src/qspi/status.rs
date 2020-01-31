#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Deep power-down mode (DPM) status of external flash.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPM_A {
    #[doc = "0: External flash is not in DPM."]
    DISABLED = 0,
    #[doc = "1: External flash is in DPM."]
    ENABLED = 1,
}
impl From<DPM_A> for bool {
    #[inline(always)]
    fn from(variant: DPM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DPM`"]
pub type DPM_R = crate::R<bool, DPM_A>;
impl DPM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPM_A {
        match self.bits {
            false => DPM_A::DISABLED,
            true => DPM_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DPM_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DPM_A::ENABLED
    }
}
#[doc = "Ready status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READY_A {
    #[doc = "1: QSPI peripheral is ready. It is allowed to trigger new tasks, writing custom instructions or enter/exit DPM."]
    READY = 1,
    #[doc = "0: QSPI peripheral is busy. It is not allowed to trigger any new tasks, writing custom instructions or enter/exit DPM."]
    BUSY = 0,
}
impl From<READY_A> for bool {
    #[inline(always)]
    fn from(variant: READY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `READY`"]
pub type READY_R = crate::R<bool, READY_A>;
impl READY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> READY_A {
        match self.bits {
            true => READY_A::READY,
            false => READY_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == READY_A::READY
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == READY_A::BUSY
    }
}
#[doc = "Reader of field `SREG`"]
pub type SREG_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 2 - Deep power-down mode (DPM) status of external flash."]
    #[inline(always)]
    pub fn dpm(&self) -> DPM_R {
        DPM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Ready status."]
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 24:31 - Value of external flash device Status Register. When the external flash has two bytes status register this field includes the value of the low byte."]
    #[inline(always)]
    pub fn sreg(&self) -> SREG_R {
        SREG_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
