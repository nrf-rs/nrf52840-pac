#[doc = "Reader of register READYNEXT"]
pub type R = crate::R<u32, super::READYNEXT>;
#[doc = "NVMC can accept a new write operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READYNEXT_A {
    #[doc = "0: NVMC cannot accept any write operation"]
    BUSY = 0,
    #[doc = "1: NVMC is ready"]
    READY = 1,
}
impl From<READYNEXT_A> for bool {
    #[inline(always)]
    fn from(variant: READYNEXT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `READYNEXT`"]
pub type READYNEXT_R = crate::R<bool, READYNEXT_A>;
impl READYNEXT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> READYNEXT_A {
        match self.bits {
            false => READYNEXT_A::BUSY,
            true => READYNEXT_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == READYNEXT_A::BUSY
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == READYNEXT_A::READY
    }
}
impl R {
    #[doc = "Bit 0 - NVMC can accept a new write operation"]
    #[inline(always)]
    pub fn readynext(&self) -> READYNEXT_R {
        READYNEXT_R::new((self.bits & 0x01) != 0)
    }
}
