#[doc = "Reader of register SHORTS"]
pub type R = crate::R<u32, super::SHORTS>;
#[doc = "Writer for register SHORTS"]
pub type W = crate::W<u32, super::SHORTS>;
#[doc = "Register SHORTS `reset()`'s with value 0"]
impl crate::ResetValue for super::SHORTS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Shortcut between READY event and START task\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READY_START_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<READY_START_A> for bool {
    #[inline(always)]
    fn from(variant: READY_START_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `READY_START`"]
pub type READY_START_R = crate::R<bool, READY_START_A>;
impl READY_START_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> READY_START_A {
        match self.bits {
            false => READY_START_A::DISABLED,
            true => READY_START_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == READY_START_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == READY_START_A::ENABLED
    }
}
#[doc = "Write proxy for field `READY_START`"]
pub struct READY_START_W<'a> {
    w: &'a mut W,
}
impl<'a> READY_START_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: READY_START_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(READY_START_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(READY_START_A::ENABLED)
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
#[doc = "Shortcut between END event and DISABLE task\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum END_DISABLE_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<END_DISABLE_A> for bool {
    #[inline(always)]
    fn from(variant: END_DISABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `END_DISABLE`"]
pub type END_DISABLE_R = crate::R<bool, END_DISABLE_A>;
impl END_DISABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> END_DISABLE_A {
        match self.bits {
            false => END_DISABLE_A::DISABLED,
            true => END_DISABLE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == END_DISABLE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == END_DISABLE_A::ENABLED
    }
}
#[doc = "Write proxy for field `END_DISABLE`"]
pub struct END_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> END_DISABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: END_DISABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(END_DISABLE_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(END_DISABLE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Shortcut between DISABLED event and TXEN task\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISABLED_TXEN_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<DISABLED_TXEN_A> for bool {
    #[inline(always)]
    fn from(variant: DISABLED_TXEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DISABLED_TXEN`"]
pub type DISABLED_TXEN_R = crate::R<bool, DISABLED_TXEN_A>;
impl DISABLED_TXEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DISABLED_TXEN_A {
        match self.bits {
            false => DISABLED_TXEN_A::DISABLED,
            true => DISABLED_TXEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DISABLED_TXEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DISABLED_TXEN_A::ENABLED
    }
}
#[doc = "Write proxy for field `DISABLED_TXEN`"]
pub struct DISABLED_TXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DISABLED_TXEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DISABLED_TXEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DISABLED_TXEN_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DISABLED_TXEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Shortcut between DISABLED event and RXEN task\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISABLED_RXEN_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<DISABLED_RXEN_A> for bool {
    #[inline(always)]
    fn from(variant: DISABLED_RXEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DISABLED_RXEN`"]
pub type DISABLED_RXEN_R = crate::R<bool, DISABLED_RXEN_A>;
impl DISABLED_RXEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DISABLED_RXEN_A {
        match self.bits {
            false => DISABLED_RXEN_A::DISABLED,
            true => DISABLED_RXEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DISABLED_RXEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DISABLED_RXEN_A::ENABLED
    }
}
#[doc = "Write proxy for field `DISABLED_RXEN`"]
pub struct DISABLED_RXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DISABLED_RXEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DISABLED_RXEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DISABLED_RXEN_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DISABLED_RXEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Shortcut between ADDRESS event and RSSISTART task\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDRESS_RSSISTART_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<ADDRESS_RSSISTART_A> for bool {
    #[inline(always)]
    fn from(variant: ADDRESS_RSSISTART_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADDRESS_RSSISTART`"]
pub type ADDRESS_RSSISTART_R = crate::R<bool, ADDRESS_RSSISTART_A>;
impl ADDRESS_RSSISTART_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDRESS_RSSISTART_A {
        match self.bits {
            false => ADDRESS_RSSISTART_A::DISABLED,
            true => ADDRESS_RSSISTART_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADDRESS_RSSISTART_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADDRESS_RSSISTART_A::ENABLED
    }
}
#[doc = "Write proxy for field `ADDRESS_RSSISTART`"]
pub struct ADDRESS_RSSISTART_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRESS_RSSISTART_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADDRESS_RSSISTART_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADDRESS_RSSISTART_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADDRESS_RSSISTART_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Shortcut between END event and START task\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum END_START_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<END_START_A> for bool {
    #[inline(always)]
    fn from(variant: END_START_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `END_START`"]
pub type END_START_R = crate::R<bool, END_START_A>;
impl END_START_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> END_START_A {
        match self.bits {
            false => END_START_A::DISABLED,
            true => END_START_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == END_START_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == END_START_A::ENABLED
    }
}
#[doc = "Write proxy for field `END_START`"]
pub struct END_START_W<'a> {
    w: &'a mut W,
}
impl<'a> END_START_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: END_START_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(END_START_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(END_START_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Shortcut between ADDRESS event and BCSTART task\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDRESS_BCSTART_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<ADDRESS_BCSTART_A> for bool {
    #[inline(always)]
    fn from(variant: ADDRESS_BCSTART_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADDRESS_BCSTART`"]
pub type ADDRESS_BCSTART_R = crate::R<bool, ADDRESS_BCSTART_A>;
impl ADDRESS_BCSTART_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDRESS_BCSTART_A {
        match self.bits {
            false => ADDRESS_BCSTART_A::DISABLED,
            true => ADDRESS_BCSTART_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADDRESS_BCSTART_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADDRESS_BCSTART_A::ENABLED
    }
}
#[doc = "Write proxy for field `ADDRESS_BCSTART`"]
pub struct ADDRESS_BCSTART_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRESS_BCSTART_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADDRESS_BCSTART_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADDRESS_BCSTART_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADDRESS_BCSTART_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Shortcut between DISABLED event and RSSISTOP task\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISABLED_RSSISTOP_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<DISABLED_RSSISTOP_A> for bool {
    #[inline(always)]
    fn from(variant: DISABLED_RSSISTOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DISABLED_RSSISTOP`"]
pub type DISABLED_RSSISTOP_R = crate::R<bool, DISABLED_RSSISTOP_A>;
impl DISABLED_RSSISTOP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DISABLED_RSSISTOP_A {
        match self.bits {
            false => DISABLED_RSSISTOP_A::DISABLED,
            true => DISABLED_RSSISTOP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DISABLED_RSSISTOP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DISABLED_RSSISTOP_A::ENABLED
    }
}
#[doc = "Write proxy for field `DISABLED_RSSISTOP`"]
pub struct DISABLED_RSSISTOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DISABLED_RSSISTOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DISABLED_RSSISTOP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DISABLED_RSSISTOP_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DISABLED_RSSISTOP_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Shortcut between RXREADY event and CCASTART task\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXREADY_CCASTART_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<RXREADY_CCASTART_A> for bool {
    #[inline(always)]
    fn from(variant: RXREADY_CCASTART_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXREADY_CCASTART`"]
pub type RXREADY_CCASTART_R = crate::R<bool, RXREADY_CCASTART_A>;
impl RXREADY_CCASTART_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXREADY_CCASTART_A {
        match self.bits {
            false => RXREADY_CCASTART_A::DISABLED,
            true => RXREADY_CCASTART_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXREADY_CCASTART_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXREADY_CCASTART_A::ENABLED
    }
}
#[doc = "Write proxy for field `RXREADY_CCASTART`"]
pub struct RXREADY_CCASTART_W<'a> {
    w: &'a mut W,
}
impl<'a> RXREADY_CCASTART_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXREADY_CCASTART_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RXREADY_CCASTART_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RXREADY_CCASTART_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Shortcut between CCAIDLE event and TXEN task\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCAIDLE_TXEN_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<CCAIDLE_TXEN_A> for bool {
    #[inline(always)]
    fn from(variant: CCAIDLE_TXEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CCAIDLE_TXEN`"]
pub type CCAIDLE_TXEN_R = crate::R<bool, CCAIDLE_TXEN_A>;
impl CCAIDLE_TXEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCAIDLE_TXEN_A {
        match self.bits {
            false => CCAIDLE_TXEN_A::DISABLED,
            true => CCAIDLE_TXEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CCAIDLE_TXEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CCAIDLE_TXEN_A::ENABLED
    }
}
#[doc = "Write proxy for field `CCAIDLE_TXEN`"]
pub struct CCAIDLE_TXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CCAIDLE_TXEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCAIDLE_TXEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CCAIDLE_TXEN_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CCAIDLE_TXEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Shortcut between CCABUSY event and DISABLE task\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCABUSY_DISABLE_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<CCABUSY_DISABLE_A> for bool {
    #[inline(always)]
    fn from(variant: CCABUSY_DISABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CCABUSY_DISABLE`"]
pub type CCABUSY_DISABLE_R = crate::R<bool, CCABUSY_DISABLE_A>;
impl CCABUSY_DISABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCABUSY_DISABLE_A {
        match self.bits {
            false => CCABUSY_DISABLE_A::DISABLED,
            true => CCABUSY_DISABLE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CCABUSY_DISABLE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CCABUSY_DISABLE_A::ENABLED
    }
}
#[doc = "Write proxy for field `CCABUSY_DISABLE`"]
pub struct CCABUSY_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CCABUSY_DISABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCABUSY_DISABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CCABUSY_DISABLE_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CCABUSY_DISABLE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Shortcut between FRAMESTART event and BCSTART task\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRAMESTART_BCSTART_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<FRAMESTART_BCSTART_A> for bool {
    #[inline(always)]
    fn from(variant: FRAMESTART_BCSTART_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FRAMESTART_BCSTART`"]
pub type FRAMESTART_BCSTART_R = crate::R<bool, FRAMESTART_BCSTART_A>;
impl FRAMESTART_BCSTART_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRAMESTART_BCSTART_A {
        match self.bits {
            false => FRAMESTART_BCSTART_A::DISABLED,
            true => FRAMESTART_BCSTART_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FRAMESTART_BCSTART_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FRAMESTART_BCSTART_A::ENABLED
    }
}
#[doc = "Write proxy for field `FRAMESTART_BCSTART`"]
pub struct FRAMESTART_BCSTART_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAMESTART_BCSTART_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRAMESTART_BCSTART_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FRAMESTART_BCSTART_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FRAMESTART_BCSTART_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Shortcut between READY event and EDSTART task\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READY_EDSTART_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<READY_EDSTART_A> for bool {
    #[inline(always)]
    fn from(variant: READY_EDSTART_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `READY_EDSTART`"]
pub type READY_EDSTART_R = crate::R<bool, READY_EDSTART_A>;
impl READY_EDSTART_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> READY_EDSTART_A {
        match self.bits {
            false => READY_EDSTART_A::DISABLED,
            true => READY_EDSTART_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == READY_EDSTART_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == READY_EDSTART_A::ENABLED
    }
}
#[doc = "Write proxy for field `READY_EDSTART`"]
pub struct READY_EDSTART_W<'a> {
    w: &'a mut W,
}
impl<'a> READY_EDSTART_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: READY_EDSTART_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(READY_EDSTART_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(READY_EDSTART_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Shortcut between EDEND event and DISABLE task\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDEND_DISABLE_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<EDEND_DISABLE_A> for bool {
    #[inline(always)]
    fn from(variant: EDEND_DISABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EDEND_DISABLE`"]
pub type EDEND_DISABLE_R = crate::R<bool, EDEND_DISABLE_A>;
impl EDEND_DISABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDEND_DISABLE_A {
        match self.bits {
            false => EDEND_DISABLE_A::DISABLED,
            true => EDEND_DISABLE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EDEND_DISABLE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EDEND_DISABLE_A::ENABLED
    }
}
#[doc = "Write proxy for field `EDEND_DISABLE`"]
pub struct EDEND_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> EDEND_DISABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDEND_DISABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EDEND_DISABLE_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EDEND_DISABLE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Shortcut between CCAIDLE event and STOP task\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCAIDLE_STOP_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<CCAIDLE_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: CCAIDLE_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CCAIDLE_STOP`"]
pub type CCAIDLE_STOP_R = crate::R<bool, CCAIDLE_STOP_A>;
impl CCAIDLE_STOP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCAIDLE_STOP_A {
        match self.bits {
            false => CCAIDLE_STOP_A::DISABLED,
            true => CCAIDLE_STOP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CCAIDLE_STOP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CCAIDLE_STOP_A::ENABLED
    }
}
#[doc = "Write proxy for field `CCAIDLE_STOP`"]
pub struct CCAIDLE_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> CCAIDLE_STOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCAIDLE_STOP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CCAIDLE_STOP_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CCAIDLE_STOP_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Shortcut between TXREADY event and START task\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXREADY_START_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<TXREADY_START_A> for bool {
    #[inline(always)]
    fn from(variant: TXREADY_START_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXREADY_START`"]
pub type TXREADY_START_R = crate::R<bool, TXREADY_START_A>;
impl TXREADY_START_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXREADY_START_A {
        match self.bits {
            false => TXREADY_START_A::DISABLED,
            true => TXREADY_START_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXREADY_START_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXREADY_START_A::ENABLED
    }
}
#[doc = "Write proxy for field `TXREADY_START`"]
pub struct TXREADY_START_W<'a> {
    w: &'a mut W,
}
impl<'a> TXREADY_START_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXREADY_START_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXREADY_START_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TXREADY_START_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Shortcut between RXREADY event and START task\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXREADY_START_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<RXREADY_START_A> for bool {
    #[inline(always)]
    fn from(variant: RXREADY_START_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXREADY_START`"]
pub type RXREADY_START_R = crate::R<bool, RXREADY_START_A>;
impl RXREADY_START_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXREADY_START_A {
        match self.bits {
            false => RXREADY_START_A::DISABLED,
            true => RXREADY_START_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXREADY_START_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXREADY_START_A::ENABLED
    }
}
#[doc = "Write proxy for field `RXREADY_START`"]
pub struct RXREADY_START_W<'a> {
    w: &'a mut W,
}
impl<'a> RXREADY_START_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXREADY_START_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RXREADY_START_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RXREADY_START_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Shortcut between PHYEND event and DISABLE task\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PHYEND_DISABLE_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<PHYEND_DISABLE_A> for bool {
    #[inline(always)]
    fn from(variant: PHYEND_DISABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PHYEND_DISABLE`"]
pub type PHYEND_DISABLE_R = crate::R<bool, PHYEND_DISABLE_A>;
impl PHYEND_DISABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PHYEND_DISABLE_A {
        match self.bits {
            false => PHYEND_DISABLE_A::DISABLED,
            true => PHYEND_DISABLE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PHYEND_DISABLE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PHYEND_DISABLE_A::ENABLED
    }
}
#[doc = "Write proxy for field `PHYEND_DISABLE`"]
pub struct PHYEND_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> PHYEND_DISABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PHYEND_DISABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PHYEND_DISABLE_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PHYEND_DISABLE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Shortcut between PHYEND event and START task\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PHYEND_START_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<PHYEND_START_A> for bool {
    #[inline(always)]
    fn from(variant: PHYEND_START_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PHYEND_START`"]
pub type PHYEND_START_R = crate::R<bool, PHYEND_START_A>;
impl PHYEND_START_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PHYEND_START_A {
        match self.bits {
            false => PHYEND_START_A::DISABLED,
            true => PHYEND_START_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PHYEND_START_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PHYEND_START_A::ENABLED
    }
}
#[doc = "Write proxy for field `PHYEND_START`"]
pub struct PHYEND_START_W<'a> {
    w: &'a mut W,
}
impl<'a> PHYEND_START_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PHYEND_START_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PHYEND_START_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PHYEND_START_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Shortcut between READY event and START task"]
    #[inline(always)]
    pub fn ready_start(&self) -> READY_START_R {
        READY_START_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Shortcut between END event and DISABLE task"]
    #[inline(always)]
    pub fn end_disable(&self) -> END_DISABLE_R {
        END_DISABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Shortcut between DISABLED event and TXEN task"]
    #[inline(always)]
    pub fn disabled_txen(&self) -> DISABLED_TXEN_R {
        DISABLED_TXEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Shortcut between DISABLED event and RXEN task"]
    #[inline(always)]
    pub fn disabled_rxen(&self) -> DISABLED_RXEN_R {
        DISABLED_RXEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Shortcut between ADDRESS event and RSSISTART task"]
    #[inline(always)]
    pub fn address_rssistart(&self) -> ADDRESS_RSSISTART_R {
        ADDRESS_RSSISTART_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Shortcut between END event and START task"]
    #[inline(always)]
    pub fn end_start(&self) -> END_START_R {
        END_START_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Shortcut between ADDRESS event and BCSTART task"]
    #[inline(always)]
    pub fn address_bcstart(&self) -> ADDRESS_BCSTART_R {
        ADDRESS_BCSTART_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Shortcut between DISABLED event and RSSISTOP task"]
    #[inline(always)]
    pub fn disabled_rssistop(&self) -> DISABLED_RSSISTOP_R {
        DISABLED_RSSISTOP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Shortcut between RXREADY event and CCASTART task"]
    #[inline(always)]
    pub fn rxready_ccastart(&self) -> RXREADY_CCASTART_R {
        RXREADY_CCASTART_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Shortcut between CCAIDLE event and TXEN task"]
    #[inline(always)]
    pub fn ccaidle_txen(&self) -> CCAIDLE_TXEN_R {
        CCAIDLE_TXEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Shortcut between CCABUSY event and DISABLE task"]
    #[inline(always)]
    pub fn ccabusy_disable(&self) -> CCABUSY_DISABLE_R {
        CCABUSY_DISABLE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Shortcut between FRAMESTART event and BCSTART task"]
    #[inline(always)]
    pub fn framestart_bcstart(&self) -> FRAMESTART_BCSTART_R {
        FRAMESTART_BCSTART_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Shortcut between READY event and EDSTART task"]
    #[inline(always)]
    pub fn ready_edstart(&self) -> READY_EDSTART_R {
        READY_EDSTART_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Shortcut between EDEND event and DISABLE task"]
    #[inline(always)]
    pub fn edend_disable(&self) -> EDEND_DISABLE_R {
        EDEND_DISABLE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Shortcut between CCAIDLE event and STOP task"]
    #[inline(always)]
    pub fn ccaidle_stop(&self) -> CCAIDLE_STOP_R {
        CCAIDLE_STOP_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Shortcut between TXREADY event and START task"]
    #[inline(always)]
    pub fn txready_start(&self) -> TXREADY_START_R {
        TXREADY_START_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Shortcut between RXREADY event and START task"]
    #[inline(always)]
    pub fn rxready_start(&self) -> RXREADY_START_R {
        RXREADY_START_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Shortcut between PHYEND event and DISABLE task"]
    #[inline(always)]
    pub fn phyend_disable(&self) -> PHYEND_DISABLE_R {
        PHYEND_DISABLE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Shortcut between PHYEND event and START task"]
    #[inline(always)]
    pub fn phyend_start(&self) -> PHYEND_START_R {
        PHYEND_START_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Shortcut between READY event and START task"]
    #[inline(always)]
    pub fn ready_start(&mut self) -> READY_START_W {
        READY_START_W { w: self }
    }
    #[doc = "Bit 1 - Shortcut between END event and DISABLE task"]
    #[inline(always)]
    pub fn end_disable(&mut self) -> END_DISABLE_W {
        END_DISABLE_W { w: self }
    }
    #[doc = "Bit 2 - Shortcut between DISABLED event and TXEN task"]
    #[inline(always)]
    pub fn disabled_txen(&mut self) -> DISABLED_TXEN_W {
        DISABLED_TXEN_W { w: self }
    }
    #[doc = "Bit 3 - Shortcut between DISABLED event and RXEN task"]
    #[inline(always)]
    pub fn disabled_rxen(&mut self) -> DISABLED_RXEN_W {
        DISABLED_RXEN_W { w: self }
    }
    #[doc = "Bit 4 - Shortcut between ADDRESS event and RSSISTART task"]
    #[inline(always)]
    pub fn address_rssistart(&mut self) -> ADDRESS_RSSISTART_W {
        ADDRESS_RSSISTART_W { w: self }
    }
    #[doc = "Bit 5 - Shortcut between END event and START task"]
    #[inline(always)]
    pub fn end_start(&mut self) -> END_START_W {
        END_START_W { w: self }
    }
    #[doc = "Bit 6 - Shortcut between ADDRESS event and BCSTART task"]
    #[inline(always)]
    pub fn address_bcstart(&mut self) -> ADDRESS_BCSTART_W {
        ADDRESS_BCSTART_W { w: self }
    }
    #[doc = "Bit 8 - Shortcut between DISABLED event and RSSISTOP task"]
    #[inline(always)]
    pub fn disabled_rssistop(&mut self) -> DISABLED_RSSISTOP_W {
        DISABLED_RSSISTOP_W { w: self }
    }
    #[doc = "Bit 11 - Shortcut between RXREADY event and CCASTART task"]
    #[inline(always)]
    pub fn rxready_ccastart(&mut self) -> RXREADY_CCASTART_W {
        RXREADY_CCASTART_W { w: self }
    }
    #[doc = "Bit 12 - Shortcut between CCAIDLE event and TXEN task"]
    #[inline(always)]
    pub fn ccaidle_txen(&mut self) -> CCAIDLE_TXEN_W {
        CCAIDLE_TXEN_W { w: self }
    }
    #[doc = "Bit 13 - Shortcut between CCABUSY event and DISABLE task"]
    #[inline(always)]
    pub fn ccabusy_disable(&mut self) -> CCABUSY_DISABLE_W {
        CCABUSY_DISABLE_W { w: self }
    }
    #[doc = "Bit 14 - Shortcut between FRAMESTART event and BCSTART task"]
    #[inline(always)]
    pub fn framestart_bcstart(&mut self) -> FRAMESTART_BCSTART_W {
        FRAMESTART_BCSTART_W { w: self }
    }
    #[doc = "Bit 15 - Shortcut between READY event and EDSTART task"]
    #[inline(always)]
    pub fn ready_edstart(&mut self) -> READY_EDSTART_W {
        READY_EDSTART_W { w: self }
    }
    #[doc = "Bit 16 - Shortcut between EDEND event and DISABLE task"]
    #[inline(always)]
    pub fn edend_disable(&mut self) -> EDEND_DISABLE_W {
        EDEND_DISABLE_W { w: self }
    }
    #[doc = "Bit 17 - Shortcut between CCAIDLE event and STOP task"]
    #[inline(always)]
    pub fn ccaidle_stop(&mut self) -> CCAIDLE_STOP_W {
        CCAIDLE_STOP_W { w: self }
    }
    #[doc = "Bit 18 - Shortcut between TXREADY event and START task"]
    #[inline(always)]
    pub fn txready_start(&mut self) -> TXREADY_START_W {
        TXREADY_START_W { w: self }
    }
    #[doc = "Bit 19 - Shortcut between RXREADY event and START task"]
    #[inline(always)]
    pub fn rxready_start(&mut self) -> RXREADY_START_W {
        RXREADY_START_W { w: self }
    }
    #[doc = "Bit 20 - Shortcut between PHYEND event and DISABLE task"]
    #[inline(always)]
    pub fn phyend_disable(&mut self) -> PHYEND_DISABLE_W {
        PHYEND_DISABLE_W { w: self }
    }
    #[doc = "Bit 21 - Shortcut between PHYEND event and START task"]
    #[inline(always)]
    pub fn phyend_start(&mut self) -> PHYEND_START_W {
        PHYEND_START_W { w: self }
    }
}
