#[doc = "Reader of register ROSC1"]
pub type R = crate::R<u32, super::ROSC1>;
#[doc = "Reader of field `ROSC1`"]
pub type ROSC1_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Sample count for ring oscillator 1"]
    #[inline(always)]
    pub fn rosc1(&self) -> ROSC1_R {
        ROSC1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
