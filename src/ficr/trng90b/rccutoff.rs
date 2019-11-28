#[doc = "Reader of register RCCUTOFF"]
pub type R = crate::R<u32, super::RCCUTOFF>;
#[doc = "Reader of field `RCCUTOFF`"]
pub type RCCUTOFF_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Repetition counter cutoff"]
    #[inline(always)]
    pub fn rccutoff(&self) -> RCCUTOFF_R {
        RCCUTOFF_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
