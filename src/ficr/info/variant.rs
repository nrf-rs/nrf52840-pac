#[doc = "Reader of register VARIANT"]
pub type R = crate::R<u32, super::VARIANT>;
#[doc = "Build code (hardware version and production configuration). Encoded as ASCII.\n\nValue on reset: 4294967295"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum VARIANT_A {
    #[doc = "1094795585: AAAA"]
    AAAA = 1094795585,
    #[doc = "1111572801: BAAA"]
    BAAA = 1111572801,
    #[doc = "1128350017: CAAA"]
    CAAA = 1128350017,
    #[doc = "1094795841: AABA"]
    AABA = 1094795841,
    #[doc = "1094795842: AABB"]
    AABB = 1094795842,
    #[doc = "1094796097: AACA"]
    AACA = 1094796097,
    #[doc = "1094795586: AAAB"]
    AAAB = 1094795586,
    #[doc = "4294967295: Unspecified"]
    UNSPECIFIED = 4294967295,
}
impl From<VARIANT_A> for u32 {
    #[inline(always)]
    fn from(variant: VARIANT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `VARIANT`"]
pub type VARIANT_R = crate::R<u32, VARIANT_A>;
impl VARIANT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, VARIANT_A> {
        use crate::Variant::*;
        match self.bits {
            1094795585 => Val(VARIANT_A::AAAA),
            1111572801 => Val(VARIANT_A::BAAA),
            1128350017 => Val(VARIANT_A::CAAA),
            1094795841 => Val(VARIANT_A::AABA),
            1094795842 => Val(VARIANT_A::AABB),
            1094796097 => Val(VARIANT_A::AACA),
            1094795586 => Val(VARIANT_A::AAAB),
            4294967295 => Val(VARIANT_A::UNSPECIFIED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `AAAA`"]
    #[inline(always)]
    pub fn is_aaaa(&self) -> bool {
        *self == VARIANT_A::AAAA
    }
    #[doc = "Checks if the value of the field is `BAAA`"]
    #[inline(always)]
    pub fn is_baaa(&self) -> bool {
        *self == VARIANT_A::BAAA
    }
    #[doc = "Checks if the value of the field is `CAAA`"]
    #[inline(always)]
    pub fn is_caaa(&self) -> bool {
        *self == VARIANT_A::CAAA
    }
    #[doc = "Checks if the value of the field is `AABA`"]
    #[inline(always)]
    pub fn is_aaba(&self) -> bool {
        *self == VARIANT_A::AABA
    }
    #[doc = "Checks if the value of the field is `AABB`"]
    #[inline(always)]
    pub fn is_aabb(&self) -> bool {
        *self == VARIANT_A::AABB
    }
    #[doc = "Checks if the value of the field is `AACA`"]
    #[inline(always)]
    pub fn is_aaca(&self) -> bool {
        *self == VARIANT_A::AACA
    }
    #[doc = "Checks if the value of the field is `AAAB`"]
    #[inline(always)]
    pub fn is_aaab(&self) -> bool {
        *self == VARIANT_A::AAAB
    }
    #[doc = "Checks if the value of the field is `UNSPECIFIED`"]
    #[inline(always)]
    pub fn is_unspecified(&self) -> bool {
        *self == VARIANT_A::UNSPECIFIED
    }
}
impl R {
    #[doc = "Bits 0:31 - Build code (hardware version and production configuration). Encoded as ASCII."]
    #[inline(always)]
    pub fn variant(&self) -> VARIANT_R {
        VARIANT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
