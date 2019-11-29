#[doc = "Reader of register ROSC3"]
pub type R = crate::R<u32, super::ROSC3>;
#[doc = "Reader of field `ROSC3`"]
pub type ROSC3_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Sample count for ring oscillator 3"]
    #[inline(always)]
    pub fn rosc3(&self) -> ROSC3_R {
        ROSC3_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
