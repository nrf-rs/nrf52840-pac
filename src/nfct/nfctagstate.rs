#[doc = "Reader of register NFCTAGSTATE"]
pub type R = crate::R<u32, super::NFCTAGSTATE>;
#[doc = "NfcTag state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum NFCTAGSTATE_A {
    #[doc = "0: Disabled or sense"]
    DISABLED = 0,
    #[doc = "2: RampUp"]
    RAMPUP = 2,
    #[doc = "3: Idle"]
    IDLE = 3,
    #[doc = "4: Receive"]
    RECEIVE = 4,
    #[doc = "5: FrameDelay"]
    FRAMEDELAY = 5,
    #[doc = "6: Transmit"]
    TRANSMIT = 6,
}
impl From<NFCTAGSTATE_A> for u8 {
    #[inline(always)]
    fn from(variant: NFCTAGSTATE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `NFCTAGSTATE`"]
pub type NFCTAGSTATE_R = crate::R<u8, NFCTAGSTATE_A>;
impl NFCTAGSTATE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, NFCTAGSTATE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(NFCTAGSTATE_A::DISABLED),
            2 => Val(NFCTAGSTATE_A::RAMPUP),
            3 => Val(NFCTAGSTATE_A::IDLE),
            4 => Val(NFCTAGSTATE_A::RECEIVE),
            5 => Val(NFCTAGSTATE_A::FRAMEDELAY),
            6 => Val(NFCTAGSTATE_A::TRANSMIT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == NFCTAGSTATE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `RAMPUP`"]
    #[inline(always)]
    pub fn is_ramp_up(&self) -> bool {
        *self == NFCTAGSTATE_A::RAMPUP
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == NFCTAGSTATE_A::IDLE
    }
    #[doc = "Checks if the value of the field is `RECEIVE`"]
    #[inline(always)]
    pub fn is_receive(&self) -> bool {
        *self == NFCTAGSTATE_A::RECEIVE
    }
    #[doc = "Checks if the value of the field is `FRAMEDELAY`"]
    #[inline(always)]
    pub fn is_frame_delay(&self) -> bool {
        *self == NFCTAGSTATE_A::FRAMEDELAY
    }
    #[doc = "Checks if the value of the field is `TRANSMIT`"]
    #[inline(always)]
    pub fn is_transmit(&self) -> bool {
        *self == NFCTAGSTATE_A::TRANSMIT
    }
}
impl R {
    #[doc = "Bits 0:2 - NfcTag state"]
    #[inline(always)]
    pub fn nfctagstate(&self) -> NFCTAGSTATE_R {
        NFCTAGSTATE_R::new((self.bits & 0x07) as u8)
    }
}
