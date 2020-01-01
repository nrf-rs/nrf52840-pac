#[doc = "Reader of register PART"]
pub type R = crate::R<u32, super::PART>;
#[doc = "Part code\n\nValue on reset: 337984"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum PART_A {
    #[doc = "337984: nRF52840"]
    N52840 = 337984,
    #[doc = "4294967295: Unspecified"]
    UNSPECIFIED = 4294967295,
}
impl From<PART_A> for u32 {
    #[inline(always)]
    fn from(variant: PART_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PART`"]
pub type PART_R = crate::R<u32, PART_A>;
impl PART_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, PART_A> {
        use crate::Variant::*;
        match self.bits {
            337984 => Val(PART_A::N52840),
            4294967295 => Val(PART_A::UNSPECIFIED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `N52840`"]
    #[inline(always)]
    pub fn is_n52840(&self) -> bool {
        *self == PART_A::N52840
    }
    #[doc = "Checks if the value of the field is `UNSPECIFIED`"]
    #[inline(always)]
    pub fn is_unspecified(&self) -> bool {
        *self == PART_A::UNSPECIFIED
    }
}
impl R {
    #[doc = "Bits 0:31 - Part code"]
    #[inline(always)]
    pub fn part(&self) -> PART_R {
        PART_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
