#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (0303941 2025-02-18))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USBHS {
    ptr: *mut u8,
}
unsafe impl Send for USBHS {}
unsafe impl Sync for USBHS {}
impl USBHS {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn ID(self) -> crate::common::Reg<regs::ID, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn HWGENERAL(self) -> crate::common::Reg<regs::HWGENERAL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn HWHOST(self) -> crate::common::Reg<regs::HWHOST, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn HWDEVICE(self) -> crate::common::Reg<regs::HWDEVICE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn HWTXBUF(self) -> crate::common::Reg<regs::HWTXBUF, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn HWRXBUF(self) -> crate::common::Reg<regs::HWRXBUF, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn GPTIMER0LD(self) -> crate::common::Reg<regs::GPTIMER0LD, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[inline(always)]
    pub const fn GPTIMER0CTRL(self) -> crate::common::Reg<regs::GPTIMER0CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[inline(always)]
    pub const fn GPTIMER1LD(self) -> crate::common::Reg<regs::GPTIMER1LD, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[inline(always)]
    pub const fn GPTIMER1CTRL(self) -> crate::common::Reg<regs::GPTIMER1CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8cusize) as _) }
    }
    #[inline(always)]
    pub const fn SBUSCFG(self) -> crate::common::Reg<regs::SBUSCFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize) as _) }
    }
    #[inline(always)]
    pub const fn CAPLENGTH(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[inline(always)]
    pub const fn HCIVERSION(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0102usize) as _) }
    }
    #[inline(always)]
    pub const fn HCSPARAMS(self) -> crate::common::Reg<regs::HCSPARAMS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[inline(always)]
    pub const fn HCCPARAMS(self) -> crate::common::Reg<regs::HCCPARAMS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[inline(always)]
    pub const fn DCIVERSION(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0120usize) as _) }
    }
    #[inline(always)]
    pub const fn DCCPARAMS(self) -> crate::common::Reg<regs::DCCPARAMS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0124usize) as _) }
    }
    #[inline(always)]
    pub const fn USBCMD(self) -> crate::common::Reg<regs::USBCMD, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0140usize) as _) }
    }
    #[inline(always)]
    pub const fn USBSTS(self) -> crate::common::Reg<regs::USBSTS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0144usize) as _) }
    }
    #[inline(always)]
    pub const fn USBINTR(self) -> crate::common::Reg<regs::USBINTR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0148usize) as _) }
    }
    #[inline(always)]
    pub const fn FRINDEX(self) -> crate::common::Reg<regs::FRINDEX, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x014cusize) as _) }
    }
    #[inline(always)]
    pub const fn DEVICEADDR(self) -> crate::common::Reg<regs::DEVICEADDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0154usize) as _) }
    }
    #[inline(always)]
    pub const fn PERIODICLISTBASE(
        self,
    ) -> crate::common::Reg<regs::PERIODICLISTBASE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0154usize) as _) }
    }
    #[inline(always)]
    pub const fn ASYNCLISTADDR(self) -> crate::common::Reg<regs::ASYNCLISTADDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0158usize) as _) }
    }
    #[inline(always)]
    pub const fn ENDPTLISTADDR(self) -> crate::common::Reg<regs::ENDPTLISTADDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0158usize) as _) }
    }
    #[inline(always)]
    pub const fn BURSTSIZE(self) -> crate::common::Reg<regs::BURSTSIZE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0160usize) as _) }
    }
    #[inline(always)]
    pub const fn TXFILLTUNING(self) -> crate::common::Reg<regs::TXFILLTUNING, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0164usize) as _) }
    }
    #[inline(always)]
    pub const fn ENDPTNAK(self) -> crate::common::Reg<regs::ENDPTNAK, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0178usize) as _) }
    }
    #[inline(always)]
    pub const fn ENDPTNAKEN(self) -> crate::common::Reg<regs::ENDPTNAKEN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x017cusize) as _) }
    }
    #[inline(always)]
    pub const fn CONFIGFLAG(self) -> crate::common::Reg<regs::CONFIGFLAG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0180usize) as _) }
    }
    #[inline(always)]
    pub const fn PORTSC1(self) -> crate::common::Reg<regs::PORTSC1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0184usize) as _) }
    }
    #[inline(always)]
    pub const fn OTGSC(self) -> crate::common::Reg<regs::OTGSC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01a4usize) as _) }
    }
    #[inline(always)]
    pub const fn USBMODE(self) -> crate::common::Reg<regs::USBMODE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01a8usize) as _) }
    }
    #[inline(always)]
    pub const fn ENDPTSETUPSTAT(
        self,
    ) -> crate::common::Reg<regs::ENDPTSETUPSTAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01acusize) as _) }
    }
    #[inline(always)]
    pub const fn ENDPTPRIME(self) -> crate::common::Reg<regs::ENDPTPRIME, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01b0usize) as _) }
    }
    #[inline(always)]
    pub const fn ENDPTFLUSH(self) -> crate::common::Reg<regs::ENDPTFLUSH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01b4usize) as _) }
    }
    #[inline(always)]
    pub const fn ENDPTSTAT(self) -> crate::common::Reg<regs::ENDPTSTAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01b8usize) as _) }
    }
    #[inline(always)]
    pub const fn ENDPTCOMPLETE(self) -> crate::common::Reg<regs::ENDPTCOMPLETE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01bcusize) as _) }
    }
    #[inline(always)]
    pub const fn ENDPTCTRL0(self) -> crate::common::Reg<regs::ENDPTCTRL0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01c0usize) as _) }
    }
    #[inline(always)]
    pub const fn ENDPTCTRL(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::ENDPTCTRL, crate::common::RW> {
        assert!(n < 7usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01c4usize + n * 4usize) as _) }
    }
}
pub mod regs {
    #[doc = "Next Asynch. Address"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ASYNCLISTADDR(pub u32);
    impl ASYNCLISTADDR {
        #[inline(always)]
        pub const fn ASYBASE(&self) -> u32 {
            let val = (self.0 >> 5usize) & 0x07ff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_ASYBASE(&mut self, val: u32) {
            self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
        }
    }
    impl Default for ASYNCLISTADDR {
        #[inline(always)]
        fn default() -> ASYNCLISTADDR {
            ASYNCLISTADDR(0)
        }
    }
    impl core::fmt::Debug for ASYNCLISTADDR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ASYNCLISTADDR")
                .field("ASYBASE", &self.ASYBASE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ASYNCLISTADDR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ASYNCLISTADDR {{ ASYBASE: {=u32:?} }}", self.ASYBASE())
        }
    }
    #[doc = "Programmable Burst Size"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BURSTSIZE(pub u32);
    impl BURSTSIZE {
        #[inline(always)]
        pub const fn RXPBURST(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_RXPBURST(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn TXPBURST(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_TXPBURST(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for BURSTSIZE {
        #[inline(always)]
        fn default() -> BURSTSIZE {
            BURSTSIZE(0)
        }
    }
    impl core::fmt::Debug for BURSTSIZE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BURSTSIZE")
                .field("RXPBURST", &self.RXPBURST())
                .field("TXPBURST", &self.TXPBURST())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BURSTSIZE {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "BURSTSIZE {{ RXPBURST: {=u8:?}, TXPBURST: {=u8:?} }}",
                self.RXPBURST(),
                self.TXPBURST()
            )
        }
    }
    #[doc = "Configure Flag"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CONFIGFLAG(pub u32);
    impl CONFIGFLAG {
        #[inline(always)]
        pub const fn CF(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for CONFIGFLAG {
        #[inline(always)]
        fn default() -> CONFIGFLAG {
            CONFIGFLAG(0)
        }
    }
    impl core::fmt::Debug for CONFIGFLAG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CONFIGFLAG")
                .field("CF", &self.CF())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CONFIGFLAG {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "CONFIGFLAG {{ CF: {=bool:?} }}", self.CF())
        }
    }
    #[doc = "Device Controller Capability Parameters"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DCCPARAMS(pub u32);
    impl DCCPARAMS {
        #[inline(always)]
        pub const fn DEN(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_DEN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[inline(always)]
        pub const fn DC(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn HC(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for DCCPARAMS {
        #[inline(always)]
        fn default() -> DCCPARAMS {
            DCCPARAMS(0)
        }
    }
    impl core::fmt::Debug for DCCPARAMS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DCCPARAMS")
                .field("DEN", &self.DEN())
                .field("DC", &self.DC())
                .field("HC", &self.HC())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DCCPARAMS {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DCCPARAMS {{ DEN: {=u8:?}, DC: {=bool:?}, HC: {=bool:?} }}",
                self.DEN(),
                self.DC(),
                self.HC()
            )
        }
    }
    #[doc = "Device Address"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DEVICEADDR(pub u32);
    impl DEVICEADDR {
        #[inline(always)]
        pub const fn USBADRA(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_USBADRA(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn USBADR(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub fn set_USBADR(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 25usize)) | (((val as u32) & 0x7f) << 25usize);
        }
    }
    impl Default for DEVICEADDR {
        #[inline(always)]
        fn default() -> DEVICEADDR {
            DEVICEADDR(0)
        }
    }
    impl core::fmt::Debug for DEVICEADDR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DEVICEADDR")
                .field("USBADRA", &self.USBADRA())
                .field("USBADR", &self.USBADR())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DEVICEADDR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DEVICEADDR {{ USBADRA: {=bool:?}, USBADR: {=u8:?} }}",
                self.USBADRA(),
                self.USBADR()
            )
        }
    }
    #[doc = "Endpoint Complete"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ENDPTCOMPLETE(pub u32);
    impl ENDPTCOMPLETE {
        #[inline(always)]
        pub const fn ERCE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_ERCE(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn ETCE(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_ETCE(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for ENDPTCOMPLETE {
        #[inline(always)]
        fn default() -> ENDPTCOMPLETE {
            ENDPTCOMPLETE(0)
        }
    }
    impl core::fmt::Debug for ENDPTCOMPLETE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ENDPTCOMPLETE")
                .field("ERCE", &self.ERCE())
                .field("ETCE", &self.ETCE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ENDPTCOMPLETE {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ENDPTCOMPLETE {{ ERCE: {=u8:?}, ETCE: {=u8:?} }}",
                self.ERCE(),
                self.ETCE()
            )
        }
    }
    #[doc = "Endpoint Control 1..Endpoint Control 7"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ENDPTCTRL(pub u32);
    impl ENDPTCTRL {
        #[inline(always)]
        pub const fn RXS(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RXS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn RXD(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RXD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn RXT(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_RXT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[inline(always)]
        pub const fn RXI(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RXI(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn RXR(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RXR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn RXE(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RXE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn TXS(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TXS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn TXD(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TXD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn TXT(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_TXT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[inline(always)]
        pub const fn TXI(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TXI(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn TXR(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TXR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn TXE(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TXE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for ENDPTCTRL {
        #[inline(always)]
        fn default() -> ENDPTCTRL {
            ENDPTCTRL(0)
        }
    }
    impl core::fmt::Debug for ENDPTCTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ENDPTCTRL")
                .field("RXS", &self.RXS())
                .field("RXD", &self.RXD())
                .field("RXT", &self.RXT())
                .field("RXI", &self.RXI())
                .field("RXR", &self.RXR())
                .field("RXE", &self.RXE())
                .field("TXS", &self.TXS())
                .field("TXD", &self.TXD())
                .field("TXT", &self.TXT())
                .field("TXI", &self.TXI())
                .field("TXR", &self.TXR())
                .field("TXE", &self.TXE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ENDPTCTRL {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "ENDPTCTRL {{ RXS: {=bool:?}, RXD: {=bool:?}, RXT: {=u8:?}, RXI: {=bool:?}, RXR: {=bool:?}, RXE: {=bool:?}, TXS: {=bool:?}, TXD: {=bool:?}, TXT: {=u8:?}, TXI: {=bool:?}, TXR: {=bool:?}, TXE: {=bool:?} }}" , self . RXS () , self . RXD () , self . RXT () , self . RXI () , self . RXR () , self . RXE () , self . TXS () , self . TXD () , self . TXT () , self . TXI () , self . TXR () , self . TXE ())
        }
    }
    #[doc = "Endpoint Control 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ENDPTCTRL0(pub u32);
    impl ENDPTCTRL0 {
        #[inline(always)]
        pub const fn RXS(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RXS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn RXT(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_RXT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[inline(always)]
        pub const fn RXE(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RXE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn TXS(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TXS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn TXT(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_TXT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[inline(always)]
        pub const fn TXE(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TXE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for ENDPTCTRL0 {
        #[inline(always)]
        fn default() -> ENDPTCTRL0 {
            ENDPTCTRL0(0)
        }
    }
    impl core::fmt::Debug for ENDPTCTRL0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ENDPTCTRL0")
                .field("RXS", &self.RXS())
                .field("RXT", &self.RXT())
                .field("RXE", &self.RXE())
                .field("TXS", &self.TXS())
                .field("TXT", &self.TXT())
                .field("TXE", &self.TXE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ENDPTCTRL0 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "ENDPTCTRL0 {{ RXS: {=bool:?}, RXT: {=u8:?}, RXE: {=bool:?}, TXS: {=bool:?}, TXT: {=u8:?}, TXE: {=bool:?} }}" , self . RXS () , self . RXT () , self . RXE () , self . TXS () , self . TXT () , self . TXE ())
        }
    }
    #[doc = "Endpoint Flush"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ENDPTFLUSH(pub u32);
    impl ENDPTFLUSH {
        #[inline(always)]
        pub const fn FERB(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_FERB(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn FETB(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_FETB(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for ENDPTFLUSH {
        #[inline(always)]
        fn default() -> ENDPTFLUSH {
            ENDPTFLUSH(0)
        }
    }
    impl core::fmt::Debug for ENDPTFLUSH {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ENDPTFLUSH")
                .field("FERB", &self.FERB())
                .field("FETB", &self.FETB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ENDPTFLUSH {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ENDPTFLUSH {{ FERB: {=u8:?}, FETB: {=u8:?} }}",
                self.FERB(),
                self.FETB()
            )
        }
    }
    #[doc = "Endpoint List Address"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ENDPTLISTADDR(pub u32);
    impl ENDPTLISTADDR {
        #[inline(always)]
        pub const fn EPBASE(&self) -> u32 {
            let val = (self.0 >> 11usize) & 0x001f_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_EPBASE(&mut self, val: u32) {
            self.0 =
                (self.0 & !(0x001f_ffff << 11usize)) | (((val as u32) & 0x001f_ffff) << 11usize);
        }
    }
    impl Default for ENDPTLISTADDR {
        #[inline(always)]
        fn default() -> ENDPTLISTADDR {
            ENDPTLISTADDR(0)
        }
    }
    impl core::fmt::Debug for ENDPTLISTADDR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ENDPTLISTADDR")
                .field("EPBASE", &self.EPBASE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ENDPTLISTADDR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ENDPTLISTADDR {{ EPBASE: {=u32:?} }}", self.EPBASE())
        }
    }
    #[doc = "Endpoint NAK"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ENDPTNAK(pub u32);
    impl ENDPTNAK {
        #[inline(always)]
        pub const fn EPRN(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_EPRN(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn EPTN(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_EPTN(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for ENDPTNAK {
        #[inline(always)]
        fn default() -> ENDPTNAK {
            ENDPTNAK(0)
        }
    }
    impl core::fmt::Debug for ENDPTNAK {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ENDPTNAK")
                .field("EPRN", &self.EPRN())
                .field("EPTN", &self.EPTN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ENDPTNAK {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ENDPTNAK {{ EPRN: {=u8:?}, EPTN: {=u8:?} }}",
                self.EPRN(),
                self.EPTN()
            )
        }
    }
    #[doc = "Endpoint NAK Enable"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ENDPTNAKEN(pub u32);
    impl ENDPTNAKEN {
        #[inline(always)]
        pub const fn EPRNE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_EPRNE(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn EPTNE(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_EPTNE(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for ENDPTNAKEN {
        #[inline(always)]
        fn default() -> ENDPTNAKEN {
            ENDPTNAKEN(0)
        }
    }
    impl core::fmt::Debug for ENDPTNAKEN {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ENDPTNAKEN")
                .field("EPRNE", &self.EPRNE())
                .field("EPTNE", &self.EPTNE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ENDPTNAKEN {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ENDPTNAKEN {{ EPRNE: {=u8:?}, EPTNE: {=u8:?} }}",
                self.EPRNE(),
                self.EPTNE()
            )
        }
    }
    #[doc = "Endpoint Prime"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ENDPTPRIME(pub u32);
    impl ENDPTPRIME {
        #[inline(always)]
        pub const fn PERB(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_PERB(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn PETB(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_PETB(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for ENDPTPRIME {
        #[inline(always)]
        fn default() -> ENDPTPRIME {
            ENDPTPRIME(0)
        }
    }
    impl core::fmt::Debug for ENDPTPRIME {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ENDPTPRIME")
                .field("PERB", &self.PERB())
                .field("PETB", &self.PETB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ENDPTPRIME {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ENDPTPRIME {{ PERB: {=u8:?}, PETB: {=u8:?} }}",
                self.PERB(),
                self.PETB()
            )
        }
    }
    #[doc = "Endpoint Setup Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ENDPTSETUPSTAT(pub u32);
    impl ENDPTSETUPSTAT {
        #[inline(always)]
        pub const fn ENDPTSETUPSTAT(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_ENDPTSETUPSTAT(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for ENDPTSETUPSTAT {
        #[inline(always)]
        fn default() -> ENDPTSETUPSTAT {
            ENDPTSETUPSTAT(0)
        }
    }
    impl core::fmt::Debug for ENDPTSETUPSTAT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ENDPTSETUPSTAT")
                .field("ENDPTSETUPSTAT", &self.ENDPTSETUPSTAT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ENDPTSETUPSTAT {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ENDPTSETUPSTAT {{ ENDPTSETUPSTAT: {=u16:?} }}",
                self.ENDPTSETUPSTAT()
            )
        }
    }
    #[doc = "Endpoint Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ENDPTSTAT(pub u32);
    impl ENDPTSTAT {
        #[inline(always)]
        pub const fn ERBR(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_ERBR(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn ETBR(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_ETBR(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for ENDPTSTAT {
        #[inline(always)]
        fn default() -> ENDPTSTAT {
            ENDPTSTAT(0)
        }
    }
    impl core::fmt::Debug for ENDPTSTAT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ENDPTSTAT")
                .field("ERBR", &self.ERBR())
                .field("ETBR", &self.ETBR())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ENDPTSTAT {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ENDPTSTAT {{ ERBR: {=u8:?}, ETBR: {=u8:?} }}",
                self.ERBR(),
                self.ETBR()
            )
        }
    }
    #[doc = "USB Frame Index"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FRINDEX(pub u32);
    impl FRINDEX {
        #[inline(always)]
        pub const fn FRINDEX(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x3fff;
            val as u16
        }
        #[inline(always)]
        pub fn set_FRINDEX(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
        }
    }
    impl Default for FRINDEX {
        #[inline(always)]
        fn default() -> FRINDEX {
            FRINDEX(0)
        }
    }
    impl core::fmt::Debug for FRINDEX {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FRINDEX")
                .field("FRINDEX", &self.FRINDEX())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FRINDEX {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "FRINDEX {{ FRINDEX: {=u16:?} }}", self.FRINDEX())
        }
    }
    #[doc = "General Purpose Timer #0 Controller"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GPTIMER0CTRL(pub u32);
    impl GPTIMER0CTRL {
        #[inline(always)]
        pub const fn GPTCNT(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_GPTCNT(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
        #[inline(always)]
        pub const fn GPTMODE(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GPTMODE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn GPTRST(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GPTRST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn GPTRUN(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GPTRUN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for GPTIMER0CTRL {
        #[inline(always)]
        fn default() -> GPTIMER0CTRL {
            GPTIMER0CTRL(0)
        }
    }
    impl core::fmt::Debug for GPTIMER0CTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GPTIMER0CTRL")
                .field("GPTCNT", &self.GPTCNT())
                .field("GPTMODE", &self.GPTMODE())
                .field("GPTRST", &self.GPTRST())
                .field("GPTRUN", &self.GPTRUN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GPTIMER0CTRL {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "GPTIMER0CTRL {{ GPTCNT: {=u32:?}, GPTMODE: {=bool:?}, GPTRST: {=bool:?}, GPTRUN: {=bool:?} }}" , self . GPTCNT () , self . GPTMODE () , self . GPTRST () , self . GPTRUN ())
        }
    }
    #[doc = "General Purpose Timer #0 Load"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GPTIMER0LD(pub u32);
    impl GPTIMER0LD {
        #[inline(always)]
        pub const fn GPTLD(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_GPTLD(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for GPTIMER0LD {
        #[inline(always)]
        fn default() -> GPTIMER0LD {
            GPTIMER0LD(0)
        }
    }
    impl core::fmt::Debug for GPTIMER0LD {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GPTIMER0LD")
                .field("GPTLD", &self.GPTLD())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GPTIMER0LD {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "GPTIMER0LD {{ GPTLD: {=u32:?} }}", self.GPTLD())
        }
    }
    #[doc = "General Purpose Timer #1 Controller"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GPTIMER1CTRL(pub u32);
    impl GPTIMER1CTRL {
        #[inline(always)]
        pub const fn GPTCNT(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_GPTCNT(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
        #[inline(always)]
        pub const fn GPTMODE(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GPTMODE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn GPTRST(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GPTRST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn GPTRUN(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GPTRUN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for GPTIMER1CTRL {
        #[inline(always)]
        fn default() -> GPTIMER1CTRL {
            GPTIMER1CTRL(0)
        }
    }
    impl core::fmt::Debug for GPTIMER1CTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GPTIMER1CTRL")
                .field("GPTCNT", &self.GPTCNT())
                .field("GPTMODE", &self.GPTMODE())
                .field("GPTRST", &self.GPTRST())
                .field("GPTRUN", &self.GPTRUN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GPTIMER1CTRL {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "GPTIMER1CTRL {{ GPTCNT: {=u32:?}, GPTMODE: {=bool:?}, GPTRST: {=bool:?}, GPTRUN: {=bool:?} }}" , self . GPTCNT () , self . GPTMODE () , self . GPTRST () , self . GPTRUN ())
        }
    }
    #[doc = "General Purpose Timer #1 Load"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GPTIMER1LD(pub u32);
    impl GPTIMER1LD {
        #[inline(always)]
        pub const fn GPTLD(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_GPTLD(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for GPTIMER1LD {
        #[inline(always)]
        fn default() -> GPTIMER1LD {
            GPTIMER1LD(0)
        }
    }
    impl core::fmt::Debug for GPTIMER1LD {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GPTIMER1LD")
                .field("GPTLD", &self.GPTLD())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GPTIMER1LD {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "GPTIMER1LD {{ GPTLD: {=u32:?} }}", self.GPTLD())
        }
    }
    #[doc = "Host Controller Capability Parameters"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HCCPARAMS(pub u32);
    impl HCCPARAMS {
        #[inline(always)]
        pub const fn ADC(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ADC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn PFL(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PFL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn ASP(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ASP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn IST(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_IST(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[inline(always)]
        pub const fn EECP(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_EECP(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for HCCPARAMS {
        #[inline(always)]
        fn default() -> HCCPARAMS {
            HCCPARAMS(0)
        }
    }
    impl core::fmt::Debug for HCCPARAMS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HCCPARAMS")
                .field("ADC", &self.ADC())
                .field("PFL", &self.PFL())
                .field("ASP", &self.ASP())
                .field("IST", &self.IST())
                .field("EECP", &self.EECP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HCCPARAMS {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "HCCPARAMS {{ ADC: {=bool:?}, PFL: {=bool:?}, ASP: {=bool:?}, IST: {=u8:?}, EECP: {=u8:?} }}" , self . ADC () , self . PFL () , self . ASP () , self . IST () , self . EECP ())
        }
    }
    #[doc = "Host Controller Structural Parameters"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HCSPARAMS(pub u32);
    impl HCSPARAMS {
        #[inline(always)]
        pub const fn N_PORTS(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_N_PORTS(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn PPC(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PPC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn N_PCC(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_N_PCC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[inline(always)]
        pub const fn N_CC(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_N_CC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[inline(always)]
        pub const fn PI(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PI(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn N_PTT(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_N_PTT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[inline(always)]
        pub const fn N_TT(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_N_TT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
    }
    impl Default for HCSPARAMS {
        #[inline(always)]
        fn default() -> HCSPARAMS {
            HCSPARAMS(0)
        }
    }
    impl core::fmt::Debug for HCSPARAMS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HCSPARAMS")
                .field("N_PORTS", &self.N_PORTS())
                .field("PPC", &self.PPC())
                .field("N_PCC", &self.N_PCC())
                .field("N_CC", &self.N_CC())
                .field("PI", &self.PI())
                .field("N_PTT", &self.N_PTT())
                .field("N_TT", &self.N_TT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HCSPARAMS {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "HCSPARAMS {{ N_PORTS: {=u8:?}, PPC: {=bool:?}, N_PCC: {=u8:?}, N_CC: {=u8:?}, PI: {=bool:?}, N_PTT: {=u8:?}, N_TT: {=u8:?} }}" , self . N_PORTS () , self . PPC () , self . N_PCC () , self . N_CC () , self . PI () , self . N_PTT () , self . N_TT ())
        }
    }
    #[doc = "Device Hardware Parameters"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HWDEVICE(pub u32);
    impl HWDEVICE {
        #[inline(always)]
        pub const fn DC(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn DEVEP(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_DEVEP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 1usize)) | (((val as u32) & 0x1f) << 1usize);
        }
    }
    impl Default for HWDEVICE {
        #[inline(always)]
        fn default() -> HWDEVICE {
            HWDEVICE(0)
        }
    }
    impl core::fmt::Debug for HWDEVICE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HWDEVICE")
                .field("DC", &self.DC())
                .field("DEVEP", &self.DEVEP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HWDEVICE {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "HWDEVICE {{ DC: {=bool:?}, DEVEP: {=u8:?} }}",
                self.DC(),
                self.DEVEP()
            )
        }
    }
    #[doc = "Hardware General"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HWGENERAL(pub u32);
    impl HWGENERAL {
        #[inline(always)]
        pub const fn PHYW(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_PHYW(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[inline(always)]
        pub const fn PHYM(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_PHYM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 6usize)) | (((val as u32) & 0x07) << 6usize);
        }
        #[inline(always)]
        pub const fn SM(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_SM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
        }
    }
    impl Default for HWGENERAL {
        #[inline(always)]
        fn default() -> HWGENERAL {
            HWGENERAL(0)
        }
    }
    impl core::fmt::Debug for HWGENERAL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HWGENERAL")
                .field("PHYW", &self.PHYW())
                .field("PHYM", &self.PHYM())
                .field("SM", &self.SM())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HWGENERAL {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "HWGENERAL {{ PHYW: {=u8:?}, PHYM: {=u8:?}, SM: {=u8:?} }}",
                self.PHYW(),
                self.PHYM(),
                self.SM()
            )
        }
    }
    #[doc = "Host Hardware Parameters"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HWHOST(pub u32);
    impl HWHOST {
        #[inline(always)]
        pub const fn HC(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn NPORT(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_NPORT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u32) & 0x07) << 1usize);
        }
    }
    impl Default for HWHOST {
        #[inline(always)]
        fn default() -> HWHOST {
            HWHOST(0)
        }
    }
    impl core::fmt::Debug for HWHOST {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HWHOST")
                .field("HC", &self.HC())
                .field("NPORT", &self.NPORT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HWHOST {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "HWHOST {{ HC: {=bool:?}, NPORT: {=u8:?} }}",
                self.HC(),
                self.NPORT()
            )
        }
    }
    #[doc = "RX Buffer Hardware Parameters"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HWRXBUF(pub u32);
    impl HWRXBUF {
        #[inline(always)]
        pub const fn RXBURST(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_RXBURST(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn RXADD(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_RXADD(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for HWRXBUF {
        #[inline(always)]
        fn default() -> HWRXBUF {
            HWRXBUF(0)
        }
    }
    impl core::fmt::Debug for HWRXBUF {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HWRXBUF")
                .field("RXBURST", &self.RXBURST())
                .field("RXADD", &self.RXADD())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HWRXBUF {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "HWRXBUF {{ RXBURST: {=u8:?}, RXADD: {=u8:?} }}",
                self.RXBURST(),
                self.RXADD()
            )
        }
    }
    #[doc = "TX Buffer Hardware Parameters"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HWTXBUF(pub u32);
    impl HWTXBUF {
        #[inline(always)]
        pub const fn TXBURST(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_TXBURST(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn TXCHANADD(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_TXCHANADD(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for HWTXBUF {
        #[inline(always)]
        fn default() -> HWTXBUF {
            HWTXBUF(0)
        }
    }
    impl core::fmt::Debug for HWTXBUF {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HWTXBUF")
                .field("TXBURST", &self.TXBURST())
                .field("TXCHANADD", &self.TXCHANADD())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HWTXBUF {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "HWTXBUF {{ TXBURST: {=u8:?}, TXCHANADD: {=u8:?} }}",
                self.TXBURST(),
                self.TXCHANADD()
            )
        }
    }
    #[doc = "Identification"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ID(pub u32);
    impl ID {
        #[inline(always)]
        pub const fn ID(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_ID(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[inline(always)]
        pub const fn NID(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_NID(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[inline(always)]
        pub const fn REVISION(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_REVISION(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for ID {
        #[inline(always)]
        fn default() -> ID {
            ID(0)
        }
    }
    impl core::fmt::Debug for ID {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ID")
                .field("ID", &self.ID())
                .field("NID", &self.NID())
                .field("REVISION", &self.REVISION())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ID {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ID {{ ID: {=u8:?}, NID: {=u8:?}, REVISION: {=u8:?} }}",
                self.ID(),
                self.NID(),
                self.REVISION()
            )
        }
    }
    #[doc = "On-The-Go Status & Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OTGSC(pub u32);
    impl OTGSC {
        #[inline(always)]
        pub const fn VD(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_VD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn VC(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_VC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn OT(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn DP(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn IDPU(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IDPU(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn ID(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ID(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn AVV(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_AVV(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn ASV(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ASV(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn BSV(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BSV(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn BSE(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BSE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn TOG_1MS(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TOG_1MS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn DPS(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DPS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn IDIS(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IDIS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn AVVIS(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_AVVIS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn ASVIS(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ASVIS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn BSVIS(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BSVIS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn BSEIS(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BSEIS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn STATUS_1MS(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_STATUS_1MS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn DPIS(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DPIS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn IDIE(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IDIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn AVVIE(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_AVVIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn ASVIE(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ASVIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn BSVIE(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BSVIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn BSEIE(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BSEIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn EN_1MS(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EN_1MS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn DPIE(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DPIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for OTGSC {
        #[inline(always)]
        fn default() -> OTGSC {
            OTGSC(0)
        }
    }
    impl core::fmt::Debug for OTGSC {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OTGSC")
                .field("VD", &self.VD())
                .field("VC", &self.VC())
                .field("OT", &self.OT())
                .field("DP", &self.DP())
                .field("IDPU", &self.IDPU())
                .field("ID", &self.ID())
                .field("AVV", &self.AVV())
                .field("ASV", &self.ASV())
                .field("BSV", &self.BSV())
                .field("BSE", &self.BSE())
                .field("TOG_1MS", &self.TOG_1MS())
                .field("DPS", &self.DPS())
                .field("IDIS", &self.IDIS())
                .field("AVVIS", &self.AVVIS())
                .field("ASVIS", &self.ASVIS())
                .field("BSVIS", &self.BSVIS())
                .field("BSEIS", &self.BSEIS())
                .field("STATUS_1MS", &self.STATUS_1MS())
                .field("DPIS", &self.DPIS())
                .field("IDIE", &self.IDIE())
                .field("AVVIE", &self.AVVIE())
                .field("ASVIE", &self.ASVIE())
                .field("BSVIE", &self.BSVIE())
                .field("BSEIE", &self.BSEIE())
                .field("EN_1MS", &self.EN_1MS())
                .field("DPIE", &self.DPIE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OTGSC {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "OTGSC {{ VD: {=bool:?}, VC: {=bool:?}, OT: {=bool:?}, DP: {=bool:?}, IDPU: {=bool:?}, ID: {=bool:?}, AVV: {=bool:?}, ASV: {=bool:?}, BSV: {=bool:?}, BSE: {=bool:?}, TOG_1MS: {=bool:?}, DPS: {=bool:?}, IDIS: {=bool:?}, AVVIS: {=bool:?}, ASVIS: {=bool:?}, BSVIS: {=bool:?}, BSEIS: {=bool:?}, STATUS_1MS: {=bool:?}, DPIS: {=bool:?}, IDIE: {=bool:?}, AVVIE: {=bool:?}, ASVIE: {=bool:?}, BSVIE: {=bool:?}, BSEIE: {=bool:?}, EN_1MS: {=bool:?}, DPIE: {=bool:?} }}" , self . VD () , self . VC () , self . OT () , self . DP () , self . IDPU () , self . ID () , self . AVV () , self . ASV () , self . BSV () , self . BSE () , self . TOG_1MS () , self . DPS () , self . IDIS () , self . AVVIS () , self . ASVIS () , self . BSVIS () , self . BSEIS () , self . STATUS_1MS () , self . DPIS () , self . IDIE () , self . AVVIE () , self . ASVIE () , self . BSVIE () , self . BSEIE () , self . EN_1MS () , self . DPIE ())
        }
    }
    #[doc = "Frame List Base Address"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PERIODICLISTBASE(pub u32);
    impl PERIODICLISTBASE {
        #[inline(always)]
        pub const fn BASEADR(&self) -> u32 {
            let val = (self.0 >> 12usize) & 0x000f_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_BASEADR(&mut self, val: u32) {
            self.0 =
                (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
        }
    }
    impl Default for PERIODICLISTBASE {
        #[inline(always)]
        fn default() -> PERIODICLISTBASE {
            PERIODICLISTBASE(0)
        }
    }
    impl core::fmt::Debug for PERIODICLISTBASE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PERIODICLISTBASE")
                .field("BASEADR", &self.BASEADR())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PERIODICLISTBASE {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PERIODICLISTBASE {{ BASEADR: {=u32:?} }}",
                self.BASEADR()
            )
        }
    }
    #[doc = "Port Status & Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PORTSC1(pub u32);
    impl PORTSC1 {
        #[inline(always)]
        pub const fn CCS(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CCS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn CSC(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CSC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn PE(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn PEC(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PEC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn OCA(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OCA(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn OCC(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OCC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn FPR(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FPR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn SUSP(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SUSP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn PR(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn HSP(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HSP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn LS(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_LS(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[inline(always)]
        pub const fn PP(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn PO(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PO(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn PIC(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_PIC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[inline(always)]
        pub const fn PTC(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_PTC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[inline(always)]
        pub const fn WKCN(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WKCN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn WKDC(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WKDC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn WKOC(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WKOC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn PHCD(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PHCD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn PFSC(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PFSC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn PTS_2(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PTS_2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn PSPD(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_PSPD(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[inline(always)]
        pub const fn PTW(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PTW(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn STS(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_STS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn PTS_1(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_PTS_1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for PORTSC1 {
        #[inline(always)]
        fn default() -> PORTSC1 {
            PORTSC1(0)
        }
    }
    impl core::fmt::Debug for PORTSC1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PORTSC1")
                .field("CCS", &self.CCS())
                .field("CSC", &self.CSC())
                .field("PE", &self.PE())
                .field("PEC", &self.PEC())
                .field("OCA", &self.OCA())
                .field("OCC", &self.OCC())
                .field("FPR", &self.FPR())
                .field("SUSP", &self.SUSP())
                .field("PR", &self.PR())
                .field("HSP", &self.HSP())
                .field("LS", &self.LS())
                .field("PP", &self.PP())
                .field("PO", &self.PO())
                .field("PIC", &self.PIC())
                .field("PTC", &self.PTC())
                .field("WKCN", &self.WKCN())
                .field("WKDC", &self.WKDC())
                .field("WKOC", &self.WKOC())
                .field("PHCD", &self.PHCD())
                .field("PFSC", &self.PFSC())
                .field("PTS_2", &self.PTS_2())
                .field("PSPD", &self.PSPD())
                .field("PTW", &self.PTW())
                .field("STS", &self.STS())
                .field("PTS_1", &self.PTS_1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PORTSC1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "PORTSC1 {{ CCS: {=bool:?}, CSC: {=bool:?}, PE: {=bool:?}, PEC: {=bool:?}, OCA: {=bool:?}, OCC: {=bool:?}, FPR: {=bool:?}, SUSP: {=bool:?}, PR: {=bool:?}, HSP: {=bool:?}, LS: {=u8:?}, PP: {=bool:?}, PO: {=bool:?}, PIC: {=u8:?}, PTC: {=u8:?}, WKCN: {=bool:?}, WKDC: {=bool:?}, WKOC: {=bool:?}, PHCD: {=bool:?}, PFSC: {=bool:?}, PTS_2: {=bool:?}, PSPD: {=u8:?}, PTW: {=bool:?}, STS: {=bool:?}, PTS_1: {=u8:?} }}" , self . CCS () , self . CSC () , self . PE () , self . PEC () , self . OCA () , self . OCC () , self . FPR () , self . SUSP () , self . PR () , self . HSP () , self . LS () , self . PP () , self . PO () , self . PIC () , self . PTC () , self . WKCN () , self . WKDC () , self . WKOC () , self . PHCD () , self . PFSC () , self . PTS_2 () , self . PSPD () , self . PTW () , self . STS () , self . PTS_1 ())
        }
    }
    #[doc = "System Bus Config"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SBUSCFG(pub u32);
    impl SBUSCFG {
        #[inline(always)]
        pub const fn AHBBRST(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_AHBBRST(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for SBUSCFG {
        #[inline(always)]
        fn default() -> SBUSCFG {
            SBUSCFG(0)
        }
    }
    impl core::fmt::Debug for SBUSCFG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SBUSCFG")
                .field("AHBBRST", &self.AHBBRST())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SBUSCFG {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SBUSCFG {{ AHBBRST: {=u8:?} }}", self.AHBBRST())
        }
    }
    #[doc = "TX FIFO Fill Tuning"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TXFILLTUNING(pub u32);
    impl TXFILLTUNING {
        #[inline(always)]
        pub const fn TXSCHOH(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub fn set_TXSCHOH(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[inline(always)]
        pub const fn TXSCHHEALTH(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_TXSCHHEALTH(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[inline(always)]
        pub const fn TXFIFOTHRES(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_TXFIFOTHRES(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
    }
    impl Default for TXFILLTUNING {
        #[inline(always)]
        fn default() -> TXFILLTUNING {
            TXFILLTUNING(0)
        }
    }
    impl core::fmt::Debug for TXFILLTUNING {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TXFILLTUNING")
                .field("TXSCHOH", &self.TXSCHOH())
                .field("TXSCHHEALTH", &self.TXSCHHEALTH())
                .field("TXFIFOTHRES", &self.TXFIFOTHRES())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TXFILLTUNING {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "TXFILLTUNING {{ TXSCHOH: {=u8:?}, TXSCHHEALTH: {=u8:?}, TXFIFOTHRES: {=u8:?} }}",
                self.TXSCHOH(),
                self.TXSCHHEALTH(),
                self.TXFIFOTHRES()
            )
        }
    }
    #[doc = "USB Command"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct USBCMD(pub u32);
    impl USBCMD {
        #[inline(always)]
        pub const fn RS(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn RST(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn FS_1(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_FS_1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[inline(always)]
        pub const fn PSE(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PSE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn ASE(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ASE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn IAA(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IAA(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn ASP(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_ASP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[inline(always)]
        pub const fn ASPE(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ASPE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn SUTW(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SUTW(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn ATDTW(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ATDTW(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn FS_2(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FS_2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn ITC(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_ITC(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for USBCMD {
        #[inline(always)]
        fn default() -> USBCMD {
            USBCMD(0)
        }
    }
    impl core::fmt::Debug for USBCMD {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("USBCMD")
                .field("RS", &self.RS())
                .field("RST", &self.RST())
                .field("FS_1", &self.FS_1())
                .field("PSE", &self.PSE())
                .field("ASE", &self.ASE())
                .field("IAA", &self.IAA())
                .field("ASP", &self.ASP())
                .field("ASPE", &self.ASPE())
                .field("SUTW", &self.SUTW())
                .field("ATDTW", &self.ATDTW())
                .field("FS_2", &self.FS_2())
                .field("ITC", &self.ITC())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for USBCMD {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "USBCMD {{ RS: {=bool:?}, RST: {=bool:?}, FS_1: {=u8:?}, PSE: {=bool:?}, ASE: {=bool:?}, IAA: {=bool:?}, ASP: {=u8:?}, ASPE: {=bool:?}, SUTW: {=bool:?}, ATDTW: {=bool:?}, FS_2: {=bool:?}, ITC: {=u8:?} }}" , self . RS () , self . RST () , self . FS_1 () , self . PSE () , self . ASE () , self . IAA () , self . ASP () , self . ASPE () , self . SUTW () , self . ATDTW () , self . FS_2 () , self . ITC ())
        }
    }
    #[doc = "Interrupt Enable"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct USBINTR(pub u32);
    impl USBINTR {
        #[inline(always)]
        pub const fn UE(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_UE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn UEE(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_UEE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn PCE(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PCE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn FRE(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FRE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn SEE(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SEE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn AAE(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_AAE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn URE(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_URE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn SRE(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SRE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn SLE(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SLE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn NAKE(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NAKE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn UAIE(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_UAIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn UPIE(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_UPIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn TIE0(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TIE0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn TIE1(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TIE1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for USBINTR {
        #[inline(always)]
        fn default() -> USBINTR {
            USBINTR(0)
        }
    }
    impl core::fmt::Debug for USBINTR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("USBINTR")
                .field("UE", &self.UE())
                .field("UEE", &self.UEE())
                .field("PCE", &self.PCE())
                .field("FRE", &self.FRE())
                .field("SEE", &self.SEE())
                .field("AAE", &self.AAE())
                .field("URE", &self.URE())
                .field("SRE", &self.SRE())
                .field("SLE", &self.SLE())
                .field("NAKE", &self.NAKE())
                .field("UAIE", &self.UAIE())
                .field("UPIE", &self.UPIE())
                .field("TIE0", &self.TIE0())
                .field("TIE1", &self.TIE1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for USBINTR {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "USBINTR {{ UE: {=bool:?}, UEE: {=bool:?}, PCE: {=bool:?}, FRE: {=bool:?}, SEE: {=bool:?}, AAE: {=bool:?}, URE: {=bool:?}, SRE: {=bool:?}, SLE: {=bool:?}, NAKE: {=bool:?}, UAIE: {=bool:?}, UPIE: {=bool:?}, TIE0: {=bool:?}, TIE1: {=bool:?} }}" , self . UE () , self . UEE () , self . PCE () , self . FRE () , self . SEE () , self . AAE () , self . URE () , self . SRE () , self . SLE () , self . NAKE () , self . UAIE () , self . UPIE () , self . TIE0 () , self . TIE1 ())
        }
    }
    #[doc = "USB Device Mode"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct USBMODE(pub u32);
    impl USBMODE {
        #[inline(always)]
        pub const fn CM(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_CM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[inline(always)]
        pub const fn ES(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ES(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn SLOM(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SLOM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn SDIS(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SDIS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for USBMODE {
        #[inline(always)]
        fn default() -> USBMODE {
            USBMODE(0)
        }
    }
    impl core::fmt::Debug for USBMODE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("USBMODE")
                .field("CM", &self.CM())
                .field("ES", &self.ES())
                .field("SLOM", &self.SLOM())
                .field("SDIS", &self.SDIS())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for USBMODE {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "USBMODE {{ CM: {=u8:?}, ES: {=bool:?}, SLOM: {=bool:?}, SDIS: {=bool:?} }}",
                self.CM(),
                self.ES(),
                self.SLOM(),
                self.SDIS()
            )
        }
    }
    #[doc = "USB Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct USBSTS(pub u32);
    impl USBSTS {
        #[inline(always)]
        pub const fn UI(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_UI(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn UEI(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_UEI(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn PCI(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PCI(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn FRI(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FRI(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn SEI(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SEI(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn AAI(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_AAI(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn URI(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_URI(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn SRI(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SRI(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn SLI(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SLI(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn ULPII(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ULPII(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn HCH(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HCH(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn RCL(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RCL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn PS(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn AS(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_AS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn NAKI(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NAKI(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn TI0(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TI0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn TI1(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TI1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for USBSTS {
        #[inline(always)]
        fn default() -> USBSTS {
            USBSTS(0)
        }
    }
    impl core::fmt::Debug for USBSTS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("USBSTS")
                .field("UI", &self.UI())
                .field("UEI", &self.UEI())
                .field("PCI", &self.PCI())
                .field("FRI", &self.FRI())
                .field("SEI", &self.SEI())
                .field("AAI", &self.AAI())
                .field("URI", &self.URI())
                .field("SRI", &self.SRI())
                .field("SLI", &self.SLI())
                .field("ULPII", &self.ULPII())
                .field("HCH", &self.HCH())
                .field("RCL", &self.RCL())
                .field("PS", &self.PS())
                .field("AS", &self.AS())
                .field("NAKI", &self.NAKI())
                .field("TI0", &self.TI0())
                .field("TI1", &self.TI1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for USBSTS {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "USBSTS {{ UI: {=bool:?}, UEI: {=bool:?}, PCI: {=bool:?}, FRI: {=bool:?}, SEI: {=bool:?}, AAI: {=bool:?}, URI: {=bool:?}, SRI: {=bool:?}, SLI: {=bool:?}, ULPII: {=bool:?}, HCH: {=bool:?}, RCL: {=bool:?}, PS: {=bool:?}, AS: {=bool:?}, NAKI: {=bool:?}, TI0: {=bool:?}, TI1: {=bool:?} }}" , self . UI () , self . UEI () , self . PCI () , self . FRI () , self . SEI () , self . AAI () , self . URI () , self . SRI () , self . SLI () , self . ULPII () , self . HCH () , self . RCL () , self . PS () , self . AS () , self . NAKI () , self . TI0 () , self . TI1 ())
        }
    }
}
