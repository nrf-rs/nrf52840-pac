#[doc = "Reader of register USBREGSTATUS"]
pub type R = crate::R<u32, super::USBREGSTATUS>;
#[doc = "VBUS input detection status (USBDETECTED and USBREMOVED events are derived from this information)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VBUSDETECT_A {
    #[doc = "0: VBUS voltage below valid threshold"]
    NOVBUS = 0,
    #[doc = "1: VBUS voltage above valid threshold"]
    VBUSPRESENT = 1,
}
impl From<VBUSDETECT_A> for bool {
    #[inline(always)]
    fn from(variant: VBUSDETECT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VBUSDETECT`"]
pub type VBUSDETECT_R = crate::R<bool, VBUSDETECT_A>;
impl VBUSDETECT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VBUSDETECT_A {
        match self.bits {
            false => VBUSDETECT_A::NOVBUS,
            true => VBUSDETECT_A::VBUSPRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOVBUS`"]
    #[inline(always)]
    pub fn is_no_vbus(&self) -> bool {
        *self == VBUSDETECT_A::NOVBUS
    }
    #[doc = "Checks if the value of the field is `VBUSPRESENT`"]
    #[inline(always)]
    pub fn is_vbus_present(&self) -> bool {
        *self == VBUSDETECT_A::VBUSPRESENT
    }
}
#[doc = "USB supply output settling time elapsed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTPUTRDY_A {
    #[doc = "0: USBREG output settling time not elapsed"]
    NOTREADY = 0,
    #[doc = "1: USBREG output settling time elapsed (same information as USBPWRRDY event)"]
    READY = 1,
}
impl From<OUTPUTRDY_A> for bool {
    #[inline(always)]
    fn from(variant: OUTPUTRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTPUTRDY`"]
pub type OUTPUTRDY_R = crate::R<bool, OUTPUTRDY_A>;
impl OUTPUTRDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTPUTRDY_A {
        match self.bits {
            false => OUTPUTRDY_A::NOTREADY,
            true => OUTPUTRDY_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `NOTREADY`"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == OUTPUTRDY_A::NOTREADY
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == OUTPUTRDY_A::READY
    }
}
impl R {
    #[doc = "Bit 0 - VBUS input detection status (USBDETECTED and USBREMOVED events are derived from this information)"]
    #[inline(always)]
    pub fn vbusdetect(&self) -> VBUSDETECT_R {
        VBUSDETECT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - USB supply output settling time elapsed"]
    #[inline(always)]
    pub fn outputrdy(&self) -> OUTPUTRDY_R {
        OUTPUTRDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
