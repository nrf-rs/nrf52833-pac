#[doc = "Reader of register LFXODEBOUNCE"]
pub type R = crate::R<u32, super::LFXODEBOUNCE>;
#[doc = "Writer for register LFXODEBOUNCE"]
pub type W = crate::W<u32, super::LFXODEBOUNCE>;
#[doc = "Register LFXODEBOUNCE `reset()`'s with value 0"]
impl crate::ResetValue for super::LFXODEBOUNCE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "LFXO debounce time.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LFXODEBOUNCE_A {
    #[doc = "0: 8192 32.768 kHz periods, or 0.25 s. Recommended for normal Operating Temperature conditions."]
    NORMAL = 0,
    #[doc = "1: 16384 32.768 kHz periods, or 0.5 s. Recommended for Extended Operating Temperature conditions."]
    EXTENDED = 1,
}
impl From<LFXODEBOUNCE_A> for bool {
    #[inline(always)]
    fn from(variant: LFXODEBOUNCE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LFXODEBOUNCE`"]
pub type LFXODEBOUNCE_R = crate::R<bool, LFXODEBOUNCE_A>;
impl LFXODEBOUNCE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LFXODEBOUNCE_A {
        match self.bits {
            false => LFXODEBOUNCE_A::NORMAL,
            true => LFXODEBOUNCE_A::EXTENDED,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == LFXODEBOUNCE_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `EXTENDED`"]
    #[inline(always)]
    pub fn is_extended(&self) -> bool {
        *self == LFXODEBOUNCE_A::EXTENDED
    }
}
#[doc = "Write proxy for field `LFXODEBOUNCE`"]
pub struct LFXODEBOUNCE_W<'a> {
    w: &'a mut W,
}
impl<'a> LFXODEBOUNCE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LFXODEBOUNCE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "8192 32.768 kHz periods, or 0.25 s. Recommended for normal Operating Temperature conditions."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(LFXODEBOUNCE_A::NORMAL)
    }
    #[doc = "16384 32.768 kHz periods, or 0.5 s. Recommended for Extended Operating Temperature conditions."]
    #[inline(always)]
    pub fn extended(self) -> &'a mut W {
        self.variant(LFXODEBOUNCE_A::EXTENDED)
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
impl R {
    #[doc = "Bit 0 - LFXO debounce time."]
    #[inline(always)]
    pub fn lfxodebounce(&self) -> LFXODEBOUNCE_R {
        LFXODEBOUNCE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LFXO debounce time."]
    #[inline(always)]
    pub fn lfxodebounce(&mut self) -> LFXODEBOUNCE_W {
        LFXODEBOUNCE_W { w: self }
    }
}
