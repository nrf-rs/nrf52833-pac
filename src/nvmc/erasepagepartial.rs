#[doc = "Writer for register ERASEPAGEPARTIAL"]
pub type W = crate::W<u32, super::ERASEPAGEPARTIAL>;
#[doc = "Register ERASEPAGEPARTIAL `reset()`'s with value 0"]
impl crate::ResetValue for super::ERASEPAGEPARTIAL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `ERASEPAGEPARTIAL`"]
pub struct ERASEPAGEPARTIAL_W<'a> {
    w: &'a mut W,
}
impl<'a> ERASEPAGEPARTIAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Register for starting partial erase of a page in code area"]
    #[inline(always)]
    pub fn erasepagepartial(&mut self) -> ERASEPAGEPARTIAL_W {
        ERASEPAGEPARTIAL_W { w: self }
    }
}
