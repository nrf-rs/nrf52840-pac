#[doc = "Reader of register AMOUNT"]
pub type R = crate::R<u32, super::AMOUNT>;
#[doc = "Reader of field `AMOUNT`"]
pub type AMOUNT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Number of bytes transferred in the last RXD transaction"]
    #[inline(always)]
    pub fn amount(&self) -> AMOUNT_R {
        AMOUNT_R::new((self.bits & 0xffff) as u16)
    }
}
