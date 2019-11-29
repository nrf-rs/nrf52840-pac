#[doc = "Writer for register HOST_IOT_KDR2"]
pub type W = crate::W<u32, super::HOST_IOT_KDR2>;
#[doc = "Register HOST_IOT_KDR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::HOST_IOT_KDR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `HOST_IOT_KDR2`"]
pub struct HOST_IOT_KDR2_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_IOT_KDR2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - K_DR bits 95:64"]
    #[inline(always)]
    pub fn host_iot_kdr2(&mut self) -> HOST_IOT_KDR2_W {
        HOST_IOT_KDR2_W { w: self }
    }
}
