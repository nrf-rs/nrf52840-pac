#[doc = "Reader of register DPMDUR"]
pub type R = crate::R<u32, super::DPMDUR>;
#[doc = "Writer for register DPMDUR"]
pub type W = crate::W<u32, super::DPMDUR>;
#[doc = "Register DPMDUR `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::DPMDUR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `ENTER`"]
pub type ENTER_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ENTER`"]
pub struct ENTER_W<'a> {
    w: &'a mut W,
}
impl<'a> ENTER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `EXIT`"]
pub type EXIT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `EXIT`"]
pub struct EXIT_W<'a> {
    w: &'a mut W,
}
impl<'a> EXIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Duration needed by external flash to enter DPM. Duration is given as ENTER * 256 * 62.5 ns."]
    #[inline(always)]
    pub fn enter(&self) -> ENTER_R {
        ENTER_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Duration needed by external flash to exit DPM. Duration is given as EXIT * 256 * 62.5 ns."]
    #[inline(always)]
    pub fn exit(&self) -> EXIT_R {
        EXIT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Duration needed by external flash to enter DPM. Duration is given as ENTER * 256 * 62.5 ns."]
    #[inline(always)]
    pub fn enter(&mut self) -> ENTER_W {
        ENTER_W { w: self }
    }
    #[doc = "Bits 16:31 - Duration needed by external flash to exit DPM. Duration is given as EXIT * 256 * 62.5 ns."]
    #[inline(always)]
    pub fn exit(&mut self) -> EXIT_W {
        EXIT_W { w: self }
    }
}
