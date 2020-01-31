#[doc = "Reader of register HOST_IOT_KPRTL_LOCK"]
pub type R = crate::R<u32, super::HOST_IOT_KPRTL_LOCK>;
#[doc = "Writer for register HOST_IOT_KPRTL_LOCK"]
pub type W = crate::W<u32, super::HOST_IOT_KPRTL_LOCK>;
#[doc = "Register HOST_IOT_KPRTL_LOCK `reset()`'s with value 0"]
impl crate::ResetValue for super::HOST_IOT_KPRTL_LOCK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "This register is the K_PRTL lock register. When this register is set, K_PRTL can not be used and a zeroed key will be used instead. The value of this register is saved in the CRYPTOCELL AO power domain.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HOST_IOT_KPRTL_LOCK_A {
    #[doc = "0: K_PRTL can be selected for use from register HOST_CRYPTOKEY_SEL"]
    DISABLED = 0,
    #[doc = "1: K_PRTL has been locked until next power-on reset (POR). If K_PRTL is selected anyway, a zeroed key will be used instead."]
    ENABLED = 1,
}
impl From<HOST_IOT_KPRTL_LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: HOST_IOT_KPRTL_LOCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HOST_IOT_KPRTL_LOCK`"]
pub type HOST_IOT_KPRTL_LOCK_R = crate::R<bool, HOST_IOT_KPRTL_LOCK_A>;
impl HOST_IOT_KPRTL_LOCK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HOST_IOT_KPRTL_LOCK_A {
        match self.bits {
            false => HOST_IOT_KPRTL_LOCK_A::DISABLED,
            true => HOST_IOT_KPRTL_LOCK_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HOST_IOT_KPRTL_LOCK_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HOST_IOT_KPRTL_LOCK_A::ENABLED
    }
}
#[doc = "Write proxy for field `HOST_IOT_KPRTL_LOCK`"]
pub struct HOST_IOT_KPRTL_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_IOT_KPRTL_LOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HOST_IOT_KPRTL_LOCK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "K_PRTL can be selected for use from register HOST_CRYPTOKEY_SEL"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HOST_IOT_KPRTL_LOCK_A::DISABLED)
    }
    #[doc = "K_PRTL has been locked until next power-on reset (POR). If K_PRTL is selected anyway, a zeroed key will be used instead."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HOST_IOT_KPRTL_LOCK_A::ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - This register is the K_PRTL lock register. When this register is set, K_PRTL can not be used and a zeroed key will be used instead. The value of this register is saved in the CRYPTOCELL AO power domain."]
    #[inline(always)]
    pub fn host_iot_kprtl_lock(&self) -> HOST_IOT_KPRTL_LOCK_R {
        HOST_IOT_KPRTL_LOCK_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This register is the K_PRTL lock register. When this register is set, K_PRTL can not be used and a zeroed key will be used instead. The value of this register is saved in the CRYPTOCELL AO power domain."]
    #[inline(always)]
    pub fn host_iot_kprtl_lock(&mut self) -> HOST_IOT_KPRTL_LOCK_W {
        HOST_IOT_KPRTL_LOCK_W { w: self }
    }
}
