# [doc = r"Register block"] # [repr (C)] pub struct SPI { ctrla : CTRLA , ctrlb : CTRLB , _reserved2 : [u8 ; 0x04] , baud : BAUD , _reserved3 : [u8 ; 0x07] , intenclr : INTENCLR , _reserved4 : [u8 ; 0x01] , intenset : INTENSET , _reserved5 : [u8 ; 0x01] , intflag : INTFLAG , _reserved6 : [u8 ; 0x01] , status : STATUS , syncbusy : SYNCBUSY , _reserved8 : [u8 ; 0x04] , addr : ADDR , data : DATA , _reserved10 : [u8 ; 0x04] , dbgctrl : DBGCTRL , } impl SPI { # [doc = "0x00 - SPI Control A"] # [inline (always)] pub const fn ctrla (& self) -> & CTRLA { & self . ctrla } # [doc = "0x04 - SPI Control B"] # [inline (always)] pub const fn ctrlb (& self) -> & CTRLB { & self . ctrlb } # [doc = "0x0c - SPI Baud Rate"] # [inline (always)] pub const fn baud (& self) -> & BAUD { & self . baud } # [doc = "0x14 - SPI Interrupt Enable Clear"] # [inline (always)] pub const fn intenclr (& self) -> & INTENCLR { & self . intenclr } # [doc = "0x16 - SPI Interrupt Enable Set"] # [inline (always)] pub const fn intenset (& self) -> & INTENSET { & self . intenset } # [doc = "0x18 - SPI Interrupt Flag Status and Clear"] # [inline (always)] pub const fn intflag (& self) -> & INTFLAG { & self . intflag } # [doc = "0x1a - SPI Status"] # [inline (always)] pub const fn status (& self) -> & STATUS { & self . status } # [doc = "0x1c - SPI Syncbusy"] # [inline (always)] pub const fn syncbusy (& self) -> & SYNCBUSY { & self . syncbusy } # [doc = "0x24 - SPI Address"] # [inline (always)] pub const fn addr (& self) -> & ADDR { & self . addr } # [doc = "0x28 - SPI Data"] # [inline (always)] pub const fn data (& self) -> & DATA { & self . data } # [doc = "0x30 - SPI Debug Control"] # [inline (always)] pub const fn dbgctrl (& self) -> & DBGCTRL { & self . dbgctrl } } # [doc = "CTRLA (rw) register accessor: SPI Control A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrla::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrla::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrla`]
module"] pub type CTRLA = crate :: Reg < ctrla :: CTRLA_SPEC > ; # [doc = "SPI Control A"] pub mod ctrla ; # [doc = "CTRLB (rw) register accessor: SPI Control B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlb`]
module"] pub type CTRLB = crate :: Reg < ctrlb :: CTRLB_SPEC > ; # [doc = "SPI Control B"] pub mod ctrlb ; # [doc = "BAUD (rw) register accessor: SPI Baud Rate\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`baud::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`baud::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@baud`]
module"] pub type BAUD = crate :: Reg < baud :: BAUD_SPEC > ; # [doc = "SPI Baud Rate"] pub mod baud ; # [doc = "INTENCLR (rw) register accessor: SPI Interrupt Enable Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intenclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intenclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenclr`]
module"] pub type INTENCLR = crate :: Reg < intenclr :: INTENCLR_SPEC > ; # [doc = "SPI Interrupt Enable Clear"] pub mod intenclr ; # [doc = "INTENSET (rw) register accessor: SPI Interrupt Enable Set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intenset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intenset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenset`]
module"] pub type INTENSET = crate :: Reg < intenset :: INTENSET_SPEC > ; # [doc = "SPI Interrupt Enable Set"] pub mod intenset ; # [doc = "INTFLAG (rw) register accessor: SPI Interrupt Flag Status and Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intflag::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intflag::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intflag`]
module"] pub type INTFLAG = crate :: Reg < intflag :: INTFLAG_SPEC > ; # [doc = "SPI Interrupt Flag Status and Clear"] pub mod intflag ; # [doc = "STATUS (rw) register accessor: SPI Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"] pub type STATUS = crate :: Reg < status :: STATUS_SPEC > ; # [doc = "SPI Status"] pub mod status ; # [doc = "SYNCBUSY (r) register accessor: SPI Syncbusy\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syncbusy::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syncbusy`]
module"] pub type SYNCBUSY = crate :: Reg < syncbusy :: SYNCBUSY_SPEC > ; # [doc = "SPI Syncbusy"] pub mod syncbusy ; # [doc = "ADDR (rw) register accessor: SPI Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr`]
module"] pub type ADDR = crate :: Reg < addr :: ADDR_SPEC > ; # [doc = "SPI Address"] pub mod addr ; # [doc = "DATA (rw) register accessor: SPI Data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data`]
module"] pub type DATA = crate :: Reg < data :: DATA_SPEC > ; # [doc = "SPI Data"] pub mod data ; # [doc = "DBGCTRL (rw) register accessor: SPI Debug Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbgctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbgctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgctrl`]
module"] pub type DBGCTRL = crate :: Reg < dbgctrl :: DBGCTRL_SPEC > ; # [doc = "SPI Debug Control"] pub mod dbgctrl ;