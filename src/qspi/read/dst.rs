#[doc = "Reader of register DST"]
pub type R = crate::R<u32, super::DST>;
#[doc = "Writer for register DST"]
pub type W = crate::W<u32, super::DST>;
#[doc = "Register DST `reset()`'s with value 0"]
impl crate::ResetValue for super::DST {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DST`"]
pub type DST_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DST`"]
pub struct DST_W<'a> {
    w: &'a mut W,
}
impl<'a> DST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Word-aligned RAM destination address."]
    #[inline(always)]
    pub fn dst(&self) -> DST_R {
        DST_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Word-aligned RAM destination address."]
    #[inline(always)]
    pub fn dst(&mut self) -> DST_W {
        DST_W { w: self }
    }
}
