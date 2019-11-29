#[doc = "Reader of register ROSC4"]
pub type R = crate::R<u32, super::ROSC4>;
#[doc = "Reader of field `ROSC4`"]
pub type ROSC4_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Sample count for ring oscillator 4"]
    #[inline(always)]
    pub fn rosc4(&self) -> ROSC4_R {
        ROSC4_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
