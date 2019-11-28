#[doc = "Reader of register BYTES"]
pub type R = crate::R<u32, super::BYTES>;
#[doc = "Reader of field `BYTES`"]
pub type BYTES_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Amount of bytes for the required entropy bits"]
    #[inline(always)]
    pub fn bytes(&self) -> BYTES_R {
        BYTES_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
