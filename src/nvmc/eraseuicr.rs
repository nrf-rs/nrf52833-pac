#[doc = "Writer for register ERASEUICR"]
pub type W = crate::W<u32, super::ERASEUICR>;
#[doc = "Register ERASEUICR `reset()`'s with value 0"]
impl crate::ResetValue for super::ERASEUICR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Register starting erase of all user information configuration registers. The erase must be enabled using CONFIG.WEN before the UICR can be erased.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERASEUICR_AW {
    #[doc = "0: No operation"]
    NOOPERATION = 0,
    #[doc = "1: Start erase of UICR"]
    ERASE = 1,
}
impl From<ERASEUICR_AW> for bool {
    #[inline(always)]
    fn from(variant: ERASEUICR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `ERASEUICR`"]
pub struct ERASEUICR_W<'a> {
    w: &'a mut W,
}
impl<'a> ERASEUICR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERASEUICR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No operation"]
    #[inline(always)]
    pub fn no_operation(self) -> &'a mut W {
        self.variant(ERASEUICR_AW::NOOPERATION)
    }
    #[doc = "Start erase of UICR"]
    #[inline(always)]
    pub fn erase(self) -> &'a mut W {
        self.variant(ERASEUICR_AW::ERASE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Register starting erase of all user information configuration registers. The erase must be enabled using CONFIG.WEN before the UICR can be erased."]
    #[inline(always)]
    pub fn eraseuicr(&mut self) -> ERASEUICR_W {
        ERASEUICR_W { w: self }
    }
}
