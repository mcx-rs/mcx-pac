#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (6651cd0 2025-05-06))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C {
    ptr: *mut u8,
}
unsafe impl Send for I3C {}
unsafe impl Sync for I3C {}
impl I3C {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn MCONFIG(self) -> crate::common::Reg<regs::MCONFIG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn SCONFIG(self) -> crate::common::Reg<regs::SCONFIG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn SSTATUS(self) -> crate::common::Reg<regs::SSTATUS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn SCTRL(self) -> crate::common::Reg<regs::SCTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn SINTSET(self) -> crate::common::Reg<regs::SINTSET, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn SINTCLR(self) -> crate::common::Reg<regs::SINTCLR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn SINTMASKED(self) -> crate::common::Reg<regs::SINTMASKED, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn SERRWARN(self) -> crate::common::Reg<regs::SERRWARN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[inline(always)]
    pub const fn SDMACTRL(self) -> crate::common::Reg<regs::SDMACTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[inline(always)]
    pub const fn SDATACTRL(self) -> crate::common::Reg<regs::SDATACTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[inline(always)]
    pub const fn SWDATAB(self) -> crate::common::Reg<regs::SWDATAB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[inline(always)]
    pub const fn SWDATABE(self) -> crate::common::Reg<regs::SWDATABE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[inline(always)]
    pub const fn SWDATAH(self) -> crate::common::Reg<regs::SWDATAH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[inline(always)]
    pub const fn SWDATAHE(self) -> crate::common::Reg<regs::SWDATAHE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[inline(always)]
    pub const fn SRDATAB(self) -> crate::common::Reg<regs::SRDATAB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[inline(always)]
    pub const fn SRDATAH(self) -> crate::common::Reg<regs::SRDATAH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[inline(always)]
    pub const fn SWDATAB1(self) -> crate::common::Reg<regs::SWDATAB1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[inline(always)]
    pub const fn SWDATAH1(self) -> crate::common::Reg<regs::SWDATAH1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[inline(always)]
    pub const fn SCAPABILITIES2(
        self,
    ) -> crate::common::Reg<regs::SCAPABILITIES2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[inline(always)]
    pub const fn SCAPABILITIES(self) -> crate::common::Reg<regs::SCAPABILITIES, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[inline(always)]
    pub const fn SDYNADDR(self) -> crate::common::Reg<regs::SDYNADDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[inline(always)]
    pub const fn SMAXLIMITS(self) -> crate::common::Reg<regs::SMAXLIMITS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
    #[inline(always)]
    pub const fn SIDPARTNO(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x6cusize) as _) }
    }
    #[inline(always)]
    pub const fn SIDEXT(self) -> crate::common::Reg<regs::SIDEXT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
    }
    #[inline(always)]
    pub const fn SVENDORID(self) -> crate::common::Reg<regs::SVENDORID, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x74usize) as _) }
    }
    #[inline(always)]
    pub const fn STCCLOCK(self) -> crate::common::Reg<regs::STCCLOCK, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x78usize) as _) }
    }
    #[inline(always)]
    pub const fn SMSGMAPADDR(self) -> crate::common::Reg<regs::SMSGMAPADDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x7cusize) as _) }
    }
    #[inline(always)]
    pub const fn MCONFIG_EXT(self) -> crate::common::Reg<regs::MCONFIG_EXT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[inline(always)]
    pub const fn MCTRL(self) -> crate::common::Reg<regs::MCTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[inline(always)]
    pub const fn MSTATUS(self) -> crate::common::Reg<regs::MSTATUS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[inline(always)]
    pub const fn MIBIRULES(self) -> crate::common::Reg<regs::MIBIRULES, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8cusize) as _) }
    }
    #[inline(always)]
    pub const fn MINTSET(self) -> crate::common::Reg<regs::MINTSET, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize) as _) }
    }
    #[inline(always)]
    pub const fn MINTCLR(self) -> crate::common::Reg<regs::MINTCLR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x94usize) as _) }
    }
    #[inline(always)]
    pub const fn MINTMASKED(self) -> crate::common::Reg<regs::MINTMASKED, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x98usize) as _) }
    }
    #[inline(always)]
    pub const fn MERRWARN(self) -> crate::common::Reg<regs::MERRWARN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x9cusize) as _) }
    }
    #[inline(always)]
    pub const fn MDMACTRL(self) -> crate::common::Reg<regs::MDMACTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize) as _) }
    }
    #[inline(always)]
    pub const fn MDATACTRL(self) -> crate::common::Reg<regs::MDATACTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xacusize) as _) }
    }
    #[inline(always)]
    pub const fn MWDATAB(self) -> crate::common::Reg<regs::MWDATAB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb0usize) as _) }
    }
    #[inline(always)]
    pub const fn MWDATABE(self) -> crate::common::Reg<regs::MWDATABE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb4usize) as _) }
    }
    #[inline(always)]
    pub const fn MWDATAH(self) -> crate::common::Reg<regs::MWDATAH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb8usize) as _) }
    }
    #[inline(always)]
    pub const fn MWDATAHE(self) -> crate::common::Reg<regs::MWDATAHE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xbcusize) as _) }
    }
    #[inline(always)]
    pub const fn MRDATAB(self) -> crate::common::Reg<regs::MRDATAB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc0usize) as _) }
    }
    #[inline(always)]
    pub const fn MRDATAH(self) -> crate::common::Reg<regs::MRDATAH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc8usize) as _) }
    }
    #[inline(always)]
    pub const fn MWDATAB1(self) -> crate::common::Reg<regs::MWDATAB1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xccusize) as _) }
    }
    #[inline(always)]
    pub const fn MWDATAH1(self) -> crate::common::Reg<regs::MWDATAH1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xccusize) as _) }
    }
    #[inline(always)]
    pub const fn MWMSG_SDR_CONTROL(
        self,
    ) -> crate::common::Reg<regs::MWMSG_SDR_CONTROL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd0usize) as _) }
    }
    #[inline(always)]
    pub const fn MWMSG_SDR_DATA(
        self,
    ) -> crate::common::Reg<regs::MWMSG_SDR_DATA, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd0usize) as _) }
    }
    #[inline(always)]
    pub const fn MRMSG_SDR(self) -> crate::common::Reg<regs::MRMSG_SDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd4usize) as _) }
    }
    #[inline(always)]
    pub const fn MWMSG_DDR_CONTROL(
        self,
    ) -> crate::common::Reg<regs::MWMSG_DDR_CONTROL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd8usize) as _) }
    }
    #[inline(always)]
    pub const fn MWMSG_DDR_CONTROL2(
        self,
    ) -> crate::common::Reg<regs::MWMSG_DDR_CONTROL2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd8usize) as _) }
    }
    #[inline(always)]
    pub const fn MWMSG_DDR_DATA(
        self,
    ) -> crate::common::Reg<regs::MWMSG_DDR_DATA, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd8usize) as _) }
    }
    #[inline(always)]
    pub const fn MRMSG_DDR(self) -> crate::common::Reg<regs::MRMSG_DDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xdcusize) as _) }
    }
    #[inline(always)]
    pub const fn MDYNADDR(self) -> crate::common::Reg<regs::MDYNADDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe4usize) as _) }
    }
    #[inline(always)]
    pub const fn SMAPCTRL0(self) -> crate::common::Reg<regs::SMAPCTRL0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x011cusize) as _) }
    }
    #[inline(always)]
    pub const fn IBIEXT1(self) -> crate::common::Reg<regs::IBIEXT1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0140usize) as _) }
    }
    #[inline(always)]
    pub const fn IBIEXT2(self) -> crate::common::Reg<regs::IBIEXT2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0144usize) as _) }
    }
    #[inline(always)]
    pub const fn SID(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ffcusize) as _) }
    }
}
pub mod regs {
    #[doc = "Extended IBI Data 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IBIEXT1(pub u32);
    impl IBIEXT1 {
        #[must_use]
        #[inline(always)]
        pub const fn CNT(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CNT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MAX(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_MAX(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn EXT1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_EXT1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn EXT2(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_EXT2(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn EXT3(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_EXT3(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for IBIEXT1 {
        #[inline(always)]
        fn default() -> IBIEXT1 {
            IBIEXT1(0)
        }
    }
    impl core::fmt::Debug for IBIEXT1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IBIEXT1")
                .field("CNT", &self.CNT())
                .field("MAX", &self.MAX())
                .field("EXT1", &self.EXT1())
                .field("EXT2", &self.EXT2())
                .field("EXT3", &self.EXT3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IBIEXT1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "IBIEXT1 {{ CNT: {=u8:?}, MAX: {=u8:?}, EXT1: {=u8:?}, EXT2: {=u8:?}, EXT3: {=u8:?} }}" , self . CNT () , self . MAX () , self . EXT1 () , self . EXT2 () , self . EXT3 ())
        }
    }
    #[doc = "Extended IBI Data 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IBIEXT2(pub u32);
    impl IBIEXT2 {
        #[must_use]
        #[inline(always)]
        pub const fn EXT4(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_EXT4(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn EXT5(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_EXT5(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn EXT6(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_EXT6(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn EXT7(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_EXT7(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for IBIEXT2 {
        #[inline(always)]
        fn default() -> IBIEXT2 {
            IBIEXT2(0)
        }
    }
    impl core::fmt::Debug for IBIEXT2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IBIEXT2")
                .field("EXT4", &self.EXT4())
                .field("EXT5", &self.EXT5())
                .field("EXT6", &self.EXT6())
                .field("EXT7", &self.EXT7())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IBIEXT2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "IBIEXT2 {{ EXT4: {=u8:?}, EXT5: {=u8:?}, EXT6: {=u8:?}, EXT7: {=u8:?} }}",
                self.EXT4(),
                self.EXT5(),
                self.EXT6(),
                self.EXT7()
            )
        }
    }
    #[doc = "Controller Configuration"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MCONFIG(pub u32);
    impl MCONFIG {
        #[must_use]
        #[inline(always)]
        pub const fn MSTENA(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_MSTENA(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DISTO(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DISTO(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn HKEEP(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_HKEEP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ODSTOP(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ODSTOP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PPBAUD(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_PPBAUD(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PPLOW(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_PPLOW(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ODBAUD(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ODBAUD(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ODHPP(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ODHPP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SKEW(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SKEW(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 25usize)) | (((val as u32) & 0x07) << 25usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn I2CBAUD(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_I2CBAUD(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for MCONFIG {
        #[inline(always)]
        fn default() -> MCONFIG {
            MCONFIG(0)
        }
    }
    impl core::fmt::Debug for MCONFIG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MCONFIG")
                .field("MSTENA", &self.MSTENA())
                .field("DISTO", &self.DISTO())
                .field("HKEEP", &self.HKEEP())
                .field("ODSTOP", &self.ODSTOP())
                .field("PPBAUD", &self.PPBAUD())
                .field("PPLOW", &self.PPLOW())
                .field("ODBAUD", &self.ODBAUD())
                .field("ODHPP", &self.ODHPP())
                .field("SKEW", &self.SKEW())
                .field("I2CBAUD", &self.I2CBAUD())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MCONFIG {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MCONFIG {{ MSTENA: {=u8:?}, DISTO: {=bool:?}, HKEEP: {=u8:?}, ODSTOP: {=bool:?}, PPBAUD: {=u8:?}, PPLOW: {=u8:?}, ODBAUD: {=u8:?}, ODHPP: {=bool:?}, SKEW: {=u8:?}, I2CBAUD: {=u8:?} }}" , self . MSTENA () , self . DISTO () , self . HKEEP () , self . ODSTOP () , self . PPBAUD () , self . PPLOW () , self . ODBAUD () , self . ODHPP () , self . SKEW () , self . I2CBAUD ())
        }
    }
    #[doc = "Controller Extended Configuration"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MCONFIG_EXT(pub u32);
    impl MCONFIG_EXT {
        #[must_use]
        #[inline(always)]
        pub const fn I3C_CAS_DEL(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_I3C_CAS_DEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn I3C_CASR_DEL(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_I3C_CASR_DEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
    }
    impl Default for MCONFIG_EXT {
        #[inline(always)]
        fn default() -> MCONFIG_EXT {
            MCONFIG_EXT(0)
        }
    }
    impl core::fmt::Debug for MCONFIG_EXT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MCONFIG_EXT")
                .field("I3C_CAS_DEL", &self.I3C_CAS_DEL())
                .field("I3C_CASR_DEL", &self.I3C_CASR_DEL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MCONFIG_EXT {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MCONFIG_EXT {{ I3C_CAS_DEL: {=u8:?}, I3C_CASR_DEL: {=u8:?} }}",
                self.I3C_CAS_DEL(),
                self.I3C_CASR_DEL()
            )
        }
    }
    #[doc = "Controller Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MCTRL(pub u32);
    impl MCTRL {
        #[must_use]
        #[inline(always)]
        pub const fn REQUEST(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_REQUEST(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TYPE(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TYPE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn IBIRESP(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_IBIRESP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DIR(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DIR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ADDR(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ADDR(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 9usize)) | (((val as u32) & 0x7f) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RDTERM(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RDTERM(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for MCTRL {
        #[inline(always)]
        fn default() -> MCTRL {
            MCTRL(0)
        }
    }
    impl core::fmt::Debug for MCTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MCTRL")
                .field("REQUEST", &self.REQUEST())
                .field("TYPE", &self.TYPE())
                .field("IBIRESP", &self.IBIRESP())
                .field("DIR", &self.DIR())
                .field("ADDR", &self.ADDR())
                .field("RDTERM", &self.RDTERM())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MCTRL {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MCTRL {{ REQUEST: {=u8:?}, TYPE: {=u8:?}, IBIRESP: {=u8:?}, DIR: {=bool:?}, ADDR: {=u8:?}, RDTERM: {=u8:?} }}" , self . REQUEST () , self . TYPE () , self . IBIRESP () , self . DIR () , self . ADDR () , self . RDTERM ())
        }
    }
    #[doc = "Controller Data Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MDATACTRL(pub u32);
    impl MDATACTRL {
        #[must_use]
        #[inline(always)]
        pub const fn FLUSHTB(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FLUSHTB(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FLUSHFB(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FLUSHFB(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn UNLOCK(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_UNLOCK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TXTRIG(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TXTRIG(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RXTRIG(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RXTRIG(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TXCOUNT(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TXCOUNT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RXCOUNT(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RXCOUNT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TXFULL(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TXFULL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RXEMPTY(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RXEMPTY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MDATACTRL {
        #[inline(always)]
        fn default() -> MDATACTRL {
            MDATACTRL(0)
        }
    }
    impl core::fmt::Debug for MDATACTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MDATACTRL")
                .field("FLUSHTB", &self.FLUSHTB())
                .field("FLUSHFB", &self.FLUSHFB())
                .field("UNLOCK", &self.UNLOCK())
                .field("TXTRIG", &self.TXTRIG())
                .field("RXTRIG", &self.RXTRIG())
                .field("TXCOUNT", &self.TXCOUNT())
                .field("RXCOUNT", &self.RXCOUNT())
                .field("TXFULL", &self.TXFULL())
                .field("RXEMPTY", &self.RXEMPTY())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MDATACTRL {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MDATACTRL {{ FLUSHTB: {=bool:?}, FLUSHFB: {=bool:?}, UNLOCK: {=bool:?}, TXTRIG: {=u8:?}, RXTRIG: {=u8:?}, TXCOUNT: {=u8:?}, RXCOUNT: {=u8:?}, TXFULL: {=bool:?}, RXEMPTY: {=bool:?} }}" , self . FLUSHTB () , self . FLUSHFB () , self . UNLOCK () , self . TXTRIG () , self . RXTRIG () , self . TXCOUNT () , self . RXCOUNT () , self . TXFULL () , self . RXEMPTY ())
        }
    }
    #[doc = "Controller DMA Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MDMACTRL(pub u32);
    impl MDMACTRL {
        #[must_use]
        #[inline(always)]
        pub const fn DMAFB(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_DMAFB(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DMATB(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_DMATB(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DMAWIDTH(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_DMAWIDTH(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
    }
    impl Default for MDMACTRL {
        #[inline(always)]
        fn default() -> MDMACTRL {
            MDMACTRL(0)
        }
    }
    impl core::fmt::Debug for MDMACTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MDMACTRL")
                .field("DMAFB", &self.DMAFB())
                .field("DMATB", &self.DMATB())
                .field("DMAWIDTH", &self.DMAWIDTH())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MDMACTRL {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MDMACTRL {{ DMAFB: {=u8:?}, DMATB: {=u8:?}, DMAWIDTH: {=u8:?} }}",
                self.DMAFB(),
                self.DMATB(),
                self.DMAWIDTH()
            )
        }
    }
    #[doc = "Controller Dynamic Address"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MDYNADDR(pub u32);
    impl MDYNADDR {
        #[must_use]
        #[inline(always)]
        pub const fn DAVALID(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DAVALID(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DADDR(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_DADDR(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
        }
    }
    impl Default for MDYNADDR {
        #[inline(always)]
        fn default() -> MDYNADDR {
            MDYNADDR(0)
        }
    }
    impl core::fmt::Debug for MDYNADDR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MDYNADDR")
                .field("DAVALID", &self.DAVALID())
                .field("DADDR", &self.DADDR())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MDYNADDR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MDYNADDR {{ DAVALID: {=bool:?}, DADDR: {=u8:?} }}",
                self.DAVALID(),
                self.DADDR()
            )
        }
    }
    #[doc = "Controller Errors and Warnings"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MERRWARN(pub u32);
    impl MERRWARN {
        #[must_use]
        #[inline(always)]
        pub const fn URUN(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_URUN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn NACK(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_NACK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WRABT(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WRABT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TERM(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TERM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn HPAR(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_HPAR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn HCRC(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_HCRC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn OREAD(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_OREAD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn OWRITE(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_OWRITE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MSGERR(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_MSGERR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INVREQ(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INVREQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TIMEOUT(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TIMEOUT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for MERRWARN {
        #[inline(always)]
        fn default() -> MERRWARN {
            MERRWARN(0)
        }
    }
    impl core::fmt::Debug for MERRWARN {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MERRWARN")
                .field("URUN", &self.URUN())
                .field("NACK", &self.NACK())
                .field("WRABT", &self.WRABT())
                .field("TERM", &self.TERM())
                .field("HPAR", &self.HPAR())
                .field("HCRC", &self.HCRC())
                .field("OREAD", &self.OREAD())
                .field("OWRITE", &self.OWRITE())
                .field("MSGERR", &self.MSGERR())
                .field("INVREQ", &self.INVREQ())
                .field("TIMEOUT", &self.TIMEOUT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MERRWARN {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MERRWARN {{ URUN: {=bool:?}, NACK: {=bool:?}, WRABT: {=bool:?}, TERM: {=bool:?}, HPAR: {=bool:?}, HCRC: {=bool:?}, OREAD: {=bool:?}, OWRITE: {=bool:?}, MSGERR: {=bool:?}, INVREQ: {=bool:?}, TIMEOUT: {=bool:?} }}" , self . URUN () , self . NACK () , self . WRABT () , self . TERM () , self . HPAR () , self . HCRC () , self . OREAD () , self . OWRITE () , self . MSGERR () , self . INVREQ () , self . TIMEOUT ())
        }
    }
    #[doc = "Controller In-band Interrupt Registry and Rules"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MIBIRULES(pub u32);
    impl MIBIRULES {
        #[must_use]
        #[inline(always)]
        pub const fn ADDR0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ADDR0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ADDR1(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ADDR1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 6usize)) | (((val as u32) & 0x3f) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ADDR2(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ADDR2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 12usize)) | (((val as u32) & 0x3f) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ADDR3(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ADDR3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 18usize)) | (((val as u32) & 0x3f) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ADDR4(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ADDR4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MSB0(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_MSB0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn NOBYTE(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_NOBYTE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MIBIRULES {
        #[inline(always)]
        fn default() -> MIBIRULES {
            MIBIRULES(0)
        }
    }
    impl core::fmt::Debug for MIBIRULES {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MIBIRULES")
                .field("ADDR0", &self.ADDR0())
                .field("ADDR1", &self.ADDR1())
                .field("ADDR2", &self.ADDR2())
                .field("ADDR3", &self.ADDR3())
                .field("ADDR4", &self.ADDR4())
                .field("MSB0", &self.MSB0())
                .field("NOBYTE", &self.NOBYTE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MIBIRULES {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MIBIRULES {{ ADDR0: {=u8:?}, ADDR1: {=u8:?}, ADDR2: {=u8:?}, ADDR3: {=u8:?}, ADDR4: {=u8:?}, MSB0: {=bool:?}, NOBYTE: {=bool:?} }}" , self . ADDR0 () , self . ADDR1 () , self . ADDR2 () , self . ADDR3 () , self . ADDR4 () , self . MSB0 () , self . NOBYTE ())
        }
    }
    #[doc = "Controller Interrupt Clear"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MINTCLR(pub u32);
    impl MINTCLR {
        #[must_use]
        #[inline(always)]
        pub const fn SLVSTART(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SLVSTART(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MCTRLDONE(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_MCTRLDONE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn COMPLETE(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_COMPLETE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RXPEND(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RXPEND(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TXNOTFULL(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TXNOTFULL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn IBIWON(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_IBIWON(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ERRWARN(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ERRWARN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn NOWMASTER(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_NOWMASTER(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
    }
    impl Default for MINTCLR {
        #[inline(always)]
        fn default() -> MINTCLR {
            MINTCLR(0)
        }
    }
    impl core::fmt::Debug for MINTCLR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MINTCLR")
                .field("SLVSTART", &self.SLVSTART())
                .field("MCTRLDONE", &self.MCTRLDONE())
                .field("COMPLETE", &self.COMPLETE())
                .field("RXPEND", &self.RXPEND())
                .field("TXNOTFULL", &self.TXNOTFULL())
                .field("IBIWON", &self.IBIWON())
                .field("ERRWARN", &self.ERRWARN())
                .field("NOWMASTER", &self.NOWMASTER())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MINTCLR {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MINTCLR {{ SLVSTART: {=bool:?}, MCTRLDONE: {=bool:?}, COMPLETE: {=bool:?}, RXPEND: {=bool:?}, TXNOTFULL: {=bool:?}, IBIWON: {=bool:?}, ERRWARN: {=bool:?}, NOWMASTER: {=bool:?} }}" , self . SLVSTART () , self . MCTRLDONE () , self . COMPLETE () , self . RXPEND () , self . TXNOTFULL () , self . IBIWON () , self . ERRWARN () , self . NOWMASTER ())
        }
    }
    #[doc = "Controller Interrupt Mask"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MINTMASKED(pub u32);
    impl MINTMASKED {
        #[must_use]
        #[inline(always)]
        pub const fn SLVSTART(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SLVSTART(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MCTRLDONE(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_MCTRLDONE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn COMPLETE(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_COMPLETE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RXPEND(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RXPEND(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TXNOTFULL(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TXNOTFULL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn IBIWON(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_IBIWON(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ERRWARN(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ERRWARN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn NOWMASTER(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_NOWMASTER(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
    }
    impl Default for MINTMASKED {
        #[inline(always)]
        fn default() -> MINTMASKED {
            MINTMASKED(0)
        }
    }
    impl core::fmt::Debug for MINTMASKED {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MINTMASKED")
                .field("SLVSTART", &self.SLVSTART())
                .field("MCTRLDONE", &self.MCTRLDONE())
                .field("COMPLETE", &self.COMPLETE())
                .field("RXPEND", &self.RXPEND())
                .field("TXNOTFULL", &self.TXNOTFULL())
                .field("IBIWON", &self.IBIWON())
                .field("ERRWARN", &self.ERRWARN())
                .field("NOWMASTER", &self.NOWMASTER())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MINTMASKED {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MINTMASKED {{ SLVSTART: {=bool:?}, MCTRLDONE: {=bool:?}, COMPLETE: {=bool:?}, RXPEND: {=bool:?}, TXNOTFULL: {=bool:?}, IBIWON: {=bool:?}, ERRWARN: {=bool:?}, NOWMASTER: {=bool:?} }}" , self . SLVSTART () , self . MCTRLDONE () , self . COMPLETE () , self . RXPEND () , self . TXNOTFULL () , self . IBIWON () , self . ERRWARN () , self . NOWMASTER ())
        }
    }
    #[doc = "Controller Interrupt Set"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MINTSET(pub u32);
    impl MINTSET {
        #[must_use]
        #[inline(always)]
        pub const fn SLVSTART(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SLVSTART(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MCTRLDONE(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_MCTRLDONE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn COMPLETE(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_COMPLETE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RXPEND(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RXPEND(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TXNOTFULL(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TXNOTFULL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn IBIWON(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_IBIWON(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ERRWARN(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ERRWARN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn NOWMASTER(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_NOWMASTER(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
    }
    impl Default for MINTSET {
        #[inline(always)]
        fn default() -> MINTSET {
            MINTSET(0)
        }
    }
    impl core::fmt::Debug for MINTSET {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MINTSET")
                .field("SLVSTART", &self.SLVSTART())
                .field("MCTRLDONE", &self.MCTRLDONE())
                .field("COMPLETE", &self.COMPLETE())
                .field("RXPEND", &self.RXPEND())
                .field("TXNOTFULL", &self.TXNOTFULL())
                .field("IBIWON", &self.IBIWON())
                .field("ERRWARN", &self.ERRWARN())
                .field("NOWMASTER", &self.NOWMASTER())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MINTSET {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MINTSET {{ SLVSTART: {=bool:?}, MCTRLDONE: {=bool:?}, COMPLETE: {=bool:?}, RXPEND: {=bool:?}, TXNOTFULL: {=bool:?}, IBIWON: {=bool:?}, ERRWARN: {=bool:?}, NOWMASTER: {=bool:?} }}" , self . SLVSTART () , self . MCTRLDONE () , self . COMPLETE () , self . RXPEND () , self . TXNOTFULL () , self . IBIWON () , self . ERRWARN () , self . NOWMASTER ())
        }
    }
    #[doc = "Controller Read Data Byte"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRDATAB(pub u32);
    impl MRDATAB {
        #[must_use]
        #[inline(always)]
        pub const fn VALUE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_VALUE(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for MRDATAB {
        #[inline(always)]
        fn default() -> MRDATAB {
            MRDATAB(0)
        }
    }
    impl core::fmt::Debug for MRDATAB {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRDATAB")
                .field("VALUE", &self.VALUE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRDATAB {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MRDATAB {{ VALUE: {=u8:?} }}", self.VALUE())
        }
    }
    #[doc = "Controller Read Data Halfword"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRDATAH(pub u32);
    impl MRDATAH {
        #[must_use]
        #[inline(always)]
        pub const fn LSB(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_LSB(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MSB(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_MSB(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for MRDATAH {
        #[inline(always)]
        fn default() -> MRDATAH {
            MRDATAH(0)
        }
    }
    impl core::fmt::Debug for MRDATAH {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRDATAH")
                .field("LSB", &self.LSB())
                .field("MSB", &self.MSB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRDATAH {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MRDATAH {{ LSB: {=u8:?}, MSB: {=u8:?} }}",
                self.LSB(),
                self.MSB()
            )
        }
    }
    #[doc = "Controller Read Message in DDR mode"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRMSG_DDR(pub u32);
    impl MRMSG_DDR {
        #[must_use]
        #[inline(always)]
        pub const fn DATA(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_DATA(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for MRMSG_DDR {
        #[inline(always)]
        fn default() -> MRMSG_DDR {
            MRMSG_DDR(0)
        }
    }
    impl core::fmt::Debug for MRMSG_DDR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRMSG_DDR")
                .field("DATA", &self.DATA())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRMSG_DDR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MRMSG_DDR {{ DATA: {=u16:?} }}", self.DATA())
        }
    }
    #[doc = "Controller Read Message in SDR mode"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRMSG_SDR(pub u32);
    impl MRMSG_SDR {
        #[must_use]
        #[inline(always)]
        pub const fn DATA(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_DATA(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for MRMSG_SDR {
        #[inline(always)]
        fn default() -> MRMSG_SDR {
            MRMSG_SDR(0)
        }
    }
    impl core::fmt::Debug for MRMSG_SDR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRMSG_SDR")
                .field("DATA", &self.DATA())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRMSG_SDR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MRMSG_SDR {{ DATA: {=u16:?} }}", self.DATA())
        }
    }
    #[doc = "Controller Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MSTATUS(pub u32);
    impl MSTATUS {
        #[must_use]
        #[inline(always)]
        pub const fn STATE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_STATE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BETWEEN(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BETWEEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn NACKED(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_NACKED(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn IBITYPE(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_IBITYPE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SLVSTART(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SLVSTART(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MCTRLDONE(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_MCTRLDONE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn COMPLETE(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_COMPLETE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RXPEND(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RXPEND(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TXNOTFULL(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TXNOTFULL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn IBIWON(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_IBIWON(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ERRWARN(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ERRWARN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn NOWMASTER(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_NOWMASTER(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn IBIADDR(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_IBIADDR(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 24usize)) | (((val as u32) & 0x7f) << 24usize);
        }
    }
    impl Default for MSTATUS {
        #[inline(always)]
        fn default() -> MSTATUS {
            MSTATUS(0)
        }
    }
    impl core::fmt::Debug for MSTATUS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MSTATUS")
                .field("STATE", &self.STATE())
                .field("BETWEEN", &self.BETWEEN())
                .field("NACKED", &self.NACKED())
                .field("IBITYPE", &self.IBITYPE())
                .field("SLVSTART", &self.SLVSTART())
                .field("MCTRLDONE", &self.MCTRLDONE())
                .field("COMPLETE", &self.COMPLETE())
                .field("RXPEND", &self.RXPEND())
                .field("TXNOTFULL", &self.TXNOTFULL())
                .field("IBIWON", &self.IBIWON())
                .field("ERRWARN", &self.ERRWARN())
                .field("NOWMASTER", &self.NOWMASTER())
                .field("IBIADDR", &self.IBIADDR())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MSTATUS {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MSTATUS {{ STATE: {=u8:?}, BETWEEN: {=bool:?}, NACKED: {=bool:?}, IBITYPE: {=u8:?}, SLVSTART: {=bool:?}, MCTRLDONE: {=bool:?}, COMPLETE: {=bool:?}, RXPEND: {=bool:?}, TXNOTFULL: {=bool:?}, IBIWON: {=bool:?}, ERRWARN: {=bool:?}, NOWMASTER: {=bool:?}, IBIADDR: {=u8:?} }}" , self . STATE () , self . BETWEEN () , self . NACKED () , self . IBITYPE () , self . SLVSTART () , self . MCTRLDONE () , self . COMPLETE () , self . RXPEND () , self . TXNOTFULL () , self . IBIWON () , self . ERRWARN () , self . NOWMASTER () , self . IBIADDR ())
        }
    }
    #[doc = "Controller Write Data Byte"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MWDATAB(pub u32);
    impl MWDATAB {
        #[must_use]
        #[inline(always)]
        pub const fn VALUE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_VALUE(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn END(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_END(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn END_ALSO(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_END_ALSO(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for MWDATAB {
        #[inline(always)]
        fn default() -> MWDATAB {
            MWDATAB(0)
        }
    }
    impl core::fmt::Debug for MWDATAB {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MWDATAB")
                .field("VALUE", &self.VALUE())
                .field("END", &self.END())
                .field("END_ALSO", &self.END_ALSO())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MWDATAB {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MWDATAB {{ VALUE: {=u8:?}, END: {=bool:?}, END_ALSO: {=bool:?} }}",
                self.VALUE(),
                self.END(),
                self.END_ALSO()
            )
        }
    }
    #[doc = "Controller Write Byte Data 1 (to Bus)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MWDATAB1(pub u32);
    impl MWDATAB1 {
        #[must_use]
        #[inline(always)]
        pub const fn VALUE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_VALUE(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for MWDATAB1 {
        #[inline(always)]
        fn default() -> MWDATAB1 {
            MWDATAB1(0)
        }
    }
    impl core::fmt::Debug for MWDATAB1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MWDATAB1")
                .field("VALUE", &self.VALUE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MWDATAB1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MWDATAB1 {{ VALUE: {=u8:?} }}", self.VALUE())
        }
    }
    #[doc = "Controller Write Data Byte End"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MWDATABE(pub u32);
    impl MWDATABE {
        #[must_use]
        #[inline(always)]
        pub const fn VALUE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_VALUE(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for MWDATABE {
        #[inline(always)]
        fn default() -> MWDATABE {
            MWDATABE(0)
        }
    }
    impl core::fmt::Debug for MWDATABE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MWDATABE")
                .field("VALUE", &self.VALUE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MWDATABE {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MWDATABE {{ VALUE: {=u8:?} }}", self.VALUE())
        }
    }
    #[doc = "Controller Write Data Halfword"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MWDATAH(pub u32);
    impl MWDATAH {
        #[must_use]
        #[inline(always)]
        pub const fn DATA0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_DATA0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DATA1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_DATA1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn END(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_END(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for MWDATAH {
        #[inline(always)]
        fn default() -> MWDATAH {
            MWDATAH(0)
        }
    }
    impl core::fmt::Debug for MWDATAH {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MWDATAH")
                .field("DATA0", &self.DATA0())
                .field("DATA1", &self.DATA1())
                .field("END", &self.END())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MWDATAH {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MWDATAH {{ DATA0: {=u8:?}, DATA1: {=u8:?}, END: {=bool:?} }}",
                self.DATA0(),
                self.DATA1(),
                self.END()
            )
        }
    }
    #[doc = "Controller Write Halfword Data (to Bus)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MWDATAH1(pub u32);
    impl MWDATAH1 {
        #[must_use]
        #[inline(always)]
        pub const fn VALUE(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_VALUE(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for MWDATAH1 {
        #[inline(always)]
        fn default() -> MWDATAH1 {
            MWDATAH1(0)
        }
    }
    impl core::fmt::Debug for MWDATAH1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MWDATAH1")
                .field("VALUE", &self.VALUE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MWDATAH1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MWDATAH1 {{ VALUE: {=u16:?} }}", self.VALUE())
        }
    }
    #[doc = "Controller Write Data Halfword End"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MWDATAHE(pub u32);
    impl MWDATAHE {
        #[must_use]
        #[inline(always)]
        pub const fn DATA0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_DATA0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DATA1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_DATA1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for MWDATAHE {
        #[inline(always)]
        fn default() -> MWDATAHE {
            MWDATAHE(0)
        }
    }
    impl core::fmt::Debug for MWDATAHE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MWDATAHE")
                .field("DATA0", &self.DATA0())
                .field("DATA1", &self.DATA1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MWDATAHE {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MWDATAHE {{ DATA0: {=u8:?}, DATA1: {=u8:?} }}",
                self.DATA0(),
                self.DATA1()
            )
        }
    }
    #[doc = "Controller Write Message in DDR mode: First Control Word"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MWMSG_DDR_CONTROL(pub u32);
    impl MWMSG_DDR_CONTROL {
        #[must_use]
        #[inline(always)]
        pub const fn ADDRCMD(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_ADDRCMD(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for MWMSG_DDR_CONTROL {
        #[inline(always)]
        fn default() -> MWMSG_DDR_CONTROL {
            MWMSG_DDR_CONTROL(0)
        }
    }
    impl core::fmt::Debug for MWMSG_DDR_CONTROL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MWMSG_DDR_CONTROL")
                .field("ADDRCMD", &self.ADDRCMD())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MWMSG_DDR_CONTROL {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MWMSG_DDR_CONTROL {{ ADDRCMD: {=u16:?} }}",
                self.ADDRCMD()
            )
        }
    }
    #[doc = "Controller Write Message in DDR Mode Control 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MWMSG_DDR_CONTROL2(pub u32);
    impl MWMSG_DDR_CONTROL2 {
        #[must_use]
        #[inline(always)]
        pub const fn LEN(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_LEN(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn END(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_END(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for MWMSG_DDR_CONTROL2 {
        #[inline(always)]
        fn default() -> MWMSG_DDR_CONTROL2 {
            MWMSG_DDR_CONTROL2(0)
        }
    }
    impl core::fmt::Debug for MWMSG_DDR_CONTROL2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MWMSG_DDR_CONTROL2")
                .field("LEN", &self.LEN())
                .field("END", &self.END())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MWMSG_DDR_CONTROL2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MWMSG_DDR_CONTROL2 {{ LEN: {=u16:?}, END: {=bool:?} }}",
                self.LEN(),
                self.END()
            )
        }
    }
    #[doc = "Controller Write Message Data in DDR mode"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MWMSG_DDR_DATA(pub u32);
    impl MWMSG_DDR_DATA {
        #[must_use]
        #[inline(always)]
        pub const fn DATA16B(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_DATA16B(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for MWMSG_DDR_DATA {
        #[inline(always)]
        fn default() -> MWMSG_DDR_DATA {
            MWMSG_DDR_DATA(0)
        }
    }
    impl core::fmt::Debug for MWMSG_DDR_DATA {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MWMSG_DDR_DATA")
                .field("DATA16B", &self.DATA16B())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MWMSG_DDR_DATA {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MWMSG_DDR_DATA {{ DATA16B: {=u16:?} }}", self.DATA16B())
        }
    }
    #[doc = "Controller Write Message Control in SDR mode"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MWMSG_SDR_CONTROL(pub u32);
    impl MWMSG_SDR_CONTROL {
        #[must_use]
        #[inline(always)]
        pub const fn DIR(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DIR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ADDR(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ADDR(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn END(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_END(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn I2C(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_I2C(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LEN(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_LEN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u32) & 0x1f) << 11usize);
        }
    }
    impl Default for MWMSG_SDR_CONTROL {
        #[inline(always)]
        fn default() -> MWMSG_SDR_CONTROL {
            MWMSG_SDR_CONTROL(0)
        }
    }
    impl core::fmt::Debug for MWMSG_SDR_CONTROL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MWMSG_SDR_CONTROL")
                .field("DIR", &self.DIR())
                .field("ADDR", &self.ADDR())
                .field("END", &self.END())
                .field("I2C", &self.I2C())
                .field("LEN", &self.LEN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MWMSG_SDR_CONTROL {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MWMSG_SDR_CONTROL {{ DIR: {=bool:?}, ADDR: {=u8:?}, END: {=bool:?}, I2C: {=bool:?}, LEN: {=u8:?} }}" , self . DIR () , self . ADDR () , self . END () , self . I2C () , self . LEN ())
        }
    }
    #[doc = "Controller Write Message Data in SDR mode"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MWMSG_SDR_DATA(pub u32);
    impl MWMSG_SDR_DATA {
        #[must_use]
        #[inline(always)]
        pub const fn DATA16B(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_DATA16B(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for MWMSG_SDR_DATA {
        #[inline(always)]
        fn default() -> MWMSG_SDR_DATA {
            MWMSG_SDR_DATA(0)
        }
    }
    impl core::fmt::Debug for MWMSG_SDR_DATA {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MWMSG_SDR_DATA")
                .field("DATA16B", &self.DATA16B())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MWMSG_SDR_DATA {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MWMSG_SDR_DATA {{ DATA16B: {=u16:?} }}", self.DATA16B())
        }
    }
    #[doc = "Target Capabilities"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SCAPABILITIES(pub u32);
    impl SCAPABILITIES {
        #[must_use]
        #[inline(always)]
        pub const fn IDENA(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_IDENA(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn IDREG(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_IDREG(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 2usize)) | (((val as u32) & 0x0f) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn HDRSUPP(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_HDRSUPP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MASTER(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_MASTER(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SADDR(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SADDR(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CCCHANDLE(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CCCHANDLE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn IBI_MR_HJ(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_IBI_MR_HJ(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TIMECTRL(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TIMECTRL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn EXTFIFO(&self) -> u8 {
            let val = (self.0 >> 23usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_EXTFIFO(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 23usize)) | (((val as u32) & 0x07) << 23usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FIFOTX(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_FIFOTX(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FIFORX(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_FIFORX(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DMA(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DMA(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for SCAPABILITIES {
        #[inline(always)]
        fn default() -> SCAPABILITIES {
            SCAPABILITIES(0)
        }
    }
    impl core::fmt::Debug for SCAPABILITIES {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SCAPABILITIES")
                .field("IDENA", &self.IDENA())
                .field("IDREG", &self.IDREG())
                .field("HDRSUPP", &self.HDRSUPP())
                .field("MASTER", &self.MASTER())
                .field("SADDR", &self.SADDR())
                .field("CCCHANDLE", &self.CCCHANDLE())
                .field("IBI_MR_HJ", &self.IBI_MR_HJ())
                .field("TIMECTRL", &self.TIMECTRL())
                .field("EXTFIFO", &self.EXTFIFO())
                .field("FIFOTX", &self.FIFOTX())
                .field("FIFORX", &self.FIFORX())
                .field("INT", &self.INT())
                .field("DMA", &self.DMA())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SCAPABILITIES {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SCAPABILITIES {{ IDENA: {=u8:?}, IDREG: {=u8:?}, HDRSUPP: {=u8:?}, MASTER: {=bool:?}, SADDR: {=u8:?}, CCCHANDLE: {=u8:?}, IBI_MR_HJ: {=u8:?}, TIMECTRL: {=bool:?}, EXTFIFO: {=u8:?}, FIFOTX: {=u8:?}, FIFORX: {=u8:?}, INT: {=bool:?}, DMA: {=bool:?} }}" , self . IDENA () , self . IDREG () , self . HDRSUPP () , self . MASTER () , self . SADDR () , self . CCCHANDLE () , self . IBI_MR_HJ () , self . TIMECTRL () , self . EXTFIFO () , self . FIFOTX () , self . FIFORX () , self . INT () , self . DMA ())
        }
    }
    #[doc = "Target Capabilities 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SCAPABILITIES2(pub u32);
    impl SCAPABILITIES2 {
        #[must_use]
        #[inline(always)]
        pub const fn MAPCNT(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_MAPCNT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn I2C10B(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_I2C10B(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn I2CDEVID(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_I2CDEVID(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn IBIEXT(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_IBIEXT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn IBIXREG(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_IBIXREG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SLVRST(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SLVRST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn GROUP(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_GROUP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn AASA(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_AASA(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SSTSUB(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SSTSUB(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SSTWR(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SSTWR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for SCAPABILITIES2 {
        #[inline(always)]
        fn default() -> SCAPABILITIES2 {
            SCAPABILITIES2(0)
        }
    }
    impl core::fmt::Debug for SCAPABILITIES2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SCAPABILITIES2")
                .field("MAPCNT", &self.MAPCNT())
                .field("I2C10B", &self.I2C10B())
                .field("I2CDEVID", &self.I2CDEVID())
                .field("IBIEXT", &self.IBIEXT())
                .field("IBIXREG", &self.IBIXREG())
                .field("SLVRST", &self.SLVRST())
                .field("GROUP", &self.GROUP())
                .field("AASA", &self.AASA())
                .field("SSTSUB", &self.SSTSUB())
                .field("SSTWR", &self.SSTWR())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SCAPABILITIES2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SCAPABILITIES2 {{ MAPCNT: {=u8:?}, I2C10B: {=bool:?}, I2CDEVID: {=bool:?}, IBIEXT: {=bool:?}, IBIXREG: {=bool:?}, SLVRST: {=bool:?}, GROUP: {=u8:?}, AASA: {=bool:?}, SSTSUB: {=bool:?}, SSTWR: {=bool:?} }}" , self . MAPCNT () , self . I2C10B () , self . I2CDEVID () , self . IBIEXT () , self . IBIXREG () , self . SLVRST () , self . GROUP () , self . AASA () , self . SSTSUB () , self . SSTWR ())
        }
    }
    #[doc = "Target Configuration"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SCONFIG(pub u32);
    impl SCONFIG {
        #[must_use]
        #[inline(always)]
        pub const fn SLVENA(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SLVENA(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn NACK(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_NACK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MATCHSS(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_MATCHSS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn S0IGNORE(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_S0IGNORE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn HDROK(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_HDROK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn OFFLINE(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_OFFLINE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BAMATCH(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_BAMATCH(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SADDR(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SADDR(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 25usize)) | (((val as u32) & 0x7f) << 25usize);
        }
    }
    impl Default for SCONFIG {
        #[inline(always)]
        fn default() -> SCONFIG {
            SCONFIG(0)
        }
    }
    impl core::fmt::Debug for SCONFIG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SCONFIG")
                .field("SLVENA", &self.SLVENA())
                .field("NACK", &self.NACK())
                .field("MATCHSS", &self.MATCHSS())
                .field("S0IGNORE", &self.S0IGNORE())
                .field("HDROK", &self.HDROK())
                .field("OFFLINE", &self.OFFLINE())
                .field("BAMATCH", &self.BAMATCH())
                .field("SADDR", &self.SADDR())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SCONFIG {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SCONFIG {{ SLVENA: {=bool:?}, NACK: {=bool:?}, MATCHSS: {=bool:?}, S0IGNORE: {=bool:?}, HDROK: {=bool:?}, OFFLINE: {=bool:?}, BAMATCH: {=u8:?}, SADDR: {=u8:?} }}" , self . SLVENA () , self . NACK () , self . MATCHSS () , self . S0IGNORE () , self . HDROK () , self . OFFLINE () , self . BAMATCH () , self . SADDR ())
        }
    }
    #[doc = "Target Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SCTRL(pub u32);
    impl SCTRL {
        #[must_use]
        #[inline(always)]
        pub const fn EVENT(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_EVENT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn EXTDATA(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_EXTDATA(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn IBIDATA(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_IBIDATA(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PENDINT(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_PENDINT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ACTSTATE(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ACTSTATE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn VENDINFO(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_VENDINFO(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for SCTRL {
        #[inline(always)]
        fn default() -> SCTRL {
            SCTRL(0)
        }
    }
    impl core::fmt::Debug for SCTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SCTRL")
                .field("EVENT", &self.EVENT())
                .field("EXTDATA", &self.EXTDATA())
                .field("IBIDATA", &self.IBIDATA())
                .field("PENDINT", &self.PENDINT())
                .field("ACTSTATE", &self.ACTSTATE())
                .field("VENDINFO", &self.VENDINFO())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SCTRL {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SCTRL {{ EVENT: {=u8:?}, EXTDATA: {=bool:?}, IBIDATA: {=u8:?}, PENDINT: {=u8:?}, ACTSTATE: {=u8:?}, VENDINFO: {=u8:?} }}" , self . EVENT () , self . EXTDATA () , self . IBIDATA () , self . PENDINT () , self . ACTSTATE () , self . VENDINFO ())
        }
    }
    #[doc = "Target Data Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SDATACTRL(pub u32);
    impl SDATACTRL {
        #[must_use]
        #[inline(always)]
        pub const fn FLUSHTB(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FLUSHTB(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FLUSHFB(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FLUSHFB(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn UNLOCK(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_UNLOCK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TXTRIG(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TXTRIG(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RXTRIG(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RXTRIG(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TXCOUNT(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TXCOUNT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RXCOUNT(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RXCOUNT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TXFULL(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TXFULL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RXEMPTY(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RXEMPTY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for SDATACTRL {
        #[inline(always)]
        fn default() -> SDATACTRL {
            SDATACTRL(0)
        }
    }
    impl core::fmt::Debug for SDATACTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SDATACTRL")
                .field("FLUSHTB", &self.FLUSHTB())
                .field("FLUSHFB", &self.FLUSHFB())
                .field("UNLOCK", &self.UNLOCK())
                .field("TXTRIG", &self.TXTRIG())
                .field("RXTRIG", &self.RXTRIG())
                .field("TXCOUNT", &self.TXCOUNT())
                .field("RXCOUNT", &self.RXCOUNT())
                .field("TXFULL", &self.TXFULL())
                .field("RXEMPTY", &self.RXEMPTY())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SDATACTRL {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SDATACTRL {{ FLUSHTB: {=bool:?}, FLUSHFB: {=bool:?}, UNLOCK: {=bool:?}, TXTRIG: {=u8:?}, RXTRIG: {=u8:?}, TXCOUNT: {=u8:?}, RXCOUNT: {=u8:?}, TXFULL: {=bool:?}, RXEMPTY: {=bool:?} }}" , self . FLUSHTB () , self . FLUSHFB () , self . UNLOCK () , self . TXTRIG () , self . RXTRIG () , self . TXCOUNT () , self . RXCOUNT () , self . TXFULL () , self . RXEMPTY ())
        }
    }
    #[doc = "Target DMA Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SDMACTRL(pub u32);
    impl SDMACTRL {
        #[must_use]
        #[inline(always)]
        pub const fn DMAFB(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_DMAFB(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DMATB(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_DMATB(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DMAWIDTH(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_DMAWIDTH(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
    }
    impl Default for SDMACTRL {
        #[inline(always)]
        fn default() -> SDMACTRL {
            SDMACTRL(0)
        }
    }
    impl core::fmt::Debug for SDMACTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SDMACTRL")
                .field("DMAFB", &self.DMAFB())
                .field("DMATB", &self.DMATB())
                .field("DMAWIDTH", &self.DMAWIDTH())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SDMACTRL {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SDMACTRL {{ DMAFB: {=u8:?}, DMATB: {=u8:?}, DMAWIDTH: {=u8:?} }}",
                self.DMAFB(),
                self.DMATB(),
                self.DMAWIDTH()
            )
        }
    }
    #[doc = "Target Dynamic Address"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SDYNADDR(pub u32);
    impl SDYNADDR {
        #[must_use]
        #[inline(always)]
        pub const fn DAVALID(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DAVALID(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DADDR(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_DADDR(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MAPSA(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_MAPSA(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SA10B(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SA10B(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn KEY(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_KEY(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for SDYNADDR {
        #[inline(always)]
        fn default() -> SDYNADDR {
            SDYNADDR(0)
        }
    }
    impl core::fmt::Debug for SDYNADDR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SDYNADDR")
                .field("DAVALID", &self.DAVALID())
                .field("DADDR", &self.DADDR())
                .field("MAPSA", &self.MAPSA())
                .field("SA10B", &self.SA10B())
                .field("KEY", &self.KEY())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SDYNADDR {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SDYNADDR {{ DAVALID: {=bool:?}, DADDR: {=u8:?}, MAPSA: {=bool:?}, SA10B: {=u8:?}, KEY: {=u16:?} }}" , self . DAVALID () , self . DADDR () , self . MAPSA () , self . SA10B () , self . KEY ())
        }
    }
    #[doc = "Target Errors and Warnings"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SERRWARN(pub u32);
    impl SERRWARN {
        #[must_use]
        #[inline(always)]
        pub const fn ORUN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ORUN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn URUN(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_URUN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn URUNNACK(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_URUNNACK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TERM(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TERM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INVSTART(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INVSTART(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SPAR(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SPAR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn HPAR(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_HPAR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn HCRC(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_HCRC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn S0S1(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_S0S1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn OREAD(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_OREAD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn OWRITE(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_OWRITE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for SERRWARN {
        #[inline(always)]
        fn default() -> SERRWARN {
            SERRWARN(0)
        }
    }
    impl core::fmt::Debug for SERRWARN {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SERRWARN")
                .field("ORUN", &self.ORUN())
                .field("URUN", &self.URUN())
                .field("URUNNACK", &self.URUNNACK())
                .field("TERM", &self.TERM())
                .field("INVSTART", &self.INVSTART())
                .field("SPAR", &self.SPAR())
                .field("HPAR", &self.HPAR())
                .field("HCRC", &self.HCRC())
                .field("S0S1", &self.S0S1())
                .field("OREAD", &self.OREAD())
                .field("OWRITE", &self.OWRITE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SERRWARN {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SERRWARN {{ ORUN: {=bool:?}, URUN: {=bool:?}, URUNNACK: {=bool:?}, TERM: {=bool:?}, INVSTART: {=bool:?}, SPAR: {=bool:?}, HPAR: {=bool:?}, HCRC: {=bool:?}, S0S1: {=bool:?}, OREAD: {=bool:?}, OWRITE: {=bool:?} }}" , self . ORUN () , self . URUN () , self . URUNNACK () , self . TERM () , self . INVSTART () , self . SPAR () , self . HPAR () , self . HCRC () , self . S0S1 () , self . OREAD () , self . OWRITE ())
        }
    }
    #[doc = "Target ID Extension"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SIDEXT(pub u32);
    impl SIDEXT {
        #[must_use]
        #[inline(always)]
        pub const fn DCR(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_DCR(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BCR(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_BCR(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for SIDEXT {
        #[inline(always)]
        fn default() -> SIDEXT {
            SIDEXT(0)
        }
    }
    impl core::fmt::Debug for SIDEXT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SIDEXT")
                .field("DCR", &self.DCR())
                .field("BCR", &self.BCR())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SIDEXT {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SIDEXT {{ DCR: {=u8:?}, BCR: {=u8:?} }}",
                self.DCR(),
                self.BCR()
            )
        }
    }
    #[doc = "Target Interrupt Clear"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SINTCLR(pub u32);
    impl SINTCLR {
        #[must_use]
        #[inline(always)]
        pub const fn START(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_START(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MATCHED(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_MATCHED(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn STOP(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_STOP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RXPEND(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RXPEND(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TXSEND(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TXSEND(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DACHG(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DACHG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CCC(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CCC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ERRWARN(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ERRWARN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DDRMATCHED(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DDRMATCHED(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CHANDLED(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CHANDLED(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn EVENT(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_EVENT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
    }
    impl Default for SINTCLR {
        #[inline(always)]
        fn default() -> SINTCLR {
            SINTCLR(0)
        }
    }
    impl core::fmt::Debug for SINTCLR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SINTCLR")
                .field("START", &self.START())
                .field("MATCHED", &self.MATCHED())
                .field("STOP", &self.STOP())
                .field("RXPEND", &self.RXPEND())
                .field("TXSEND", &self.TXSEND())
                .field("DACHG", &self.DACHG())
                .field("CCC", &self.CCC())
                .field("ERRWARN", &self.ERRWARN())
                .field("DDRMATCHED", &self.DDRMATCHED())
                .field("CHANDLED", &self.CHANDLED())
                .field("EVENT", &self.EVENT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SINTCLR {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SINTCLR {{ START: {=bool:?}, MATCHED: {=bool:?}, STOP: {=bool:?}, RXPEND: {=bool:?}, TXSEND: {=bool:?}, DACHG: {=bool:?}, CCC: {=bool:?}, ERRWARN: {=bool:?}, DDRMATCHED: {=bool:?}, CHANDLED: {=bool:?}, EVENT: {=bool:?} }}" , self . START () , self . MATCHED () , self . STOP () , self . RXPEND () , self . TXSEND () , self . DACHG () , self . CCC () , self . ERRWARN () , self . DDRMATCHED () , self . CHANDLED () , self . EVENT ())
        }
    }
    #[doc = "Target Interrupt Mask"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SINTMASKED(pub u32);
    impl SINTMASKED {
        #[must_use]
        #[inline(always)]
        pub const fn START(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_START(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MATCHED(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_MATCHED(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn STOP(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_STOP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RXPEND(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RXPEND(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TXSEND(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TXSEND(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DACHG(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DACHG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CCC(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CCC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ERRWARN(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ERRWARN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DDRMATCHED(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DDRMATCHED(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CHANDLED(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CHANDLED(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn EVENT(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_EVENT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
    }
    impl Default for SINTMASKED {
        #[inline(always)]
        fn default() -> SINTMASKED {
            SINTMASKED(0)
        }
    }
    impl core::fmt::Debug for SINTMASKED {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SINTMASKED")
                .field("START", &self.START())
                .field("MATCHED", &self.MATCHED())
                .field("STOP", &self.STOP())
                .field("RXPEND", &self.RXPEND())
                .field("TXSEND", &self.TXSEND())
                .field("DACHG", &self.DACHG())
                .field("CCC", &self.CCC())
                .field("ERRWARN", &self.ERRWARN())
                .field("DDRMATCHED", &self.DDRMATCHED())
                .field("CHANDLED", &self.CHANDLED())
                .field("EVENT", &self.EVENT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SINTMASKED {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SINTMASKED {{ START: {=bool:?}, MATCHED: {=bool:?}, STOP: {=bool:?}, RXPEND: {=bool:?}, TXSEND: {=bool:?}, DACHG: {=bool:?}, CCC: {=bool:?}, ERRWARN: {=bool:?}, DDRMATCHED: {=bool:?}, CHANDLED: {=bool:?}, EVENT: {=bool:?} }}" , self . START () , self . MATCHED () , self . STOP () , self . RXPEND () , self . TXSEND () , self . DACHG () , self . CCC () , self . ERRWARN () , self . DDRMATCHED () , self . CHANDLED () , self . EVENT ())
        }
    }
    #[doc = "Target Interrupt Set"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SINTSET(pub u32);
    impl SINTSET {
        #[must_use]
        #[inline(always)]
        pub const fn START(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_START(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MATCHED(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_MATCHED(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn STOP(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_STOP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RXPEND(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RXPEND(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TXSEND(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TXSEND(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DACHG(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DACHG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CCC(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CCC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ERRWARN(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ERRWARN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DDRMATCHED(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DDRMATCHED(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CHANDLED(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CHANDLED(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn EVENT(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_EVENT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
    }
    impl Default for SINTSET {
        #[inline(always)]
        fn default() -> SINTSET {
            SINTSET(0)
        }
    }
    impl core::fmt::Debug for SINTSET {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SINTSET")
                .field("START", &self.START())
                .field("MATCHED", &self.MATCHED())
                .field("STOP", &self.STOP())
                .field("RXPEND", &self.RXPEND())
                .field("TXSEND", &self.TXSEND())
                .field("DACHG", &self.DACHG())
                .field("CCC", &self.CCC())
                .field("ERRWARN", &self.ERRWARN())
                .field("DDRMATCHED", &self.DDRMATCHED())
                .field("CHANDLED", &self.CHANDLED())
                .field("EVENT", &self.EVENT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SINTSET {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SINTSET {{ START: {=bool:?}, MATCHED: {=bool:?}, STOP: {=bool:?}, RXPEND: {=bool:?}, TXSEND: {=bool:?}, DACHG: {=bool:?}, CCC: {=bool:?}, ERRWARN: {=bool:?}, DDRMATCHED: {=bool:?}, CHANDLED: {=bool:?}, EVENT: {=bool:?} }}" , self . START () , self . MATCHED () , self . STOP () , self . RXPEND () , self . TXSEND () , self . DACHG () , self . CCC () , self . ERRWARN () , self . DDRMATCHED () , self . CHANDLED () , self . EVENT ())
        }
    }
    #[doc = "Map Feature Control 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SMAPCTRL0(pub u32);
    impl SMAPCTRL0 {
        #[must_use]
        #[inline(always)]
        pub const fn ENA(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ENA(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DA(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_DA(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CAUSE(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CAUSE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
    }
    impl Default for SMAPCTRL0 {
        #[inline(always)]
        fn default() -> SMAPCTRL0 {
            SMAPCTRL0(0)
        }
    }
    impl core::fmt::Debug for SMAPCTRL0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SMAPCTRL0")
                .field("ENA", &self.ENA())
                .field("DA", &self.DA())
                .field("CAUSE", &self.CAUSE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SMAPCTRL0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SMAPCTRL0 {{ ENA: {=bool:?}, DA: {=u8:?}, CAUSE: {=u8:?} }}",
                self.ENA(),
                self.DA(),
                self.CAUSE()
            )
        }
    }
    #[doc = "Target Maximum Limits"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SMAXLIMITS(pub u32);
    impl SMAXLIMITS {
        #[must_use]
        #[inline(always)]
        pub const fn MAXRD(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_MAXRD(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MAXWR(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_MAXWR(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for SMAXLIMITS {
        #[inline(always)]
        fn default() -> SMAXLIMITS {
            SMAXLIMITS(0)
        }
    }
    impl core::fmt::Debug for SMAXLIMITS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SMAXLIMITS")
                .field("MAXRD", &self.MAXRD())
                .field("MAXWR", &self.MAXWR())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SMAXLIMITS {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SMAXLIMITS {{ MAXRD: {=u16:?}, MAXWR: {=u16:?} }}",
                self.MAXRD(),
                self.MAXWR()
            )
        }
    }
    #[doc = "Target Message Map Address"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SMSGMAPADDR(pub u32);
    impl SMSGMAPADDR {
        #[must_use]
        #[inline(always)]
        pub const fn MAPLAST(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_MAPLAST(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LASTSTATIC(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_LASTSTATIC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MAPLASTM1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_MAPLASTM1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MAPLASTM2(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_MAPLASTM2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for SMSGMAPADDR {
        #[inline(always)]
        fn default() -> SMSGMAPADDR {
            SMSGMAPADDR(0)
        }
    }
    impl core::fmt::Debug for SMSGMAPADDR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SMSGMAPADDR")
                .field("MAPLAST", &self.MAPLAST())
                .field("LASTSTATIC", &self.LASTSTATIC())
                .field("MAPLASTM1", &self.MAPLASTM1())
                .field("MAPLASTM2", &self.MAPLASTM2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SMSGMAPADDR {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SMSGMAPADDR {{ MAPLAST: {=u8:?}, LASTSTATIC: {=bool:?}, MAPLASTM1: {=u8:?}, MAPLASTM2: {=u8:?} }}" , self . MAPLAST () , self . LASTSTATIC () , self . MAPLASTM1 () , self . MAPLASTM2 ())
        }
    }
    #[doc = "Target Read Data Byte"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SRDATAB(pub u32);
    impl SRDATAB {
        #[must_use]
        #[inline(always)]
        pub const fn DATA0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_DATA0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for SRDATAB {
        #[inline(always)]
        fn default() -> SRDATAB {
            SRDATAB(0)
        }
    }
    impl core::fmt::Debug for SRDATAB {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SRDATAB")
                .field("DATA0", &self.DATA0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SRDATAB {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SRDATAB {{ DATA0: {=u8:?} }}", self.DATA0())
        }
    }
    #[doc = "Target Read Data Halfword"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SRDATAH(pub u32);
    impl SRDATAH {
        #[must_use]
        #[inline(always)]
        pub const fn LSB(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_LSB(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MSB(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_MSB(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for SRDATAH {
        #[inline(always)]
        fn default() -> SRDATAH {
            SRDATAH(0)
        }
    }
    impl core::fmt::Debug for SRDATAH {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SRDATAH")
                .field("LSB", &self.LSB())
                .field("MSB", &self.MSB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SRDATAH {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SRDATAH {{ LSB: {=u8:?}, MSB: {=u8:?} }}",
                self.LSB(),
                self.MSB()
            )
        }
    }
    #[doc = "Target Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SSTATUS(pub u32);
    impl SSTATUS {
        #[must_use]
        #[inline(always)]
        pub const fn STNOTSTOP(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_STNOTSTOP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn STMSG(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_STMSG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn STCCCH(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_STCCCH(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn STREQRD(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_STREQRD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn STREQWR(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_STREQWR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn STDAA(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_STDAA(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn STHDR(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_STHDR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn START(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_START(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MATCHED(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_MATCHED(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn STOP(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_STOP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RX_PEND(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RX_PEND(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TXNOTFULL(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TXNOTFULL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DACHG(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DACHG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CCC(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CCC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ERRWARN(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ERRWARN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn HDRMATCH(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_HDRMATCH(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CHANDLED(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CHANDLED(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn EVENT(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_EVENT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn EVDET(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_EVDET(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn IBIDIS(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_IBIDIS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MRDIS(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_MRDIS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn HJDIS(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_HJDIS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ACTSTATE(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ACTSTATE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TIMECTRL(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TIMECTRL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for SSTATUS {
        #[inline(always)]
        fn default() -> SSTATUS {
            SSTATUS(0)
        }
    }
    impl core::fmt::Debug for SSTATUS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SSTATUS")
                .field("STNOTSTOP", &self.STNOTSTOP())
                .field("STMSG", &self.STMSG())
                .field("STCCCH", &self.STCCCH())
                .field("STREQRD", &self.STREQRD())
                .field("STREQWR", &self.STREQWR())
                .field("STDAA", &self.STDAA())
                .field("STHDR", &self.STHDR())
                .field("START", &self.START())
                .field("MATCHED", &self.MATCHED())
                .field("STOP", &self.STOP())
                .field("RX_PEND", &self.RX_PEND())
                .field("TXNOTFULL", &self.TXNOTFULL())
                .field("DACHG", &self.DACHG())
                .field("CCC", &self.CCC())
                .field("ERRWARN", &self.ERRWARN())
                .field("HDRMATCH", &self.HDRMATCH())
                .field("CHANDLED", &self.CHANDLED())
                .field("EVENT", &self.EVENT())
                .field("EVDET", &self.EVDET())
                .field("IBIDIS", &self.IBIDIS())
                .field("MRDIS", &self.MRDIS())
                .field("HJDIS", &self.HJDIS())
                .field("ACTSTATE", &self.ACTSTATE())
                .field("TIMECTRL", &self.TIMECTRL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SSTATUS {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SSTATUS {{ STNOTSTOP: {=bool:?}, STMSG: {=bool:?}, STCCCH: {=bool:?}, STREQRD: {=bool:?}, STREQWR: {=bool:?}, STDAA: {=bool:?}, STHDR: {=bool:?}, START: {=bool:?}, MATCHED: {=bool:?}, STOP: {=bool:?}, RX_PEND: {=bool:?}, TXNOTFULL: {=bool:?}, DACHG: {=bool:?}, CCC: {=bool:?}, ERRWARN: {=bool:?}, HDRMATCH: {=bool:?}, CHANDLED: {=bool:?}, EVENT: {=bool:?}, EVDET: {=u8:?}, IBIDIS: {=bool:?}, MRDIS: {=bool:?}, HJDIS: {=bool:?}, ACTSTATE: {=u8:?}, TIMECTRL: {=u8:?} }}" , self . STNOTSTOP () , self . STMSG () , self . STCCCH () , self . STREQRD () , self . STREQWR () , self . STDAA () , self . STHDR () , self . START () , self . MATCHED () , self . STOP () , self . RX_PEND () , self . TXNOTFULL () , self . DACHG () , self . CCC () , self . ERRWARN () , self . HDRMATCH () , self . CHANDLED () , self . EVENT () , self . EVDET () , self . IBIDIS () , self . MRDIS () , self . HJDIS () , self . ACTSTATE () , self . TIMECTRL ())
        }
    }
    #[doc = "Target Time Control Clock"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct STCCLOCK(pub u32);
    impl STCCLOCK {
        #[must_use]
        #[inline(always)]
        pub const fn ACCURACY(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ACCURACY(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FREQ(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_FREQ(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for STCCLOCK {
        #[inline(always)]
        fn default() -> STCCLOCK {
            STCCLOCK(0)
        }
    }
    impl core::fmt::Debug for STCCLOCK {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("STCCLOCK")
                .field("ACCURACY", &self.ACCURACY())
                .field("FREQ", &self.FREQ())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for STCCLOCK {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "STCCLOCK {{ ACCURACY: {=u8:?}, FREQ: {=u8:?} }}",
                self.ACCURACY(),
                self.FREQ()
            )
        }
    }
    #[doc = "Target Vendor ID"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SVENDORID(pub u32);
    impl SVENDORID {
        #[must_use]
        #[inline(always)]
        pub const fn VID(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x7fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_VID(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
        }
    }
    impl Default for SVENDORID {
        #[inline(always)]
        fn default() -> SVENDORID {
            SVENDORID(0)
        }
    }
    impl core::fmt::Debug for SVENDORID {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SVENDORID")
                .field("VID", &self.VID())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SVENDORID {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SVENDORID {{ VID: {=u16:?} }}", self.VID())
        }
    }
    #[doc = "Target Write Data Byte"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SWDATAB(pub u32);
    impl SWDATAB {
        #[must_use]
        #[inline(always)]
        pub const fn DATA(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_DATA(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn END(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_END(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn END_ALSO(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_END_ALSO(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for SWDATAB {
        #[inline(always)]
        fn default() -> SWDATAB {
            SWDATAB(0)
        }
    }
    impl core::fmt::Debug for SWDATAB {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SWDATAB")
                .field("DATA", &self.DATA())
                .field("END", &self.END())
                .field("END_ALSO", &self.END_ALSO())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SWDATAB {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SWDATAB {{ DATA: {=u8:?}, END: {=bool:?}, END_ALSO: {=bool:?} }}",
                self.DATA(),
                self.END(),
                self.END_ALSO()
            )
        }
    }
    #[doc = "Target Write Data Byte"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SWDATAB1(pub u32);
    impl SWDATAB1 {
        #[must_use]
        #[inline(always)]
        pub const fn DATA(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_DATA(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for SWDATAB1 {
        #[inline(always)]
        fn default() -> SWDATAB1 {
            SWDATAB1(0)
        }
    }
    impl core::fmt::Debug for SWDATAB1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SWDATAB1")
                .field("DATA", &self.DATA())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SWDATAB1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SWDATAB1 {{ DATA: {=u8:?} }}", self.DATA())
        }
    }
    #[doc = "Target Write Data Byte End"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SWDATABE(pub u32);
    impl SWDATABE {
        #[must_use]
        #[inline(always)]
        pub const fn DATA(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_DATA(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for SWDATABE {
        #[inline(always)]
        fn default() -> SWDATABE {
            SWDATABE(0)
        }
    }
    impl core::fmt::Debug for SWDATABE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SWDATABE")
                .field("DATA", &self.DATA())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SWDATABE {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SWDATABE {{ DATA: {=u8:?} }}", self.DATA())
        }
    }
    #[doc = "Target Write Data Halfword"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SWDATAH(pub u32);
    impl SWDATAH {
        #[must_use]
        #[inline(always)]
        pub const fn DATA0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_DATA0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DATA1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_DATA1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn END(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_END(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for SWDATAH {
        #[inline(always)]
        fn default() -> SWDATAH {
            SWDATAH(0)
        }
    }
    impl core::fmt::Debug for SWDATAH {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SWDATAH")
                .field("DATA0", &self.DATA0())
                .field("DATA1", &self.DATA1())
                .field("END", &self.END())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SWDATAH {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SWDATAH {{ DATA0: {=u8:?}, DATA1: {=u8:?}, END: {=bool:?} }}",
                self.DATA0(),
                self.DATA1(),
                self.END()
            )
        }
    }
    #[doc = "Target Write Data Halfword"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SWDATAH1(pub u32);
    impl SWDATAH1 {
        #[must_use]
        #[inline(always)]
        pub const fn DATA(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_DATA(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for SWDATAH1 {
        #[inline(always)]
        fn default() -> SWDATAH1 {
            SWDATAH1(0)
        }
    }
    impl core::fmt::Debug for SWDATAH1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SWDATAH1")
                .field("DATA", &self.DATA())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SWDATAH1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SWDATAH1 {{ DATA: {=u16:?} }}", self.DATA())
        }
    }
    #[doc = "Target Write Data Halfword End"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SWDATAHE(pub u32);
    impl SWDATAHE {
        #[must_use]
        #[inline(always)]
        pub const fn DATA0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_DATA0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DATA1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_DATA1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for SWDATAHE {
        #[inline(always)]
        fn default() -> SWDATAHE {
            SWDATAHE(0)
        }
    }
    impl core::fmt::Debug for SWDATAHE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SWDATAHE")
                .field("DATA0", &self.DATA0())
                .field("DATA1", &self.DATA1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SWDATAHE {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SWDATAHE {{ DATA0: {=u8:?}, DATA1: {=u8:?} }}",
                self.DATA0(),
                self.DATA1()
            )
        }
    }
}
