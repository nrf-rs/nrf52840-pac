#[doc = "Reader of register STARTUP"]
pub type R = crate::R<u32, super::STARTUP>;
#[doc = "Reader of field `STARTUP`"]
pub type STARTUP_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Amount of bytes for the startup tests"]
    #[inline(always)]
    pub fn startup(&self) -> STARTUP_R {
        STARTUP_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
