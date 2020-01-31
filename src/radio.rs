#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Enable RADIO in TX mode"]
    pub tasks_txen: TASKS_TXEN,
    #[doc = "0x04 - Enable RADIO in RX mode"]
    pub tasks_rxen: TASKS_RXEN,
    #[doc = "0x08 - Start RADIO"]
    pub tasks_start: TASKS_START,
    #[doc = "0x0c - Stop RADIO"]
    pub tasks_stop: TASKS_STOP,
    #[doc = "0x10 - Disable RADIO"]
    pub tasks_disable: TASKS_DISABLE,
    #[doc = "0x14 - Start the RSSI and take one single sample of the receive signal strength"]
    pub tasks_rssistart: TASKS_RSSISTART,
    #[doc = "0x18 - Stop the RSSI measurement"]
    pub tasks_rssistop: TASKS_RSSISTOP,
    #[doc = "0x1c - Start the bit counter"]
    pub tasks_bcstart: TASKS_BCSTART,
    #[doc = "0x20 - Stop the bit counter"]
    pub tasks_bcstop: TASKS_BCSTOP,
    #[doc = "0x24 - Start the energy detect measurement used in IEEE 802.15.4 mode"]
    pub tasks_edstart: TASKS_EDSTART,
    #[doc = "0x28 - Stop the energy detect measurement"]
    pub tasks_edstop: TASKS_EDSTOP,
    #[doc = "0x2c - Start the clear channel assessment used in IEEE 802.15.4 mode"]
    pub tasks_ccastart: TASKS_CCASTART,
    #[doc = "0x30 - Stop the clear channel assessment"]
    pub tasks_ccastop: TASKS_CCASTOP,
    _reserved13: [u8; 204usize],
    #[doc = "0x100 - RADIO has ramped up and is ready to be started"]
    pub events_ready: EVENTS_READY,
    #[doc = "0x104 - Address sent or received"]
    pub events_address: EVENTS_ADDRESS,
    #[doc = "0x108 - Packet payload sent or received"]
    pub events_payload: EVENTS_PAYLOAD,
    #[doc = "0x10c - Packet sent or received"]
    pub events_end: EVENTS_END,
    #[doc = "0x110 - RADIO has been disabled"]
    pub events_disabled: EVENTS_DISABLED,
    #[doc = "0x114 - A device address match occurred on the last received packet"]
    pub events_devmatch: EVENTS_DEVMATCH,
    #[doc = "0x118 - No device address match occurred on the last received packet"]
    pub events_devmiss: EVENTS_DEVMISS,
    #[doc = "0x11c - Sampling of receive signal strength complete"]
    pub events_rssiend: EVENTS_RSSIEND,
    _reserved21: [u8; 8usize],
    #[doc = "0x128 - Bit counter reached bit count value"]
    pub events_bcmatch: EVENTS_BCMATCH,
    _reserved22: [u8; 4usize],
    #[doc = "0x130 - Packet received with CRC ok"]
    pub events_crcok: EVENTS_CRCOK,
    #[doc = "0x134 - Packet received with CRC error"]
    pub events_crcerror: EVENTS_CRCERROR,
    #[doc = "0x138 - IEEE 802.15.4 length field received"]
    pub events_framestart: EVENTS_FRAMESTART,
    #[doc = "0x13c - Sampling of energy detection complete. A new ED sample is ready for readout from the RADIO.EDSAMPLE register."]
    pub events_edend: EVENTS_EDEND,
    #[doc = "0x140 - The sampling of energy detection has stopped"]
    pub events_edstopped: EVENTS_EDSTOPPED,
    #[doc = "0x144 - Wireless medium in idle - clear to send"]
    pub events_ccaidle: EVENTS_CCAIDLE,
    #[doc = "0x148 - Wireless medium busy - do not send"]
    pub events_ccabusy: EVENTS_CCABUSY,
    #[doc = "0x14c - The CCA has stopped"]
    pub events_ccastopped: EVENTS_CCASTOPPED,
    #[doc = "0x150 - Ble_LR CI field received, receive mode is changed from Ble_LR125Kbit to Ble_LR500Kbit."]
    pub events_rateboost: EVENTS_RATEBOOST,
    #[doc = "0x154 - RADIO has ramped up and is ready to be started TX path"]
    pub events_txready: EVENTS_TXREADY,
    #[doc = "0x158 - RADIO has ramped up and is ready to be started RX path"]
    pub events_rxready: EVENTS_RXREADY,
    #[doc = "0x15c - MAC header match found"]
    pub events_mhrmatch: EVENTS_MHRMATCH,
    _reserved34: [u8; 12usize],
    #[doc = "0x16c - Generated in Ble_LR125Kbit, Ble_LR500Kbit and BleIeee802154_250Kbit modes when last bit is sent on air."]
    pub events_phyend: EVENTS_PHYEND,
    _reserved35: [u8; 144usize],
    #[doc = "0x200 - Shortcut register"]
    pub shorts: SHORTS,
    _reserved36: [u8; 256usize],
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved38: [u8; 244usize],
    #[doc = "0x400 - CRC status"]
    pub crcstatus: CRCSTATUS,
    _reserved39: [u8; 4usize],
    #[doc = "0x408 - Received address"]
    pub rxmatch: RXMATCH,
    #[doc = "0x40c - CRC field of previously received packet"]
    pub rxcrc: RXCRC,
    #[doc = "0x410 - Device address match index"]
    pub dai: DAI,
    #[doc = "0x414 - Payload status"]
    pub pdustat: PDUSTAT,
    _reserved43: [u8; 236usize],
    #[doc = "0x504 - Packet pointer"]
    pub packetptr: PACKETPTR,
    #[doc = "0x508 - Frequency"]
    pub frequency: FREQUENCY,
    #[doc = "0x50c - Output power"]
    pub txpower: TXPOWER,
    #[doc = "0x510 - Data rate and modulation"]
    pub mode: MODE,
    #[doc = "0x514 - Packet configuration register 0"]
    pub pcnf0: PCNF0,
    #[doc = "0x518 - Packet configuration register 1"]
    pub pcnf1: PCNF1,
    #[doc = "0x51c - Base address 0"]
    pub base0: BASE0,
    #[doc = "0x520 - Base address 1"]
    pub base1: BASE1,
    #[doc = "0x524 - Prefixes bytes for logical addresses 0-3"]
    pub prefix0: PREFIX0,
    #[doc = "0x528 - Prefixes bytes for logical addresses 4-7"]
    pub prefix1: PREFIX1,
    #[doc = "0x52c - Transmit address select"]
    pub txaddress: TXADDRESS,
    #[doc = "0x530 - Receive address select"]
    pub rxaddresses: RXADDRESSES,
    #[doc = "0x534 - CRC configuration"]
    pub crccnf: CRCCNF,
    #[doc = "0x538 - CRC polynomial"]
    pub crcpoly: CRCPOLY,
    #[doc = "0x53c - CRC initial value"]
    pub crcinit: CRCINIT,
    _reserved58: [u8; 4usize],
    #[doc = "0x544 - Interframe spacing in us"]
    pub tifs: TIFS,
    #[doc = "0x548 - RSSI sample"]
    pub rssisample: RSSISAMPLE,
    _reserved60: [u8; 4usize],
    #[doc = "0x550 - Current radio state"]
    pub state: STATE,
    #[doc = "0x554 - Data whitening initial value"]
    pub datawhiteiv: DATAWHITEIV,
    _reserved62: [u8; 8usize],
    #[doc = "0x560 - Bit counter compare"]
    pub bcc: BCC,
    _reserved63: [u8; 156usize],
    #[doc = "0x600 - Description collection\\[n\\]: Device address base segment n"]
    pub dab: [DAB; 8],
    #[doc = "0x620 - Description collection\\[n\\]: Device address prefix n"]
    pub dap: [DAP; 8],
    #[doc = "0x640 - Device address match configuration"]
    pub dacnf: DACNF,
    #[doc = "0x644 - Search pattern configuration"]
    pub mhrmatchconf: MHRMATCHCONF,
    #[doc = "0x648 - Pattern mask"]
    pub mhrmatchmas: MHRMATCHMAS,
    _reserved68: [u8; 4usize],
    #[doc = "0x650 - Radio mode configuration register 0"]
    pub modecnf0: MODECNF0,
    _reserved69: [u8; 12usize],
    #[doc = "0x660 - IEEE 802.15.4 start of frame delimiter"]
    pub sfd: SFD,
    #[doc = "0x664 - IEEE 802.15.4 energy detect loop count"]
    pub edcnt: EDCNT,
    #[doc = "0x668 - IEEE 802.15.4 energy detect level"]
    pub edsample: EDSAMPLE,
    #[doc = "0x66c - IEEE 802.15.4 clear channel assessment control"]
    pub ccactrl: CCACTRL,
    _reserved73: [u8; 2444usize],
    #[doc = "0xffc - Peripheral power control"]
    pub power: POWER,
}
#[doc = "Enable RADIO in TX mode\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_txen](tasks_txen) module"]
pub type TASKS_TXEN = crate::Reg<u32, _TASKS_TXEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_TXEN;
#[doc = "`write(|w| ..)` method takes [tasks_txen::W](tasks_txen::W) writer structure"]
impl crate::Writable for TASKS_TXEN {}
#[doc = "Enable RADIO in TX mode"]
pub mod tasks_txen;
#[doc = "Enable RADIO in RX mode\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_rxen](tasks_rxen) module"]
pub type TASKS_RXEN = crate::Reg<u32, _TASKS_RXEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_RXEN;
#[doc = "`write(|w| ..)` method takes [tasks_rxen::W](tasks_rxen::W) writer structure"]
impl crate::Writable for TASKS_RXEN {}
#[doc = "Enable RADIO in RX mode"]
pub mod tasks_rxen;
#[doc = "Start RADIO\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_start](tasks_start) module"]
pub type TASKS_START = crate::Reg<u32, _TASKS_START>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_START;
#[doc = "`write(|w| ..)` method takes [tasks_start::W](tasks_start::W) writer structure"]
impl crate::Writable for TASKS_START {}
#[doc = "Start RADIO"]
pub mod tasks_start;
#[doc = "Stop RADIO\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_stop](tasks_stop) module"]
pub type TASKS_STOP = crate::Reg<u32, _TASKS_STOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_STOP;
#[doc = "`write(|w| ..)` method takes [tasks_stop::W](tasks_stop::W) writer structure"]
impl crate::Writable for TASKS_STOP {}
#[doc = "Stop RADIO"]
pub mod tasks_stop;
#[doc = "Disable RADIO\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_disable](tasks_disable) module"]
pub type TASKS_DISABLE = crate::Reg<u32, _TASKS_DISABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_DISABLE;
#[doc = "`write(|w| ..)` method takes [tasks_disable::W](tasks_disable::W) writer structure"]
impl crate::Writable for TASKS_DISABLE {}
#[doc = "Disable RADIO"]
pub mod tasks_disable;
#[doc = "Start the RSSI and take one single sample of the receive signal strength\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_rssistart](tasks_rssistart) module"]
pub type TASKS_RSSISTART = crate::Reg<u32, _TASKS_RSSISTART>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_RSSISTART;
#[doc = "`write(|w| ..)` method takes [tasks_rssistart::W](tasks_rssistart::W) writer structure"]
impl crate::Writable for TASKS_RSSISTART {}
#[doc = "Start the RSSI and take one single sample of the receive signal strength"]
pub mod tasks_rssistart;
#[doc = "Stop the RSSI measurement\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_rssistop](tasks_rssistop) module"]
pub type TASKS_RSSISTOP = crate::Reg<u32, _TASKS_RSSISTOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_RSSISTOP;
#[doc = "`write(|w| ..)` method takes [tasks_rssistop::W](tasks_rssistop::W) writer structure"]
impl crate::Writable for TASKS_RSSISTOP {}
#[doc = "Stop the RSSI measurement"]
pub mod tasks_rssistop;
#[doc = "Start the bit counter\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_bcstart](tasks_bcstart) module"]
pub type TASKS_BCSTART = crate::Reg<u32, _TASKS_BCSTART>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_BCSTART;
#[doc = "`write(|w| ..)` method takes [tasks_bcstart::W](tasks_bcstart::W) writer structure"]
impl crate::Writable for TASKS_BCSTART {}
#[doc = "Start the bit counter"]
pub mod tasks_bcstart;
#[doc = "Stop the bit counter\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_bcstop](tasks_bcstop) module"]
pub type TASKS_BCSTOP = crate::Reg<u32, _TASKS_BCSTOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_BCSTOP;
#[doc = "`write(|w| ..)` method takes [tasks_bcstop::W](tasks_bcstop::W) writer structure"]
impl crate::Writable for TASKS_BCSTOP {}
#[doc = "Stop the bit counter"]
pub mod tasks_bcstop;
#[doc = "Start the energy detect measurement used in IEEE 802.15.4 mode\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_edstart](tasks_edstart) module"]
pub type TASKS_EDSTART = crate::Reg<u32, _TASKS_EDSTART>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_EDSTART;
#[doc = "`write(|w| ..)` method takes [tasks_edstart::W](tasks_edstart::W) writer structure"]
impl crate::Writable for TASKS_EDSTART {}
#[doc = "Start the energy detect measurement used in IEEE 802.15.4 mode"]
pub mod tasks_edstart;
#[doc = "Stop the energy detect measurement\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_edstop](tasks_edstop) module"]
pub type TASKS_EDSTOP = crate::Reg<u32, _TASKS_EDSTOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_EDSTOP;
#[doc = "`write(|w| ..)` method takes [tasks_edstop::W](tasks_edstop::W) writer structure"]
impl crate::Writable for TASKS_EDSTOP {}
#[doc = "Stop the energy detect measurement"]
pub mod tasks_edstop;
#[doc = "Start the clear channel assessment used in IEEE 802.15.4 mode\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_ccastart](tasks_ccastart) module"]
pub type TASKS_CCASTART = crate::Reg<u32, _TASKS_CCASTART>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_CCASTART;
#[doc = "`write(|w| ..)` method takes [tasks_ccastart::W](tasks_ccastart::W) writer structure"]
impl crate::Writable for TASKS_CCASTART {}
#[doc = "Start the clear channel assessment used in IEEE 802.15.4 mode"]
pub mod tasks_ccastart;
#[doc = "Stop the clear channel assessment\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_ccastop](tasks_ccastop) module"]
pub type TASKS_CCASTOP = crate::Reg<u32, _TASKS_CCASTOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_CCASTOP;
#[doc = "`write(|w| ..)` method takes [tasks_ccastop::W](tasks_ccastop::W) writer structure"]
impl crate::Writable for TASKS_CCASTOP {}
#[doc = "Stop the clear channel assessment"]
pub mod tasks_ccastop;
#[doc = "RADIO has ramped up and is ready to be started\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_ready](events_ready) module"]
pub type EVENTS_READY = crate::Reg<u32, _EVENTS_READY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_READY;
#[doc = "`read()` method returns [events_ready::R](events_ready::R) reader structure"]
impl crate::Readable for EVENTS_READY {}
#[doc = "`write(|w| ..)` method takes [events_ready::W](events_ready::W) writer structure"]
impl crate::Writable for EVENTS_READY {}
#[doc = "RADIO has ramped up and is ready to be started"]
pub mod events_ready;
#[doc = "Address sent or received\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_address](events_address) module"]
pub type EVENTS_ADDRESS = crate::Reg<u32, _EVENTS_ADDRESS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_ADDRESS;
#[doc = "`read()` method returns [events_address::R](events_address::R) reader structure"]
impl crate::Readable for EVENTS_ADDRESS {}
#[doc = "`write(|w| ..)` method takes [events_address::W](events_address::W) writer structure"]
impl crate::Writable for EVENTS_ADDRESS {}
#[doc = "Address sent or received"]
pub mod events_address;
#[doc = "Packet payload sent or received\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_payload](events_payload) module"]
pub type EVENTS_PAYLOAD = crate::Reg<u32, _EVENTS_PAYLOAD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_PAYLOAD;
#[doc = "`read()` method returns [events_payload::R](events_payload::R) reader structure"]
impl crate::Readable for EVENTS_PAYLOAD {}
#[doc = "`write(|w| ..)` method takes [events_payload::W](events_payload::W) writer structure"]
impl crate::Writable for EVENTS_PAYLOAD {}
#[doc = "Packet payload sent or received"]
pub mod events_payload;
#[doc = "Packet sent or received\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_end](events_end) module"]
pub type EVENTS_END = crate::Reg<u32, _EVENTS_END>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_END;
#[doc = "`read()` method returns [events_end::R](events_end::R) reader structure"]
impl crate::Readable for EVENTS_END {}
#[doc = "`write(|w| ..)` method takes [events_end::W](events_end::W) writer structure"]
impl crate::Writable for EVENTS_END {}
#[doc = "Packet sent or received"]
pub mod events_end;
#[doc = "RADIO has been disabled\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_disabled](events_disabled) module"]
pub type EVENTS_DISABLED = crate::Reg<u32, _EVENTS_DISABLED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_DISABLED;
#[doc = "`read()` method returns [events_disabled::R](events_disabled::R) reader structure"]
impl crate::Readable for EVENTS_DISABLED {}
#[doc = "`write(|w| ..)` method takes [events_disabled::W](events_disabled::W) writer structure"]
impl crate::Writable for EVENTS_DISABLED {}
#[doc = "RADIO has been disabled"]
pub mod events_disabled;
#[doc = "A device address match occurred on the last received packet\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_devmatch](events_devmatch) module"]
pub type EVENTS_DEVMATCH = crate::Reg<u32, _EVENTS_DEVMATCH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_DEVMATCH;
#[doc = "`read()` method returns [events_devmatch::R](events_devmatch::R) reader structure"]
impl crate::Readable for EVENTS_DEVMATCH {}
#[doc = "`write(|w| ..)` method takes [events_devmatch::W](events_devmatch::W) writer structure"]
impl crate::Writable for EVENTS_DEVMATCH {}
#[doc = "A device address match occurred on the last received packet"]
pub mod events_devmatch;
#[doc = "No device address match occurred on the last received packet\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_devmiss](events_devmiss) module"]
pub type EVENTS_DEVMISS = crate::Reg<u32, _EVENTS_DEVMISS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_DEVMISS;
#[doc = "`read()` method returns [events_devmiss::R](events_devmiss::R) reader structure"]
impl crate::Readable for EVENTS_DEVMISS {}
#[doc = "`write(|w| ..)` method takes [events_devmiss::W](events_devmiss::W) writer structure"]
impl crate::Writable for EVENTS_DEVMISS {}
#[doc = "No device address match occurred on the last received packet"]
pub mod events_devmiss;
#[doc = "Sampling of receive signal strength complete\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_rssiend](events_rssiend) module"]
pub type EVENTS_RSSIEND = crate::Reg<u32, _EVENTS_RSSIEND>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_RSSIEND;
#[doc = "`read()` method returns [events_rssiend::R](events_rssiend::R) reader structure"]
impl crate::Readable for EVENTS_RSSIEND {}
#[doc = "`write(|w| ..)` method takes [events_rssiend::W](events_rssiend::W) writer structure"]
impl crate::Writable for EVENTS_RSSIEND {}
#[doc = "Sampling of receive signal strength complete"]
pub mod events_rssiend;
#[doc = "Bit counter reached bit count value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_bcmatch](events_bcmatch) module"]
pub type EVENTS_BCMATCH = crate::Reg<u32, _EVENTS_BCMATCH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_BCMATCH;
#[doc = "`read()` method returns [events_bcmatch::R](events_bcmatch::R) reader structure"]
impl crate::Readable for EVENTS_BCMATCH {}
#[doc = "`write(|w| ..)` method takes [events_bcmatch::W](events_bcmatch::W) writer structure"]
impl crate::Writable for EVENTS_BCMATCH {}
#[doc = "Bit counter reached bit count value"]
pub mod events_bcmatch;
#[doc = "Packet received with CRC ok\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_crcok](events_crcok) module"]
pub type EVENTS_CRCOK = crate::Reg<u32, _EVENTS_CRCOK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_CRCOK;
#[doc = "`read()` method returns [events_crcok::R](events_crcok::R) reader structure"]
impl crate::Readable for EVENTS_CRCOK {}
#[doc = "`write(|w| ..)` method takes [events_crcok::W](events_crcok::W) writer structure"]
impl crate::Writable for EVENTS_CRCOK {}
#[doc = "Packet received with CRC ok"]
pub mod events_crcok;
#[doc = "Packet received with CRC error\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_crcerror](events_crcerror) module"]
pub type EVENTS_CRCERROR = crate::Reg<u32, _EVENTS_CRCERROR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_CRCERROR;
#[doc = "`read()` method returns [events_crcerror::R](events_crcerror::R) reader structure"]
impl crate::Readable for EVENTS_CRCERROR {}
#[doc = "`write(|w| ..)` method takes [events_crcerror::W](events_crcerror::W) writer structure"]
impl crate::Writable for EVENTS_CRCERROR {}
#[doc = "Packet received with CRC error"]
pub mod events_crcerror;
#[doc = "IEEE 802.15.4 length field received\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_framestart](events_framestart) module"]
pub type EVENTS_FRAMESTART = crate::Reg<u32, _EVENTS_FRAMESTART>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_FRAMESTART;
#[doc = "`read()` method returns [events_framestart::R](events_framestart::R) reader structure"]
impl crate::Readable for EVENTS_FRAMESTART {}
#[doc = "`write(|w| ..)` method takes [events_framestart::W](events_framestart::W) writer structure"]
impl crate::Writable for EVENTS_FRAMESTART {}
#[doc = "IEEE 802.15.4 length field received"]
pub mod events_framestart;
#[doc = "Sampling of energy detection complete. A new ED sample is ready for readout from the RADIO.EDSAMPLE register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_edend](events_edend) module"]
pub type EVENTS_EDEND = crate::Reg<u32, _EVENTS_EDEND>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_EDEND;
#[doc = "`read()` method returns [events_edend::R](events_edend::R) reader structure"]
impl crate::Readable for EVENTS_EDEND {}
#[doc = "`write(|w| ..)` method takes [events_edend::W](events_edend::W) writer structure"]
impl crate::Writable for EVENTS_EDEND {}
#[doc = "Sampling of energy detection complete. A new ED sample is ready for readout from the RADIO.EDSAMPLE register."]
pub mod events_edend;
#[doc = "The sampling of energy detection has stopped\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_edstopped](events_edstopped) module"]
pub type EVENTS_EDSTOPPED = crate::Reg<u32, _EVENTS_EDSTOPPED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_EDSTOPPED;
#[doc = "`read()` method returns [events_edstopped::R](events_edstopped::R) reader structure"]
impl crate::Readable for EVENTS_EDSTOPPED {}
#[doc = "`write(|w| ..)` method takes [events_edstopped::W](events_edstopped::W) writer structure"]
impl crate::Writable for EVENTS_EDSTOPPED {}
#[doc = "The sampling of energy detection has stopped"]
pub mod events_edstopped;
#[doc = "Wireless medium in idle - clear to send\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_ccaidle](events_ccaidle) module"]
pub type EVENTS_CCAIDLE = crate::Reg<u32, _EVENTS_CCAIDLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_CCAIDLE;
#[doc = "`read()` method returns [events_ccaidle::R](events_ccaidle::R) reader structure"]
impl crate::Readable for EVENTS_CCAIDLE {}
#[doc = "`write(|w| ..)` method takes [events_ccaidle::W](events_ccaidle::W) writer structure"]
impl crate::Writable for EVENTS_CCAIDLE {}
#[doc = "Wireless medium in idle - clear to send"]
pub mod events_ccaidle;
#[doc = "Wireless medium busy - do not send\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_ccabusy](events_ccabusy) module"]
pub type EVENTS_CCABUSY = crate::Reg<u32, _EVENTS_CCABUSY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_CCABUSY;
#[doc = "`read()` method returns [events_ccabusy::R](events_ccabusy::R) reader structure"]
impl crate::Readable for EVENTS_CCABUSY {}
#[doc = "`write(|w| ..)` method takes [events_ccabusy::W](events_ccabusy::W) writer structure"]
impl crate::Writable for EVENTS_CCABUSY {}
#[doc = "Wireless medium busy - do not send"]
pub mod events_ccabusy;
#[doc = "The CCA has stopped\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_ccastopped](events_ccastopped) module"]
pub type EVENTS_CCASTOPPED = crate::Reg<u32, _EVENTS_CCASTOPPED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_CCASTOPPED;
#[doc = "`read()` method returns [events_ccastopped::R](events_ccastopped::R) reader structure"]
impl crate::Readable for EVENTS_CCASTOPPED {}
#[doc = "`write(|w| ..)` method takes [events_ccastopped::W](events_ccastopped::W) writer structure"]
impl crate::Writable for EVENTS_CCASTOPPED {}
#[doc = "The CCA has stopped"]
pub mod events_ccastopped;
#[doc = "Ble_LR CI field received, receive mode is changed from Ble_LR125Kbit to Ble_LR500Kbit.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_rateboost](events_rateboost) module"]
pub type EVENTS_RATEBOOST = crate::Reg<u32, _EVENTS_RATEBOOST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_RATEBOOST;
#[doc = "`read()` method returns [events_rateboost::R](events_rateboost::R) reader structure"]
impl crate::Readable for EVENTS_RATEBOOST {}
#[doc = "`write(|w| ..)` method takes [events_rateboost::W](events_rateboost::W) writer structure"]
impl crate::Writable for EVENTS_RATEBOOST {}
#[doc = "Ble_LR CI field received, receive mode is changed from Ble_LR125Kbit to Ble_LR500Kbit."]
pub mod events_rateboost;
#[doc = "RADIO has ramped up and is ready to be started TX path\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_txready](events_txready) module"]
pub type EVENTS_TXREADY = crate::Reg<u32, _EVENTS_TXREADY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_TXREADY;
#[doc = "`read()` method returns [events_txready::R](events_txready::R) reader structure"]
impl crate::Readable for EVENTS_TXREADY {}
#[doc = "`write(|w| ..)` method takes [events_txready::W](events_txready::W) writer structure"]
impl crate::Writable for EVENTS_TXREADY {}
#[doc = "RADIO has ramped up and is ready to be started TX path"]
pub mod events_txready;
#[doc = "RADIO has ramped up and is ready to be started RX path\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_rxready](events_rxready) module"]
pub type EVENTS_RXREADY = crate::Reg<u32, _EVENTS_RXREADY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_RXREADY;
#[doc = "`read()` method returns [events_rxready::R](events_rxready::R) reader structure"]
impl crate::Readable for EVENTS_RXREADY {}
#[doc = "`write(|w| ..)` method takes [events_rxready::W](events_rxready::W) writer structure"]
impl crate::Writable for EVENTS_RXREADY {}
#[doc = "RADIO has ramped up and is ready to be started RX path"]
pub mod events_rxready;
#[doc = "MAC header match found\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_mhrmatch](events_mhrmatch) module"]
pub type EVENTS_MHRMATCH = crate::Reg<u32, _EVENTS_MHRMATCH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_MHRMATCH;
#[doc = "`read()` method returns [events_mhrmatch::R](events_mhrmatch::R) reader structure"]
impl crate::Readable for EVENTS_MHRMATCH {}
#[doc = "`write(|w| ..)` method takes [events_mhrmatch::W](events_mhrmatch::W) writer structure"]
impl crate::Writable for EVENTS_MHRMATCH {}
#[doc = "MAC header match found"]
pub mod events_mhrmatch;
#[doc = "Generated in Ble_LR125Kbit, Ble_LR500Kbit and BleIeee802154_250Kbit modes when last bit is sent on air.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_phyend](events_phyend) module"]
pub type EVENTS_PHYEND = crate::Reg<u32, _EVENTS_PHYEND>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_PHYEND;
#[doc = "`read()` method returns [events_phyend::R](events_phyend::R) reader structure"]
impl crate::Readable for EVENTS_PHYEND {}
#[doc = "`write(|w| ..)` method takes [events_phyend::W](events_phyend::W) writer structure"]
impl crate::Writable for EVENTS_PHYEND {}
#[doc = "Generated in Ble_LR125Kbit, Ble_LR500Kbit and BleIeee802154_250Kbit modes when last bit is sent on air."]
pub mod events_phyend;
#[doc = "Shortcut register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shorts](shorts) module"]
pub type SHORTS = crate::Reg<u32, _SHORTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHORTS;
#[doc = "`read()` method returns [shorts::R](shorts::R) reader structure"]
impl crate::Readable for SHORTS {}
#[doc = "`write(|w| ..)` method takes [shorts::W](shorts::W) writer structure"]
impl crate::Writable for SHORTS {}
#[doc = "Shortcut register"]
pub mod shorts;
#[doc = "Enable interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenset](intenset) module"]
pub type INTENSET = crate::Reg<u32, _INTENSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTENSET;
#[doc = "`read()` method returns [intenset::R](intenset::R) reader structure"]
impl crate::Readable for INTENSET {}
#[doc = "`write(|w| ..)` method takes [intenset::W](intenset::W) writer structure"]
impl crate::Writable for INTENSET {}
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "Disable interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenclr](intenclr) module"]
pub type INTENCLR = crate::Reg<u32, _INTENCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTENCLR;
#[doc = "`read()` method returns [intenclr::R](intenclr::R) reader structure"]
impl crate::Readable for INTENCLR {}
#[doc = "`write(|w| ..)` method takes [intenclr::W](intenclr::W) writer structure"]
impl crate::Writable for INTENCLR {}
#[doc = "Disable interrupt"]
pub mod intenclr;
#[doc = "CRC status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crcstatus](crcstatus) module"]
pub type CRCSTATUS = crate::Reg<u32, _CRCSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRCSTATUS;
#[doc = "`read()` method returns [crcstatus::R](crcstatus::R) reader structure"]
impl crate::Readable for CRCSTATUS {}
#[doc = "CRC status"]
pub mod crcstatus;
#[doc = "Received address\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxmatch](rxmatch) module"]
pub type RXMATCH = crate::Reg<u32, _RXMATCH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXMATCH;
#[doc = "`read()` method returns [rxmatch::R](rxmatch::R) reader structure"]
impl crate::Readable for RXMATCH {}
#[doc = "Received address"]
pub mod rxmatch;
#[doc = "CRC field of previously received packet\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxcrc](rxcrc) module"]
pub type RXCRC = crate::Reg<u32, _RXCRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXCRC;
#[doc = "`read()` method returns [rxcrc::R](rxcrc::R) reader structure"]
impl crate::Readable for RXCRC {}
#[doc = "CRC field of previously received packet"]
pub mod rxcrc;
#[doc = "Device address match index\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dai](dai) module"]
pub type DAI = crate::Reg<u32, _DAI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAI;
#[doc = "`read()` method returns [dai::R](dai::R) reader structure"]
impl crate::Readable for DAI {}
#[doc = "Device address match index"]
pub mod dai;
#[doc = "Payload status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdustat](pdustat) module"]
pub type PDUSTAT = crate::Reg<u32, _PDUSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDUSTAT;
#[doc = "`read()` method returns [pdustat::R](pdustat::R) reader structure"]
impl crate::Readable for PDUSTAT {}
#[doc = "Payload status"]
pub mod pdustat;
#[doc = "Packet pointer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [packetptr](packetptr) module"]
pub type PACKETPTR = crate::Reg<u32, _PACKETPTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PACKETPTR;
#[doc = "`read()` method returns [packetptr::R](packetptr::R) reader structure"]
impl crate::Readable for PACKETPTR {}
#[doc = "`write(|w| ..)` method takes [packetptr::W](packetptr::W) writer structure"]
impl crate::Writable for PACKETPTR {}
#[doc = "Packet pointer"]
pub mod packetptr;
#[doc = "Frequency\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frequency](frequency) module"]
pub type FREQUENCY = crate::Reg<u32, _FREQUENCY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FREQUENCY;
#[doc = "`read()` method returns [frequency::R](frequency::R) reader structure"]
impl crate::Readable for FREQUENCY {}
#[doc = "`write(|w| ..)` method takes [frequency::W](frequency::W) writer structure"]
impl crate::Writable for FREQUENCY {}
#[doc = "Frequency"]
pub mod frequency;
#[doc = "Output power\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txpower](txpower) module"]
pub type TXPOWER = crate::Reg<u32, _TXPOWER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXPOWER;
#[doc = "`read()` method returns [txpower::R](txpower::R) reader structure"]
impl crate::Readable for TXPOWER {}
#[doc = "`write(|w| ..)` method takes [txpower::W](txpower::W) writer structure"]
impl crate::Writable for TXPOWER {}
#[doc = "Output power"]
pub mod txpower;
#[doc = "Data rate and modulation\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mode](mode) module"]
pub type MODE = crate::Reg<u32, _MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MODE;
#[doc = "`read()` method returns [mode::R](mode::R) reader structure"]
impl crate::Readable for MODE {}
#[doc = "`write(|w| ..)` method takes [mode::W](mode::W) writer structure"]
impl crate::Writable for MODE {}
#[doc = "Data rate and modulation"]
pub mod mode;
#[doc = "Packet configuration register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcnf0](pcnf0) module"]
pub type PCNF0 = crate::Reg<u32, _PCNF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCNF0;
#[doc = "`read()` method returns [pcnf0::R](pcnf0::R) reader structure"]
impl crate::Readable for PCNF0 {}
#[doc = "`write(|w| ..)` method takes [pcnf0::W](pcnf0::W) writer structure"]
impl crate::Writable for PCNF0 {}
#[doc = "Packet configuration register 0"]
pub mod pcnf0;
#[doc = "Packet configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcnf1](pcnf1) module"]
pub type PCNF1 = crate::Reg<u32, _PCNF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCNF1;
#[doc = "`read()` method returns [pcnf1::R](pcnf1::R) reader structure"]
impl crate::Readable for PCNF1 {}
#[doc = "`write(|w| ..)` method takes [pcnf1::W](pcnf1::W) writer structure"]
impl crate::Writable for PCNF1 {}
#[doc = "Packet configuration register 1"]
pub mod pcnf1;
#[doc = "Base address 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [base0](base0) module"]
pub type BASE0 = crate::Reg<u32, _BASE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BASE0;
#[doc = "`read()` method returns [base0::R](base0::R) reader structure"]
impl crate::Readable for BASE0 {}
#[doc = "`write(|w| ..)` method takes [base0::W](base0::W) writer structure"]
impl crate::Writable for BASE0 {}
#[doc = "Base address 0"]
pub mod base0;
#[doc = "Base address 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [base1](base1) module"]
pub type BASE1 = crate::Reg<u32, _BASE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BASE1;
#[doc = "`read()` method returns [base1::R](base1::R) reader structure"]
impl crate::Readable for BASE1 {}
#[doc = "`write(|w| ..)` method takes [base1::W](base1::W) writer structure"]
impl crate::Writable for BASE1 {}
#[doc = "Base address 1"]
pub mod base1;
#[doc = "Prefixes bytes for logical addresses 0-3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prefix0](prefix0) module"]
pub type PREFIX0 = crate::Reg<u32, _PREFIX0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PREFIX0;
#[doc = "`read()` method returns [prefix0::R](prefix0::R) reader structure"]
impl crate::Readable for PREFIX0 {}
#[doc = "`write(|w| ..)` method takes [prefix0::W](prefix0::W) writer structure"]
impl crate::Writable for PREFIX0 {}
#[doc = "Prefixes bytes for logical addresses 0-3"]
pub mod prefix0;
#[doc = "Prefixes bytes for logical addresses 4-7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prefix1](prefix1) module"]
pub type PREFIX1 = crate::Reg<u32, _PREFIX1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PREFIX1;
#[doc = "`read()` method returns [prefix1::R](prefix1::R) reader structure"]
impl crate::Readable for PREFIX1 {}
#[doc = "`write(|w| ..)` method takes [prefix1::W](prefix1::W) writer structure"]
impl crate::Writable for PREFIX1 {}
#[doc = "Prefixes bytes for logical addresses 4-7"]
pub mod prefix1;
#[doc = "Transmit address select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txaddress](txaddress) module"]
pub type TXADDRESS = crate::Reg<u32, _TXADDRESS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXADDRESS;
#[doc = "`read()` method returns [txaddress::R](txaddress::R) reader structure"]
impl crate::Readable for TXADDRESS {}
#[doc = "`write(|w| ..)` method takes [txaddress::W](txaddress::W) writer structure"]
impl crate::Writable for TXADDRESS {}
#[doc = "Transmit address select"]
pub mod txaddress;
#[doc = "Receive address select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxaddresses](rxaddresses) module"]
pub type RXADDRESSES = crate::Reg<u32, _RXADDRESSES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXADDRESSES;
#[doc = "`read()` method returns [rxaddresses::R](rxaddresses::R) reader structure"]
impl crate::Readable for RXADDRESSES {}
#[doc = "`write(|w| ..)` method takes [rxaddresses::W](rxaddresses::W) writer structure"]
impl crate::Writable for RXADDRESSES {}
#[doc = "Receive address select"]
pub mod rxaddresses;
#[doc = "CRC configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crccnf](crccnf) module"]
pub type CRCCNF = crate::Reg<u32, _CRCCNF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRCCNF;
#[doc = "`read()` method returns [crccnf::R](crccnf::R) reader structure"]
impl crate::Readable for CRCCNF {}
#[doc = "`write(|w| ..)` method takes [crccnf::W](crccnf::W) writer structure"]
impl crate::Writable for CRCCNF {}
#[doc = "CRC configuration"]
pub mod crccnf;
#[doc = "CRC polynomial\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crcpoly](crcpoly) module"]
pub type CRCPOLY = crate::Reg<u32, _CRCPOLY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRCPOLY;
#[doc = "`read()` method returns [crcpoly::R](crcpoly::R) reader structure"]
impl crate::Readable for CRCPOLY {}
#[doc = "`write(|w| ..)` method takes [crcpoly::W](crcpoly::W) writer structure"]
impl crate::Writable for CRCPOLY {}
#[doc = "CRC polynomial"]
pub mod crcpoly;
#[doc = "CRC initial value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crcinit](crcinit) module"]
pub type CRCINIT = crate::Reg<u32, _CRCINIT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRCINIT;
#[doc = "`read()` method returns [crcinit::R](crcinit::R) reader structure"]
impl crate::Readable for CRCINIT {}
#[doc = "`write(|w| ..)` method takes [crcinit::W](crcinit::W) writer structure"]
impl crate::Writable for CRCINIT {}
#[doc = "CRC initial value"]
pub mod crcinit;
#[doc = "Interframe spacing in us\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tifs](tifs) module"]
pub type TIFS = crate::Reg<u32, _TIFS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIFS;
#[doc = "`read()` method returns [tifs::R](tifs::R) reader structure"]
impl crate::Readable for TIFS {}
#[doc = "`write(|w| ..)` method takes [tifs::W](tifs::W) writer structure"]
impl crate::Writable for TIFS {}
#[doc = "Interframe spacing in us"]
pub mod tifs;
#[doc = "RSSI sample\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rssisample](rssisample) module"]
pub type RSSISAMPLE = crate::Reg<u32, _RSSISAMPLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSSISAMPLE;
#[doc = "`read()` method returns [rssisample::R](rssisample::R) reader structure"]
impl crate::Readable for RSSISAMPLE {}
#[doc = "RSSI sample"]
pub mod rssisample;
#[doc = "Current radio state\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [state](state) module"]
pub type STATE = crate::Reg<u32, _STATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATE;
#[doc = "`read()` method returns [state::R](state::R) reader structure"]
impl crate::Readable for STATE {}
#[doc = "Current radio state"]
pub mod state;
#[doc = "Data whitening initial value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [datawhiteiv](datawhiteiv) module"]
pub type DATAWHITEIV = crate::Reg<u32, _DATAWHITEIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATAWHITEIV;
#[doc = "`read()` method returns [datawhiteiv::R](datawhiteiv::R) reader structure"]
impl crate::Readable for DATAWHITEIV {}
#[doc = "`write(|w| ..)` method takes [datawhiteiv::W](datawhiteiv::W) writer structure"]
impl crate::Writable for DATAWHITEIV {}
#[doc = "Data whitening initial value"]
pub mod datawhiteiv;
#[doc = "Bit counter compare\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bcc](bcc) module"]
pub type BCC = crate::Reg<u32, _BCC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BCC;
#[doc = "`read()` method returns [bcc::R](bcc::R) reader structure"]
impl crate::Readable for BCC {}
#[doc = "`write(|w| ..)` method takes [bcc::W](bcc::W) writer structure"]
impl crate::Writable for BCC {}
#[doc = "Bit counter compare"]
pub mod bcc;
#[doc = "Description collection\\[n\\]: Device address base segment n\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dab](dab) module"]
pub type DAB = crate::Reg<u32, _DAB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAB;
#[doc = "`read()` method returns [dab::R](dab::R) reader structure"]
impl crate::Readable for DAB {}
#[doc = "`write(|w| ..)` method takes [dab::W](dab::W) writer structure"]
impl crate::Writable for DAB {}
#[doc = "Description collection\\[n\\]: Device address base segment n"]
pub mod dab;
#[doc = "Description collection\\[n\\]: Device address prefix n\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dap](dap) module"]
pub type DAP = crate::Reg<u32, _DAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAP;
#[doc = "`read()` method returns [dap::R](dap::R) reader structure"]
impl crate::Readable for DAP {}
#[doc = "`write(|w| ..)` method takes [dap::W](dap::W) writer structure"]
impl crate::Writable for DAP {}
#[doc = "Description collection\\[n\\]: Device address prefix n"]
pub mod dap;
#[doc = "Device address match configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dacnf](dacnf) module"]
pub type DACNF = crate::Reg<u32, _DACNF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DACNF;
#[doc = "`read()` method returns [dacnf::R](dacnf::R) reader structure"]
impl crate::Readable for DACNF {}
#[doc = "`write(|w| ..)` method takes [dacnf::W](dacnf::W) writer structure"]
impl crate::Writable for DACNF {}
#[doc = "Device address match configuration"]
pub mod dacnf;
#[doc = "Search pattern configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mhrmatchconf](mhrmatchconf) module"]
pub type MHRMATCHCONF = crate::Reg<u32, _MHRMATCHCONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MHRMATCHCONF;
#[doc = "`read()` method returns [mhrmatchconf::R](mhrmatchconf::R) reader structure"]
impl crate::Readable for MHRMATCHCONF {}
#[doc = "`write(|w| ..)` method takes [mhrmatchconf::W](mhrmatchconf::W) writer structure"]
impl crate::Writable for MHRMATCHCONF {}
#[doc = "Search pattern configuration"]
pub mod mhrmatchconf;
#[doc = "Pattern mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mhrmatchmas](mhrmatchmas) module"]
pub type MHRMATCHMAS = crate::Reg<u32, _MHRMATCHMAS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MHRMATCHMAS;
#[doc = "`read()` method returns [mhrmatchmas::R](mhrmatchmas::R) reader structure"]
impl crate::Readable for MHRMATCHMAS {}
#[doc = "`write(|w| ..)` method takes [mhrmatchmas::W](mhrmatchmas::W) writer structure"]
impl crate::Writable for MHRMATCHMAS {}
#[doc = "Pattern mask"]
pub mod mhrmatchmas;
#[doc = "Radio mode configuration register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [modecnf0](modecnf0) module"]
pub type MODECNF0 = crate::Reg<u32, _MODECNF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MODECNF0;
#[doc = "`read()` method returns [modecnf0::R](modecnf0::R) reader structure"]
impl crate::Readable for MODECNF0 {}
#[doc = "`write(|w| ..)` method takes [modecnf0::W](modecnf0::W) writer structure"]
impl crate::Writable for MODECNF0 {}
#[doc = "Radio mode configuration register 0"]
pub mod modecnf0;
#[doc = "IEEE 802.15.4 start of frame delimiter\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sfd](sfd) module"]
pub type SFD = crate::Reg<u32, _SFD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SFD;
#[doc = "`read()` method returns [sfd::R](sfd::R) reader structure"]
impl crate::Readable for SFD {}
#[doc = "`write(|w| ..)` method takes [sfd::W](sfd::W) writer structure"]
impl crate::Writable for SFD {}
#[doc = "IEEE 802.15.4 start of frame delimiter"]
pub mod sfd;
#[doc = "IEEE 802.15.4 energy detect loop count\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [edcnt](edcnt) module"]
pub type EDCNT = crate::Reg<u32, _EDCNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EDCNT;
#[doc = "`read()` method returns [edcnt::R](edcnt::R) reader structure"]
impl crate::Readable for EDCNT {}
#[doc = "`write(|w| ..)` method takes [edcnt::W](edcnt::W) writer structure"]
impl crate::Writable for EDCNT {}
#[doc = "IEEE 802.15.4 energy detect loop count"]
pub mod edcnt;
#[doc = "IEEE 802.15.4 energy detect level\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [edsample](edsample) module"]
pub type EDSAMPLE = crate::Reg<u32, _EDSAMPLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EDSAMPLE;
#[doc = "`read()` method returns [edsample::R](edsample::R) reader structure"]
impl crate::Readable for EDSAMPLE {}
#[doc = "`write(|w| ..)` method takes [edsample::W](edsample::W) writer structure"]
impl crate::Writable for EDSAMPLE {}
#[doc = "IEEE 802.15.4 energy detect level"]
pub mod edsample;
#[doc = "IEEE 802.15.4 clear channel assessment control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccactrl](ccactrl) module"]
pub type CCACTRL = crate::Reg<u32, _CCACTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCACTRL;
#[doc = "`read()` method returns [ccactrl::R](ccactrl::R) reader structure"]
impl crate::Readable for CCACTRL {}
#[doc = "`write(|w| ..)` method takes [ccactrl::W](ccactrl::W) writer structure"]
impl crate::Writable for CCACTRL {}
#[doc = "IEEE 802.15.4 clear channel assessment control"]
pub mod ccactrl;
#[doc = "Peripheral power control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [power](power) module"]
pub type POWER = crate::Reg<u32, _POWER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _POWER;
#[doc = "`read()` method returns [power::R](power::R) reader structure"]
impl crate::Readable for POWER {}
#[doc = "`write(|w| ..)` method takes [power::W](power::W) writer structure"]
impl crate::Writable for POWER {}
#[doc = "Peripheral power control"]
pub mod power;
