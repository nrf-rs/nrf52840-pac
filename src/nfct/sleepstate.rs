#[doc = "Reader of register SLEEPSTATE"]
pub type R = crate::R<u32, super::SLEEPSTATE>;
#[doc = "Reflects the sleep state during automatic collision resolution. Set to IDLE by a GOIDLE task. Set to SLEEP_A when a valid SLEEP_REQ frame is received or by a GOSLEEP task.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEEPSTATE_A {
    #[doc = "0: State is IDLE."]
    IDLE = 0,
    #[doc = "1: State is SLEEP_A."]
    SLEEPA = 1,
}
impl From<SLEEPSTATE_A> for bool {
    #[inline(always)]
    fn from(variant: SLEEPSTATE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SLEEPSTATE`"]
pub type SLEEPSTATE_R = crate::R<bool, SLEEPSTATE_A>;
impl SLEEPSTATE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLEEPSTATE_A {
        match self.bits {
            false => SLEEPSTATE_A::IDLE,
            true => SLEEPSTATE_A::SLEEPA,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == SLEEPSTATE_A::IDLE
    }
    #[doc = "Checks if the value of the field is `SLEEPA`"]
    #[inline(always)]
    pub fn is_sleep_a(&self) -> bool {
        *self == SLEEPSTATE_A::SLEEPA
    }
}
impl R {
    #[doc = "Bit 0 - Reflects the sleep state during automatic collision resolution. Set to IDLE by a GOIDLE task. Set to SLEEP_A when a valid SLEEP_REQ frame is received or by a GOSLEEP task."]
    #[inline(always)]
    pub fn sleepstate(&self) -> SLEEPSTATE_R {
        SLEEPSTATE_R::new((self.bits & 0x01) != 0)
    }
}
