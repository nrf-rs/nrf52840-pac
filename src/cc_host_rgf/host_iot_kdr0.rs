#[doc = "Reader of register HOST_IOT_KDR0"]
pub type R = crate::R<u32, super::HOST_IOT_KDR0>;
#[doc = "Writer for register HOST_IOT_KDR0"]
pub type W = crate::W<u32, super::HOST_IOT_KDR0>;
#[doc = "Register HOST_IOT_KDR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::HOST_IOT_KDR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HOST_IOT_KDR0`"]
pub type HOST_IOT_KDR0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `HOST_IOT_KDR0`"]
pub struct HOST_IOT_KDR0_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_IOT_KDR0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Write: K_DR bits 31:0 Read: 0x00000000 when 128-bit K_DR key value is not yet retained in the CRYPTOCELL AO power domain Read: 0x00000001 when 128-bit K_DR key value is successfully retained in the CRYPTOCELL AO power domain"]
    #[inline(always)]
    pub fn host_iot_kdr0(&self) -> HOST_IOT_KDR0_R {
        HOST_IOT_KDR0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Write: K_DR bits 31:0 Read: 0x00000000 when 128-bit K_DR key value is not yet retained in the CRYPTOCELL AO power domain Read: 0x00000001 when 128-bit K_DR key value is successfully retained in the CRYPTOCELL AO power domain"]
    #[inline(always)]
    pub fn host_iot_kdr0(&mut self) -> HOST_IOT_KDR0_W {
        HOST_IOT_KDR0_W { w: self }
    }
}
