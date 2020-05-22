#[doc = "Writer for register ERASEPAGE"]
pub type W = crate::W<u32, super::ERASEPAGE>;
#[doc = "Register ERASEPAGE `reset()`'s with value 0"]
impl crate::ResetValue for super::ERASEPAGE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `ERASEPAGE`"]
pub struct ERASEPAGE_W<'a> {
    w: &'a mut W,
}
impl<'a> ERASEPAGE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Register for starting erase of a page in code area"]
    #[inline(always)]
    pub fn erasepage(&mut self) -> ERASEPAGE_W {
        ERASEPAGE_W { w: self }
    }
}
