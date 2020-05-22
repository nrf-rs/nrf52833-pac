#[doc = "Writer for register ERASEPCR1"]
pub type W = crate::W<u32, super::ERASEPCR1>;
#[doc = "Register ERASEPCR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::ERASEPCR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `ERASEPCR1`"]
pub struct ERASEPCR1_W<'a> {
    w: &'a mut W,
}
impl<'a> ERASEPCR1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Register for erasing a page in code area, equivalent to ERASEPAGE"]
    #[inline(always)]
    pub fn erasepcr1(&mut self) -> ERASEPCR1_W {
        ERASEPCR1_W { w: self }
    }
}
