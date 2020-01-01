#[doc = "Reader of register PRODTEST[%s]"]
pub type R = crate::R<u32, super::PRODTEST>;
#[doc = "Production test signature n\n\nValue on reset: 4294967295"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum PRODTEST_A {
    #[doc = "3141677471: Production tests done"]
    DONE = 3141677471,
    #[doc = "4294967295: Production tests not done"]
    NOTDONE = 4294967295,
}
impl From<PRODTEST_A> for u32 {
    #[inline(always)]
    fn from(variant: PRODTEST_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PRODTEST`"]
pub type PRODTEST_R = crate::R<u32, PRODTEST_A>;
impl PRODTEST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, PRODTEST_A> {
        use crate::Variant::*;
        match self.bits {
            3141677471 => Val(PRODTEST_A::DONE),
            4294967295 => Val(PRODTEST_A::NOTDONE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DONE`"]
    #[inline(always)]
    pub fn is_done(&self) -> bool {
        *self == PRODTEST_A::DONE
    }
    #[doc = "Checks if the value of the field is `NOTDONE`"]
    #[inline(always)]
    pub fn is_not_done(&self) -> bool {
        *self == PRODTEST_A::NOTDONE
    }
}
impl R {
    #[doc = "Bits 0:31 - Production test signature n"]
    #[inline(always)]
    pub fn prodtest(&self) -> PRODTEST_R {
        PRODTEST_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
