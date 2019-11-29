#[doc = "Reader of register APCUTOFF"]
pub type R = crate::R<u32, super::APCUTOFF>;
#[doc = "Reader of field `APCUTOFF`"]
pub type APCUTOFF_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Adaptive proportion cutoff"]
    #[inline(always)]
    pub fn apcutoff(&self) -> APCUTOFF_R {
        APCUTOFF_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
