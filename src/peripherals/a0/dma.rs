#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (d5ec99b 2024-12-16))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CH {
    ptr: *mut u8,
}
unsafe impl Send for CH {}
unsafe impl Sync for CH {}
impl CH {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn CH_CSR(self) -> crate::common::Reg<regs::CH_CH_CSR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn CH_ES(self) -> crate::common::Reg<regs::CH_CH_ES, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn CH_INT(self) -> crate::common::Reg<regs::CH_CH_INT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn CH_SBR(self) -> crate::common::Reg<regs::CH_CH_SBR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn CH_PRI(self) -> crate::common::Reg<regs::CH_CH_PRI, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn CH_MUX(self) -> crate::common::Reg<regs::CH_CH_MUX, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn TCD_SADDR(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[inline(always)]
    pub const fn TCD_SOFF(self) -> crate::common::Reg<regs::CH_TCD_SOFF, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[inline(always)]
    pub const fn TCD_ATTR(self) -> crate::common::Reg<regs::CH_TCD_ATTR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x26usize) as _) }
    }
    #[inline(always)]
    pub const fn TCD_NBYTES_MLOFFNO(
        self,
    ) -> crate::common::Reg<regs::CH_TCD_NBYTES_MLOFFNO, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[inline(always)]
    pub const fn TCD_NBYTES_MLOFFYES(
        self,
    ) -> crate::common::Reg<regs::CH_TCD_NBYTES_MLOFFYES, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[inline(always)]
    pub const fn TCD_SLAST_SDA(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[inline(always)]
    pub const fn TCD_DADDR(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[inline(always)]
    pub const fn TCD_DOFF(self) -> crate::common::Reg<regs::CH_TCD_DOFF, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[inline(always)]
    pub const fn TCD_CITER_ELINKNO(
        self,
    ) -> crate::common::Reg<regs::CH_TCD_CITER_ELINKNO, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x36usize) as _) }
    }
    #[inline(always)]
    pub const fn TCD_CITER_ELINKYES(
        self,
    ) -> crate::common::Reg<regs::CH_TCD_CITER_ELINKYES, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x36usize) as _) }
    }
    #[inline(always)]
    pub const fn TCD_DLAST_SGA(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[inline(always)]
    pub const fn TCD_CSR(self) -> crate::common::Reg<regs::CH_TCD_CSR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[inline(always)]
    pub const fn TCD_BITER_ELINKNO(
        self,
    ) -> crate::common::Reg<regs::CH_TCD_BITER_ELINKNO, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3eusize) as _) }
    }
    #[inline(always)]
    pub const fn TCD_BITER_ELINKYES(
        self,
    ) -> crate::common::Reg<regs::CH_TCD_BITER_ELINKYES, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3eusize) as _) }
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA {
    ptr: *mut u8,
}
unsafe impl Send for DMA {}
unsafe impl Sync for DMA {}
impl DMA {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn MP_CSR(self) -> crate::common::Reg<regs::MP_CSR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn MP_ES(self) -> crate::common::Reg<regs::MP_ES, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn MP_INT(self) -> crate::common::Reg<regs::MP_INT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn MP_HRS(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn CH_GRPRI(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn CH(self, n: usize) -> CH {
        assert!(n < 4usize);
        unsafe { CH::from_ptr(self.ptr.add(0x1000usize + n * 4096usize) as _) }
    }
}
pub mod regs {
    #[doc = "Channel Control and Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CH_CH_CSR(pub u32);
    impl CH_CH_CSR {
        #[inline(always)]
        pub const fn ERQ(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ERQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn EARQ(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EARQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn EEI(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EEI(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn EBW(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EBW(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn DONE(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DONE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn ACTIVE(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ACTIVE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for CH_CH_CSR {
        #[inline(always)]
        fn default() -> CH_CH_CSR {
            CH_CH_CSR(0)
        }
    }
    impl core::fmt::Debug for CH_CH_CSR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CH_CH_CSR")
                .field("ERQ", &self.ERQ())
                .field("EARQ", &self.EARQ())
                .field("EEI", &self.EEI())
                .field("EBW", &self.EBW())
                .field("DONE", &self.DONE())
                .field("ACTIVE", &self.ACTIVE())
                .finish()
        }
    }
    #[doc = "Channel Error Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CH_CH_ES(pub u32);
    impl CH_CH_ES {
        #[inline(always)]
        pub const fn DBE(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DBE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn SBE(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SBE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn SGE(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SGE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn NCE(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NCE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn DOE(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DOE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn DAE(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DAE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn SOE(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SOE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn SAE(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SAE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn ERR(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ERR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for CH_CH_ES {
        #[inline(always)]
        fn default() -> CH_CH_ES {
            CH_CH_ES(0)
        }
    }
    impl core::fmt::Debug for CH_CH_ES {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CH_CH_ES")
                .field("DBE", &self.DBE())
                .field("SBE", &self.SBE())
                .field("SGE", &self.SGE())
                .field("NCE", &self.NCE())
                .field("DOE", &self.DOE())
                .field("DAE", &self.DAE())
                .field("SOE", &self.SOE())
                .field("SAE", &self.SAE())
                .field("ERR", &self.ERR())
                .finish()
        }
    }
    #[doc = "Channel Interrupt Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CH_CH_INT(pub u32);
    impl CH_CH_INT {
        #[inline(always)]
        pub const fn INT(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for CH_CH_INT {
        #[inline(always)]
        fn default() -> CH_CH_INT {
            CH_CH_INT(0)
        }
    }
    impl core::fmt::Debug for CH_CH_INT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CH_CH_INT")
                .field("INT", &self.INT())
                .finish()
        }
    }
    #[doc = "Channel Multiplexor Configuration"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CH_CH_MUX(pub u32);
    impl CH_CH_MUX {
        #[inline(always)]
        pub const fn SRC(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub fn set_SRC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
    }
    impl Default for CH_CH_MUX {
        #[inline(always)]
        fn default() -> CH_CH_MUX {
            CH_CH_MUX(0)
        }
    }
    impl core::fmt::Debug for CH_CH_MUX {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CH_CH_MUX")
                .field("SRC", &self.SRC())
                .finish()
        }
    }
    #[doc = "Channel Priority"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CH_CH_PRI(pub u32);
    impl CH_CH_PRI {
        #[inline(always)]
        pub const fn APL(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_APL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[inline(always)]
        pub const fn DPA(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DPA(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn ECP(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ECP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for CH_CH_PRI {
        #[inline(always)]
        fn default() -> CH_CH_PRI {
            CH_CH_PRI(0)
        }
    }
    impl core::fmt::Debug for CH_CH_PRI {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CH_CH_PRI")
                .field("APL", &self.APL())
                .field("DPA", &self.DPA())
                .field("ECP", &self.ECP())
                .finish()
        }
    }
    #[doc = "Channel System Bus"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CH_CH_SBR(pub u32);
    impl CH_CH_SBR {
        #[inline(always)]
        pub const fn MID(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_MID(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn PAL(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PAL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn EMI(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EMI(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for CH_CH_SBR {
        #[inline(always)]
        fn default() -> CH_CH_SBR {
            CH_CH_SBR(0)
        }
    }
    impl core::fmt::Debug for CH_CH_SBR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CH_CH_SBR")
                .field("MID", &self.MID())
                .field("PAL", &self.PAL())
                .field("EMI", &self.EMI())
                .finish()
        }
    }
    #[doc = "Channel Arbitration Group"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CH_GRPRI(pub u32);
    impl CH_GRPRI {
        #[inline(always)]
        pub const fn GRPRI(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_GRPRI(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
    }
    impl Default for CH_GRPRI {
        #[inline(always)]
        fn default() -> CH_GRPRI {
            CH_GRPRI(0)
        }
    }
    impl core::fmt::Debug for CH_GRPRI {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CH_GRPRI")
                .field("GRPRI", &self.GRPRI())
                .finish()
        }
    }
    #[doc = "TCD Transfer Attributes"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CH_TCD_ATTR(pub u32);
    impl CH_TCD_ATTR {
        #[inline(always)]
        pub const fn DSIZE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_DSIZE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[inline(always)]
        pub const fn DMOD(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_DMOD(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u32) & 0x1f) << 3usize);
        }
        #[inline(always)]
        pub const fn SSIZE(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_SSIZE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[inline(always)]
        pub const fn SMOD(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_SMOD(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u32) & 0x1f) << 11usize);
        }
    }
    impl Default for CH_TCD_ATTR {
        #[inline(always)]
        fn default() -> CH_TCD_ATTR {
            CH_TCD_ATTR(0)
        }
    }
    impl core::fmt::Debug for CH_TCD_ATTR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CH_TCD_ATTR")
                .field("DSIZE", &self.DSIZE())
                .field("DMOD", &self.DMOD())
                .field("SSIZE", &self.SSIZE())
                .field("SMOD", &self.SMOD())
                .finish()
        }
    }
    #[doc = "TCD Beginning Major Loop Count (Minor Loop Channel Linking Disabled)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CH_TCD_BITER_ELINKNO(pub u32);
    impl CH_TCD_BITER_ELINKNO {
        #[inline(always)]
        pub const fn BITER(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x7fff;
            val as u16
        }
        #[inline(always)]
        pub fn set_BITER(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
        }
        #[inline(always)]
        pub const fn ELINK(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ELINK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for CH_TCD_BITER_ELINKNO {
        #[inline(always)]
        fn default() -> CH_TCD_BITER_ELINKNO {
            CH_TCD_BITER_ELINKNO(0)
        }
    }
    impl core::fmt::Debug for CH_TCD_BITER_ELINKNO {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CH_TCD_BITER_ELINKNO")
                .field("BITER", &self.BITER())
                .field("ELINK", &self.ELINK())
                .finish()
        }
    }
    #[doc = "TCD Beginning Major Loop Count (Minor Loop Channel Linking Enabled)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CH_TCD_BITER_ELINKYES(pub u32);
    impl CH_TCD_BITER_ELINKYES {
        #[inline(always)]
        pub const fn BITER(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[inline(always)]
        pub fn set_BITER(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[inline(always)]
        pub const fn LINKCH(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_LINKCH(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
        }
        #[inline(always)]
        pub const fn ELINK(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ELINK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for CH_TCD_BITER_ELINKYES {
        #[inline(always)]
        fn default() -> CH_TCD_BITER_ELINKYES {
            CH_TCD_BITER_ELINKYES(0)
        }
    }
    impl core::fmt::Debug for CH_TCD_BITER_ELINKYES {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CH_TCD_BITER_ELINKYES")
                .field("BITER", &self.BITER())
                .field("LINKCH", &self.LINKCH())
                .field("ELINK", &self.ELINK())
                .finish()
        }
    }
    #[doc = "TCD Current Major Loop Count (Minor Loop Channel Linking Disabled)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CH_TCD_CITER_ELINKNO(pub u32);
    impl CH_TCD_CITER_ELINKNO {
        #[inline(always)]
        pub const fn CITER(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x7fff;
            val as u16
        }
        #[inline(always)]
        pub fn set_CITER(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
        }
        #[inline(always)]
        pub const fn ELINK(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ELINK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for CH_TCD_CITER_ELINKNO {
        #[inline(always)]
        fn default() -> CH_TCD_CITER_ELINKNO {
            CH_TCD_CITER_ELINKNO(0)
        }
    }
    impl core::fmt::Debug for CH_TCD_CITER_ELINKNO {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CH_TCD_CITER_ELINKNO")
                .field("CITER", &self.CITER())
                .field("ELINK", &self.ELINK())
                .finish()
        }
    }
    #[doc = "TCD Current Major Loop Count (Minor Loop Channel Linking Enabled)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CH_TCD_CITER_ELINKYES(pub u32);
    impl CH_TCD_CITER_ELINKYES {
        #[inline(always)]
        pub const fn CITER(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[inline(always)]
        pub fn set_CITER(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[inline(always)]
        pub const fn LINKCH(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_LINKCH(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
        }
        #[inline(always)]
        pub const fn ELINK(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ELINK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for CH_TCD_CITER_ELINKYES {
        #[inline(always)]
        fn default() -> CH_TCD_CITER_ELINKYES {
            CH_TCD_CITER_ELINKYES(0)
        }
    }
    impl core::fmt::Debug for CH_TCD_CITER_ELINKYES {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CH_TCD_CITER_ELINKYES")
                .field("CITER", &self.CITER())
                .field("LINKCH", &self.LINKCH())
                .field("ELINK", &self.ELINK())
                .finish()
        }
    }
    #[doc = "TCD Control and Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CH_TCD_CSR(pub u32);
    impl CH_TCD_CSR {
        #[inline(always)]
        pub const fn START(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_START(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn INTMAJOR(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INTMAJOR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn INTHALF(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INTHALF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn DREQ(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DREQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn ESG(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ESG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn MAJORELINK(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_MAJORELINK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn EEOP(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EEOP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn ESDA(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ESDA(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn MAJORLINKCH(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_MAJORLINKCH(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[inline(always)]
        pub const fn BWC(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_BWC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
    }
    impl Default for CH_TCD_CSR {
        #[inline(always)]
        fn default() -> CH_TCD_CSR {
            CH_TCD_CSR(0)
        }
    }
    impl core::fmt::Debug for CH_TCD_CSR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CH_TCD_CSR")
                .field("START", &self.START())
                .field("INTMAJOR", &self.INTMAJOR())
                .field("INTHALF", &self.INTHALF())
                .field("DREQ", &self.DREQ())
                .field("ESG", &self.ESG())
                .field("MAJORELINK", &self.MAJORELINK())
                .field("EEOP", &self.EEOP())
                .field("ESDA", &self.ESDA())
                .field("MAJORLINKCH", &self.MAJORLINKCH())
                .field("BWC", &self.BWC())
                .finish()
        }
    }
    #[doc = "TCD Signed Destination Address Offset"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CH_TCD_DOFF(pub u32);
    impl CH_TCD_DOFF {
        #[inline(always)]
        pub const fn DOFF(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_DOFF(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for CH_TCD_DOFF {
        #[inline(always)]
        fn default() -> CH_TCD_DOFF {
            CH_TCD_DOFF(0)
        }
    }
    impl core::fmt::Debug for CH_TCD_DOFF {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CH_TCD_DOFF")
                .field("DOFF", &self.DOFF())
                .finish()
        }
    }
    #[doc = "TCD Transfer Size Without Minor Loop Offsets"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CH_TCD_NBYTES_MLOFFNO(pub u32);
    impl CH_TCD_NBYTES_MLOFFNO {
        #[inline(always)]
        pub const fn NBYTES(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x3fff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_NBYTES(&mut self, val: u32) {
            self.0 = (self.0 & !(0x3fff_ffff << 0usize)) | (((val as u32) & 0x3fff_ffff) << 0usize);
        }
        #[inline(always)]
        pub const fn DMLOE(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DMLOE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn SMLOE(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SMLOE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for CH_TCD_NBYTES_MLOFFNO {
        #[inline(always)]
        fn default() -> CH_TCD_NBYTES_MLOFFNO {
            CH_TCD_NBYTES_MLOFFNO(0)
        }
    }
    impl core::fmt::Debug for CH_TCD_NBYTES_MLOFFNO {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CH_TCD_NBYTES_MLOFFNO")
                .field("NBYTES", &self.NBYTES())
                .field("DMLOE", &self.DMLOE())
                .field("SMLOE", &self.SMLOE())
                .finish()
        }
    }
    #[doc = "TCD Transfer Size with Minor Loop Offsets"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CH_TCD_NBYTES_MLOFFYES(pub u32);
    impl CH_TCD_NBYTES_MLOFFYES {
        #[inline(always)]
        pub const fn NBYTES(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[inline(always)]
        pub fn set_NBYTES(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[inline(always)]
        pub const fn MLOFF(&self) -> u32 {
            let val = (self.0 >> 10usize) & 0x000f_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_MLOFF(&mut self, val: u32) {
            self.0 =
                (self.0 & !(0x000f_ffff << 10usize)) | (((val as u32) & 0x000f_ffff) << 10usize);
        }
        #[inline(always)]
        pub const fn DMLOE(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DMLOE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn SMLOE(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SMLOE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for CH_TCD_NBYTES_MLOFFYES {
        #[inline(always)]
        fn default() -> CH_TCD_NBYTES_MLOFFYES {
            CH_TCD_NBYTES_MLOFFYES(0)
        }
    }
    impl core::fmt::Debug for CH_TCD_NBYTES_MLOFFYES {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CH_TCD_NBYTES_MLOFFYES")
                .field("NBYTES", &self.NBYTES())
                .field("MLOFF", &self.MLOFF())
                .field("DMLOE", &self.DMLOE())
                .field("SMLOE", &self.SMLOE())
                .finish()
        }
    }
    #[doc = "TCD Signed Source Address Offset"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CH_TCD_SOFF(pub u32);
    impl CH_TCD_SOFF {
        #[inline(always)]
        pub const fn SOFF(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_SOFF(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for CH_TCD_SOFF {
        #[inline(always)]
        fn default() -> CH_TCD_SOFF {
            CH_TCD_SOFF(0)
        }
    }
    impl core::fmt::Debug for CH_TCD_SOFF {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CH_TCD_SOFF")
                .field("SOFF", &self.SOFF())
                .finish()
        }
    }
    #[doc = "Management Page Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MP_CSR(pub u32);
    impl MP_CSR {
        #[inline(always)]
        pub const fn EDBG(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EDBG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn ERCA(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ERCA(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn HAE(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HAE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn HALT(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HALT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn GCLC(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GCLC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn GMRC(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GMRC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn ECX(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ECX(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn CX(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CX(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn ACTIVE_ID(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_ACTIVE_ID(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[inline(always)]
        pub const fn ACTIVE(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ACTIVE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MP_CSR {
        #[inline(always)]
        fn default() -> MP_CSR {
            MP_CSR(0)
        }
    }
    impl core::fmt::Debug for MP_CSR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MP_CSR")
                .field("EDBG", &self.EDBG())
                .field("ERCA", &self.ERCA())
                .field("HAE", &self.HAE())
                .field("HALT", &self.HALT())
                .field("GCLC", &self.GCLC())
                .field("GMRC", &self.GMRC())
                .field("ECX", &self.ECX())
                .field("CX", &self.CX())
                .field("ACTIVE_ID", &self.ACTIVE_ID())
                .field("ACTIVE", &self.ACTIVE())
                .finish()
        }
    }
    #[doc = "Management Page Error Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MP_ES(pub u32);
    impl MP_ES {
        #[inline(always)]
        pub const fn DBE(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DBE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn SBE(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SBE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn SGE(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SGE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn NCE(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NCE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn DOE(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DOE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn DAE(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DAE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn SOE(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SOE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn SAE(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SAE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn ECX(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ECX(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn ERRCHN(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_ERRCHN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[inline(always)]
        pub const fn VLD(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_VLD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MP_ES {
        #[inline(always)]
        fn default() -> MP_ES {
            MP_ES(0)
        }
    }
    impl core::fmt::Debug for MP_ES {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MP_ES")
                .field("DBE", &self.DBE())
                .field("SBE", &self.SBE())
                .field("SGE", &self.SGE())
                .field("NCE", &self.NCE())
                .field("DOE", &self.DOE())
                .field("DAE", &self.DAE())
                .field("SOE", &self.SOE())
                .field("SAE", &self.SAE())
                .field("ECX", &self.ECX())
                .field("ERRCHN", &self.ERRCHN())
                .field("VLD", &self.VLD())
                .finish()
        }
    }
    #[doc = "Management Page Interrupt Request Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MP_INT(pub u32);
    impl MP_INT {
        #[inline(always)]
        pub const fn INT(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_INT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for MP_INT {
        #[inline(always)]
        fn default() -> MP_INT {
            MP_INT(0)
        }
    }
    impl core::fmt::Debug for MP_INT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MP_INT").field("INT", &self.INT()).finish()
        }
    }
}
