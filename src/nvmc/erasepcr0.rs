#[doc = "Writer for register ERASEPCR0"]
pub type W = crate::W<u32, super::ERASEPCR0>;
#[doc = "Register ERASEPCR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::ERASEPCR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `ERASEPCR0`"]
pub struct ERASEPCR0_W<'a> {
    w: &'a mut W,
}
impl<'a> ERASEPCR0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Register for starting erase of a page in code area, equivalent to ERASEPAGE"]
    #[inline(always)]
    pub fn erasepcr0(&mut self) -> ERASEPCR0_W {
        ERASEPCR0_W { w: self }
    }
}
