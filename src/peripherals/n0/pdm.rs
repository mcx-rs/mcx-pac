#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (d5ec99b 2024-12-16))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PDM {
    ptr: *mut u8,
}
unsafe impl Send for PDM {}
unsafe impl Sync for PDM {}
impl PDM {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn CTRL_1(self) -> crate::common::Reg<regs::CTRL_1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn CTRL_2(self) -> crate::common::Reg<regs::CTRL_2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn STAT(self) -> crate::common::Reg<regs::STAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn FIFO_CTRL(self) -> crate::common::Reg<regs::FIFO_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn FIFO_STAT(self) -> crate::common::Reg<regs::FIFO_STAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn DATACH(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn DC_CTRL(self) -> crate::common::Reg<regs::DC_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[inline(always)]
    pub const fn DC_OUT_CTRL(self) -> crate::common::Reg<regs::DC_OUT_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
    #[inline(always)]
    pub const fn RANGE_CTRL(self) -> crate::common::Reg<regs::RANGE_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x74usize) as _) }
    }
    #[inline(always)]
    pub const fn RANGE_STAT(self) -> crate::common::Reg<regs::RANGE_STAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x7cusize) as _) }
    }
    #[inline(always)]
    pub const fn FSYNC_CTRL(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[inline(always)]
    pub const fn VERID(self) -> crate::common::Reg<regs::VERID, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[inline(always)]
    pub const fn PARAM(self) -> crate::common::Reg<regs::PARAM, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
}
pub mod regs {
    #[doc = "MICFIL Control 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CTRL_1(pub u32);
    impl CTRL_1 {
        #[inline(always)]
        pub const fn CH0EN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CH0EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn CH1EN(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CH1EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn CH2EN(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CH2EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn CH3EN(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CH3EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn FSYNCEN(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FSYNCEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn DECFILS(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DECFILS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn ERREN(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ERREN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn DISEL(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_DISEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[inline(always)]
        pub const fn DBGE(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DBGE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn SRES(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SRES(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn DBG(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DBG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn PDMIEN(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PDMIEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn DOZEN(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DOZEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn MDIS(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_MDIS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for CTRL_1 {
        #[inline(always)]
        fn default() -> CTRL_1 {
            CTRL_1(0)
        }
    }
    impl core::fmt::Debug for CTRL_1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CTRL_1")
                .field("CH0EN", &self.CH0EN())
                .field("CH1EN", &self.CH1EN())
                .field("CH2EN", &self.CH2EN())
                .field("CH3EN", &self.CH3EN())
                .field("FSYNCEN", &self.FSYNCEN())
                .field("DECFILS", &self.DECFILS())
                .field("ERREN", &self.ERREN())
                .field("DISEL", &self.DISEL())
                .field("DBGE", &self.DBGE())
                .field("SRES", &self.SRES())
                .field("DBG", &self.DBG())
                .field("PDMIEN", &self.PDMIEN())
                .field("DOZEN", &self.DOZEN())
                .field("MDIS", &self.MDIS())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CTRL_1 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CTRL_1 {
                CH0EN: bool,
                CH1EN: bool,
                CH2EN: bool,
                CH3EN: bool,
                FSYNCEN: bool,
                DECFILS: bool,
                ERREN: bool,
                DISEL: u8,
                DBGE: bool,
                SRES: bool,
                DBG: bool,
                PDMIEN: bool,
                DOZEN: bool,
                MDIS: bool,
            }
            let proxy = CTRL_1 {
                CH0EN: self.CH0EN(),
                CH1EN: self.CH1EN(),
                CH2EN: self.CH2EN(),
                CH3EN: self.CH3EN(),
                FSYNCEN: self.FSYNCEN(),
                DECFILS: self.DECFILS(),
                ERREN: self.ERREN(),
                DISEL: self.DISEL(),
                DBGE: self.DBGE(),
                SRES: self.SRES(),
                DBG: self.DBG(),
                PDMIEN: self.PDMIEN(),
                DOZEN: self.DOZEN(),
                MDIS: self.MDIS(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "MICFIL Control 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CTRL_2(pub u32);
    impl CTRL_2 {
        #[inline(always)]
        pub const fn CLKDIV(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_CLKDIV(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn CLKDIVDIS(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CLKDIVDIS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn CICOSR(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_CICOSR(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[inline(always)]
        pub const fn QSEL(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_QSEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 25usize)) | (((val as u32) & 0x07) << 25usize);
        }
    }
    impl Default for CTRL_2 {
        #[inline(always)]
        fn default() -> CTRL_2 {
            CTRL_2(0)
        }
    }
    impl core::fmt::Debug for CTRL_2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CTRL_2")
                .field("CLKDIV", &self.CLKDIV())
                .field("CLKDIVDIS", &self.CLKDIVDIS())
                .field("CICOSR", &self.CICOSR())
                .field("QSEL", &self.QSEL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CTRL_2 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CTRL_2 {
                CLKDIV: u8,
                CLKDIVDIS: bool,
                CICOSR: u8,
                QSEL: u8,
            }
            let proxy = CTRL_2 {
                CLKDIV: self.CLKDIV(),
                CLKDIVDIS: self.CLKDIVDIS(),
                CICOSR: self.CICOSR(),
                QSEL: self.QSEL(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "MICFIL DC Remover Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DC_CTRL(pub u32);
    impl DC_CTRL {
        #[inline(always)]
        pub const fn DCCONFIG0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_DCCONFIG0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[inline(always)]
        pub const fn DCCONFIG1(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_DCCONFIG1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[inline(always)]
        pub const fn DCCONFIG2(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_DCCONFIG2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[inline(always)]
        pub const fn DCCONFIG3(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_DCCONFIG3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
    }
    impl Default for DC_CTRL {
        #[inline(always)]
        fn default() -> DC_CTRL {
            DC_CTRL(0)
        }
    }
    impl core::fmt::Debug for DC_CTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DC_CTRL")
                .field("DCCONFIG0", &self.DCCONFIG0())
                .field("DCCONFIG1", &self.DCCONFIG1())
                .field("DCCONFIG2", &self.DCCONFIG2())
                .field("DCCONFIG3", &self.DCCONFIG3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DC_CTRL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct DC_CTRL {
                DCCONFIG0: u8,
                DCCONFIG1: u8,
                DCCONFIG2: u8,
                DCCONFIG3: u8,
            }
            let proxy = DC_CTRL {
                DCCONFIG0: self.DCCONFIG0(),
                DCCONFIG1: self.DCCONFIG1(),
                DCCONFIG2: self.DCCONFIG2(),
                DCCONFIG3: self.DCCONFIG3(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "MICFIL Output DC Remover Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DC_OUT_CTRL(pub u32);
    impl DC_OUT_CTRL {
        #[inline(always)]
        pub const fn DCCONFIG0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_DCCONFIG0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[inline(always)]
        pub const fn DCCONFIG1(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_DCCONFIG1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[inline(always)]
        pub const fn DCCONFIG2(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_DCCONFIG2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[inline(always)]
        pub const fn DCCONFIG3(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_DCCONFIG3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
    }
    impl Default for DC_OUT_CTRL {
        #[inline(always)]
        fn default() -> DC_OUT_CTRL {
            DC_OUT_CTRL(0)
        }
    }
    impl core::fmt::Debug for DC_OUT_CTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DC_OUT_CTRL")
                .field("DCCONFIG0", &self.DCCONFIG0())
                .field("DCCONFIG1", &self.DCCONFIG1())
                .field("DCCONFIG2", &self.DCCONFIG2())
                .field("DCCONFIG3", &self.DCCONFIG3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DC_OUT_CTRL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct DC_OUT_CTRL {
                DCCONFIG0: u8,
                DCCONFIG1: u8,
                DCCONFIG2: u8,
                DCCONFIG3: u8,
            }
            let proxy = DC_OUT_CTRL {
                DCCONFIG0: self.DCCONFIG0(),
                DCCONFIG1: self.DCCONFIG1(),
                DCCONFIG2: self.DCCONFIG2(),
                DCCONFIG3: self.DCCONFIG3(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "MICFIL FIFO Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FIFO_CTRL(pub u32);
    impl FIFO_CTRL {
        #[inline(always)]
        pub const fn FIFOWMK(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_FIFOWMK(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for FIFO_CTRL {
        #[inline(always)]
        fn default() -> FIFO_CTRL {
            FIFO_CTRL(0)
        }
    }
    impl core::fmt::Debug for FIFO_CTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FIFO_CTRL")
                .field("FIFOWMK", &self.FIFOWMK())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FIFO_CTRL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct FIFO_CTRL {
                FIFOWMK: u8,
            }
            let proxy = FIFO_CTRL {
                FIFOWMK: self.FIFOWMK(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "MICFIL FIFO Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FIFO_STAT(pub u32);
    impl FIFO_STAT {
        #[inline(always)]
        pub const fn FIFOOVF0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FIFOOVF0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn FIFOOVF1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FIFOOVF1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn FIFOOVF2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FIFOOVF2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn FIFOOVF3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FIFOOVF3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn FIFOUND0(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FIFOUND0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn FIFOUND1(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FIFOUND1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn FIFOUND2(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FIFOUND2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn FIFOUND3(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FIFOUND3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
    }
    impl Default for FIFO_STAT {
        #[inline(always)]
        fn default() -> FIFO_STAT {
            FIFO_STAT(0)
        }
    }
    impl core::fmt::Debug for FIFO_STAT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FIFO_STAT")
                .field("FIFOOVF0", &self.FIFOOVF0())
                .field("FIFOOVF1", &self.FIFOOVF1())
                .field("FIFOOVF2", &self.FIFOOVF2())
                .field("FIFOOVF3", &self.FIFOOVF3())
                .field("FIFOUND0", &self.FIFOUND0())
                .field("FIFOUND1", &self.FIFOUND1())
                .field("FIFOUND2", &self.FIFOUND2())
                .field("FIFOUND3", &self.FIFOUND3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FIFO_STAT {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct FIFO_STAT {
                FIFOOVF0: bool,
                FIFOOVF1: bool,
                FIFOOVF2: bool,
                FIFOOVF3: bool,
                FIFOUND0: bool,
                FIFOUND1: bool,
                FIFOUND2: bool,
                FIFOUND3: bool,
            }
            let proxy = FIFO_STAT {
                FIFOOVF0: self.FIFOOVF0(),
                FIFOOVF1: self.FIFOOVF1(),
                FIFOOVF2: self.FIFOOVF2(),
                FIFOOVF3: self.FIFOOVF3(),
                FIFOUND0: self.FIFOUND0(),
                FIFOUND1: self.FIFOUND1(),
                FIFOUND2: self.FIFOUND2(),
                FIFOUND3: self.FIFOUND3(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Parameter"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PARAM(pub u32);
    impl PARAM {
        #[inline(always)]
        pub const fn NPAIR(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_NPAIR(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn FIFO_PTRWID(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_FIFO_PTRWID(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[inline(always)]
        pub const fn FIL_OUT_WIDTH_24B(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FIL_OUT_WIDTH_24B(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn LOW_POWER(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LOW_POWER(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn DC_BYPASS(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DC_BYPASS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn DC_OUT_BYPASS(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DC_OUT_BYPASS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
    }
    impl Default for PARAM {
        #[inline(always)]
        fn default() -> PARAM {
            PARAM(0)
        }
    }
    impl core::fmt::Debug for PARAM {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PARAM")
                .field("NPAIR", &self.NPAIR())
                .field("FIFO_PTRWID", &self.FIFO_PTRWID())
                .field("FIL_OUT_WIDTH_24B", &self.FIL_OUT_WIDTH_24B())
                .field("LOW_POWER", &self.LOW_POWER())
                .field("DC_BYPASS", &self.DC_BYPASS())
                .field("DC_OUT_BYPASS", &self.DC_OUT_BYPASS())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PARAM {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct PARAM {
                NPAIR: u8,
                FIFO_PTRWID: u8,
                FIL_OUT_WIDTH_24B: bool,
                LOW_POWER: bool,
                DC_BYPASS: bool,
                DC_OUT_BYPASS: bool,
            }
            let proxy = PARAM {
                NPAIR: self.NPAIR(),
                FIFO_PTRWID: self.FIFO_PTRWID(),
                FIL_OUT_WIDTH_24B: self.FIL_OUT_WIDTH_24B(),
                LOW_POWER: self.LOW_POWER(),
                DC_BYPASS: self.DC_BYPASS(),
                DC_OUT_BYPASS: self.DC_OUT_BYPASS(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "MICFIL Range Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RANGE_CTRL(pub u32);
    impl RANGE_CTRL {
        #[inline(always)]
        pub const fn RANGEADJ0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_RANGEADJ0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn RANGEADJ1(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_RANGEADJ1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[inline(always)]
        pub const fn RANGEADJ2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_RANGEADJ2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[inline(always)]
        pub const fn RANGEADJ3(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_RANGEADJ3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
    }
    impl Default for RANGE_CTRL {
        #[inline(always)]
        fn default() -> RANGE_CTRL {
            RANGE_CTRL(0)
        }
    }
    impl core::fmt::Debug for RANGE_CTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RANGE_CTRL")
                .field("RANGEADJ0", &self.RANGEADJ0())
                .field("RANGEADJ1", &self.RANGEADJ1())
                .field("RANGEADJ2", &self.RANGEADJ2())
                .field("RANGEADJ3", &self.RANGEADJ3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RANGE_CTRL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct RANGE_CTRL {
                RANGEADJ0: u8,
                RANGEADJ1: u8,
                RANGEADJ2: u8,
                RANGEADJ3: u8,
            }
            let proxy = RANGE_CTRL {
                RANGEADJ0: self.RANGEADJ0(),
                RANGEADJ1: self.RANGEADJ1(),
                RANGEADJ2: self.RANGEADJ2(),
                RANGEADJ3: self.RANGEADJ3(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "MICFIL Range Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RANGE_STAT(pub u32);
    impl RANGE_STAT {
        #[inline(always)]
        pub const fn RANGEOVF0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RANGEOVF0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn RANGEOVF1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RANGEOVF1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn RANGEOVF2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RANGEOVF2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn RANGEOVF3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RANGEOVF3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn RANGEUNF0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RANGEUNF0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn RANGEUNF1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RANGEUNF1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn RANGEUNF2(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RANGEUNF2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn RANGEUNF3(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RANGEUNF3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
    }
    impl Default for RANGE_STAT {
        #[inline(always)]
        fn default() -> RANGE_STAT {
            RANGE_STAT(0)
        }
    }
    impl core::fmt::Debug for RANGE_STAT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RANGE_STAT")
                .field("RANGEOVF0", &self.RANGEOVF0())
                .field("RANGEOVF1", &self.RANGEOVF1())
                .field("RANGEOVF2", &self.RANGEOVF2())
                .field("RANGEOVF3", &self.RANGEOVF3())
                .field("RANGEUNF0", &self.RANGEUNF0())
                .field("RANGEUNF1", &self.RANGEUNF1())
                .field("RANGEUNF2", &self.RANGEUNF2())
                .field("RANGEUNF3", &self.RANGEUNF3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RANGE_STAT {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct RANGE_STAT {
                RANGEOVF0: bool,
                RANGEOVF1: bool,
                RANGEOVF2: bool,
                RANGEOVF3: bool,
                RANGEUNF0: bool,
                RANGEUNF1: bool,
                RANGEUNF2: bool,
                RANGEUNF3: bool,
            }
            let proxy = RANGE_STAT {
                RANGEOVF0: self.RANGEOVF0(),
                RANGEOVF1: self.RANGEOVF1(),
                RANGEOVF2: self.RANGEOVF2(),
                RANGEOVF3: self.RANGEOVF3(),
                RANGEUNF0: self.RANGEUNF0(),
                RANGEUNF1: self.RANGEUNF1(),
                RANGEUNF2: self.RANGEUNF2(),
                RANGEUNF3: self.RANGEUNF3(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "MICFIL Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct STAT(pub u32);
    impl STAT {
        #[inline(always)]
        pub const fn CH0F(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CH0F(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn CH1F(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CH1F(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn CH2F(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CH2F(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn CH3F(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CH3F(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn BSY_FIL(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BSY_FIL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for STAT {
        #[inline(always)]
        fn default() -> STAT {
            STAT(0)
        }
    }
    impl core::fmt::Debug for STAT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("STAT")
                .field("CH0F", &self.CH0F())
                .field("CH1F", &self.CH1F())
                .field("CH2F", &self.CH2F())
                .field("CH3F", &self.CH3F())
                .field("BSY_FIL", &self.BSY_FIL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for STAT {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct STAT {
                CH0F: bool,
                CH1F: bool,
                CH2F: bool,
                CH3F: bool,
                BSY_FIL: bool,
            }
            let proxy = STAT {
                CH0F: self.CH0F(),
                CH1F: self.CH1F(),
                CH2F: self.CH2F(),
                CH3F: self.CH3F(),
                BSY_FIL: self.BSY_FIL(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Version ID"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VERID(pub u32);
    impl VERID {
        #[inline(always)]
        pub const fn FEATURE(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_FEATURE(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[inline(always)]
        pub const fn MINOR(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_MINOR(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[inline(always)]
        pub const fn MAJOR(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_MAJOR(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for VERID {
        #[inline(always)]
        fn default() -> VERID {
            VERID(0)
        }
    }
    impl core::fmt::Debug for VERID {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("VERID")
                .field("FEATURE", &self.FEATURE())
                .field("MINOR", &self.MINOR())
                .field("MAJOR", &self.MAJOR())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for VERID {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct VERID {
                FEATURE: u16,
                MINOR: u8,
                MAJOR: u8,
            }
            let proxy = VERID {
                FEATURE: self.FEATURE(),
                MINOR: self.MINOR(),
                MAJOR: self.MAJOR(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
}
