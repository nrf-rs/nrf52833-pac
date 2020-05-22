#[doc = "Reader of register PACKAGE"]
pub type R = crate::R<u32, super::PACKAGE>;
#[doc = "Package option\n\nValue on reset: 4294967295"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum PACKAGE_A {
    #[doc = "8199: QDxx - 40-pin QFN"]
    QD = 8199,
    #[doc = "8196: QIxx - 73-pin aQFN"]
    QI = 8196,
    #[doc = "8200: CJxx - WLCSP"]
    CJ = 8200,
    #[doc = "4294967295: Unspecified"]
    UNSPECIFIED = 4294967295,
}
impl From<PACKAGE_A> for u32 {
    #[inline(always)]
    fn from(variant: PACKAGE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PACKAGE`"]
pub type PACKAGE_R = crate::R<u32, PACKAGE_A>;
impl PACKAGE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, PACKAGE_A> {
        use crate::Variant::*;
        match self.bits {
            8199 => Val(PACKAGE_A::QD),
            8196 => Val(PACKAGE_A::QI),
            8200 => Val(PACKAGE_A::CJ),
            4294967295 => Val(PACKAGE_A::UNSPECIFIED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `QD`"]
    #[inline(always)]
    pub fn is_qd(&self) -> bool {
        *self == PACKAGE_A::QD
    }
    #[doc = "Checks if the value of the field is `QI`"]
    #[inline(always)]
    pub fn is_qi(&self) -> bool {
        *self == PACKAGE_A::QI
    }
    #[doc = "Checks if the value of the field is `CJ`"]
    #[inline(always)]
    pub fn is_cj(&self) -> bool {
        *self == PACKAGE_A::CJ
    }
    #[doc = "Checks if the value of the field is `UNSPECIFIED`"]
    #[inline(always)]
    pub fn is_unspecified(&self) -> bool {
        *self == PACKAGE_A::UNSPECIFIED
    }
}
impl R {
    #[doc = "Bits 0:31 - Package option"]
    #[inline(always)]
    pub fn package(&self) -> PACKAGE_R {
        PACKAGE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
