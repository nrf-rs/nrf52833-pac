#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 20usize],
    #[doc = "0x14 - Description collection: Reserved for Nordic firmware design"]
    pub nrffw: [NRFFW; 13],
    _reserved1: [u8; 8usize],
    #[doc = "0x50 - Description collection: Reserved for Nordic hardware design"]
    pub nrfhw: [NRFHW; 12],
    #[doc = "0x80 - Description collection: Reserved for customer"]
    pub customer: [CUSTOMER; 32],
    _reserved3: [u8; 256usize],
    #[doc = "0x200 - Description collection: Mapping of the nRESET function (see POWER chapter for details)"]
    pub pselreset: [PSELRESET; 2],
    #[doc = "0x208 - Access port protection"]
    pub approtect: APPROTECT,
    #[doc = "0x20c - Setting of pins dedicated to NFC functionality: NFC antenna or GPIO"]
    pub nfcpins: NFCPINS,
    #[doc = "0x210 - Processor debug control"]
    pub debugctrl: DEBUGCTRL,
    _reserved7: [u8; 240usize],
    #[doc = "0x304 - Output voltage from REG0 regulator stage. The maximum output voltage from this stage is given as VDDH - VREG0DROP."]
    pub regout0: REGOUT0,
}
#[doc = "Description collection: Reserved for Nordic firmware design\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nrffw](nrffw) module"]
pub type NRFFW = crate::Reg<u32, _NRFFW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NRFFW;
#[doc = "`read()` method returns [nrffw::R](nrffw::R) reader structure"]
impl crate::Readable for NRFFW {}
#[doc = "`write(|w| ..)` method takes [nrffw::W](nrffw::W) writer structure"]
impl crate::Writable for NRFFW {}
#[doc = "Description collection: Reserved for Nordic firmware design"]
pub mod nrffw;
#[doc = "Description collection: Reserved for Nordic hardware design\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nrfhw](nrfhw) module"]
pub type NRFHW = crate::Reg<u32, _NRFHW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NRFHW;
#[doc = "`read()` method returns [nrfhw::R](nrfhw::R) reader structure"]
impl crate::Readable for NRFHW {}
#[doc = "`write(|w| ..)` method takes [nrfhw::W](nrfhw::W) writer structure"]
impl crate::Writable for NRFHW {}
#[doc = "Description collection: Reserved for Nordic hardware design"]
pub mod nrfhw;
#[doc = "Description collection: Reserved for customer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [customer](customer) module"]
pub type CUSTOMER = crate::Reg<u32, _CUSTOMER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CUSTOMER;
#[doc = "`read()` method returns [customer::R](customer::R) reader structure"]
impl crate::Readable for CUSTOMER {}
#[doc = "`write(|w| ..)` method takes [customer::W](customer::W) writer structure"]
impl crate::Writable for CUSTOMER {}
#[doc = "Description collection: Reserved for customer"]
pub mod customer;
#[doc = "Description collection: Mapping of the nRESET function (see POWER chapter for details)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pselreset](pselreset) module"]
pub type PSELRESET = crate::Reg<u32, _PSELRESET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PSELRESET;
#[doc = "`read()` method returns [pselreset::R](pselreset::R) reader structure"]
impl crate::Readable for PSELRESET {}
#[doc = "`write(|w| ..)` method takes [pselreset::W](pselreset::W) writer structure"]
impl crate::Writable for PSELRESET {}
#[doc = "Description collection: Mapping of the nRESET function (see POWER chapter for details)"]
pub mod pselreset;
#[doc = "Access port protection\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [approtect](approtect) module"]
pub type APPROTECT = crate::Reg<u32, _APPROTECT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APPROTECT;
#[doc = "`read()` method returns [approtect::R](approtect::R) reader structure"]
impl crate::Readable for APPROTECT {}
#[doc = "`write(|w| ..)` method takes [approtect::W](approtect::W) writer structure"]
impl crate::Writable for APPROTECT {}
#[doc = "Access port protection"]
pub mod approtect;
#[doc = "Setting of pins dedicated to NFC functionality: NFC antenna or GPIO\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nfcpins](nfcpins) module"]
pub type NFCPINS = crate::Reg<u32, _NFCPINS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NFCPINS;
#[doc = "`read()` method returns [nfcpins::R](nfcpins::R) reader structure"]
impl crate::Readable for NFCPINS {}
#[doc = "`write(|w| ..)` method takes [nfcpins::W](nfcpins::W) writer structure"]
impl crate::Writable for NFCPINS {}
#[doc = "Setting of pins dedicated to NFC functionality: NFC antenna or GPIO"]
pub mod nfcpins;
#[doc = "Processor debug control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [debugctrl](debugctrl) module"]
pub type DEBUGCTRL = crate::Reg<u32, _DEBUGCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEBUGCTRL;
#[doc = "`read()` method returns [debugctrl::R](debugctrl::R) reader structure"]
impl crate::Readable for DEBUGCTRL {}
#[doc = "`write(|w| ..)` method takes [debugctrl::W](debugctrl::W) writer structure"]
impl crate::Writable for DEBUGCTRL {}
#[doc = "Processor debug control"]
pub mod debugctrl;
#[doc = "Output voltage from REG0 regulator stage. The maximum output voltage from this stage is given as VDDH - VREG0DROP.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [regout0](regout0) module"]
pub type REGOUT0 = crate::Reg<u32, _REGOUT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REGOUT0;
#[doc = "`read()` method returns [regout0::R](regout0::R) reader structure"]
impl crate::Readable for REGOUT0 {}
#[doc = "`write(|w| ..)` method takes [regout0::W](regout0::W) writer structure"]
impl crate::Writable for REGOUT0 {}
#[doc = "Output voltage from REG0 regulator stage. The maximum output voltage from this stage is given as VDDH - VREG0DROP."]
pub mod regout0;
