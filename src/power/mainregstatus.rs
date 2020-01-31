#[doc = "Reader of register MAINREGSTATUS"]
pub type R = crate::R<u32, super::MAINREGSTATUS>;
#[doc = "Main supply status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAINREGSTATUS_A {
    #[doc = "0: Normal voltage mode. Voltage supplied on VDD."]
    NORMAL = 0,
    #[doc = "1: High voltage mode. Voltage supplied on VDDH."]
    HIGH = 1,
}
impl From<MAINREGSTATUS_A> for bool {
    #[inline(always)]
    fn from(variant: MAINREGSTATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MAINREGSTATUS`"]
pub type MAINREGSTATUS_R = crate::R<bool, MAINREGSTATUS_A>;
impl MAINREGSTATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MAINREGSTATUS_A {
        match self.bits {
            false => MAINREGSTATUS_A::NORMAL,
            true => MAINREGSTATUS_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == MAINREGSTATUS_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == MAINREGSTATUS_A::HIGH
    }
}
impl R {
    #[doc = "Bit 0 - Main supply status"]
    #[inline(always)]
    pub fn mainregstatus(&self) -> MAINREGSTATUS_R {
        MAINREGSTATUS_R::new((self.bits & 0x01) != 0)
    }
}
