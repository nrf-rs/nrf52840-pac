#[doc = "Reader of register ROSC2"]
pub type R = crate::R<u32, super::ROSC2>;
#[doc = "Reader of field `ROSC2`"]
pub type ROSC2_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Sample count for ring oscillator 2"]
    #[inline(always)]
    pub fn rosc2(&self) -> ROSC2_R {
        ROSC2_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
