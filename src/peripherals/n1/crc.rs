#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (d5ec99b 2024-12-16))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CRC {
    ptr: *mut u8,
}
unsafe impl Send for CRC {}
unsafe impl Sync for CRC {}
impl CRC {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn DATA(self) -> crate::common::Reg<regs::DATA, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn DATAL(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn DATALL(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn DATALU(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01usize) as _) }
    }
    #[inline(always)]
    pub const fn DATAH(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02usize) as _) }
    }
    #[inline(always)]
    pub const fn DATAHL(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02usize) as _) }
    }
    #[inline(always)]
    pub const fn DATAHU(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03usize) as _) }
    }
    #[inline(always)]
    pub const fn GPOLY(self) -> crate::common::Reg<regs::GPOLY, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn GPOLYL(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn GPOLYLL(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn GPOLYLU(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05usize) as _) }
    }
    #[inline(always)]
    pub const fn GPOLYH(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06usize) as _) }
    }
    #[inline(always)]
    pub const fn GPOLYHL(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06usize) as _) }
    }
    #[inline(always)]
    pub const fn GPOLYHU(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07usize) as _) }
    }
    #[inline(always)]
    pub const fn CTRL(self) -> crate::common::Reg<regs::CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn CTRLHU(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0busize) as _) }
    }
}
pub mod regs {
    #[doc = "CRC_DATAH register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ACCESS16BIT_DATAH(pub u32);
    impl ACCESS16BIT_DATAH {
        #[inline(always)]
        pub const fn DATAH(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_DATAH(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for ACCESS16BIT_DATAH {
        #[inline(always)]
        fn default() -> ACCESS16BIT_DATAH {
            ACCESS16BIT_DATAH(0)
        }
    }
    impl core::fmt::Debug for ACCESS16BIT_DATAH {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ACCESS16BIT_DATAH")
                .field("DATAH", &self.DATAH())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ACCESS16BIT_DATAH {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ACCESS16BIT_DATAH {
                DATAH: u16,
            }
            let proxy = ACCESS16BIT_DATAH {
                DATAH: self.DATAH(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "CRC_DATAL register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ACCESS16BIT_DATAL(pub u32);
    impl ACCESS16BIT_DATAL {
        #[inline(always)]
        pub const fn DATAL(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_DATAL(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for ACCESS16BIT_DATAL {
        #[inline(always)]
        fn default() -> ACCESS16BIT_DATAL {
            ACCESS16BIT_DATAL(0)
        }
    }
    impl core::fmt::Debug for ACCESS16BIT_DATAL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ACCESS16BIT_DATAL")
                .field("DATAL", &self.DATAL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ACCESS16BIT_DATAL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ACCESS16BIT_DATAL {
                DATAL: u16,
            }
            let proxy = ACCESS16BIT_DATAL {
                DATAL: self.DATAL(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "CRC_DATAHL register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ACCESS8BIT_DATAHL(pub u32);
    impl ACCESS8BIT_DATAHL {
        #[inline(always)]
        pub const fn DATAHL(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATAHL(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for ACCESS8BIT_DATAHL {
        #[inline(always)]
        fn default() -> ACCESS8BIT_DATAHL {
            ACCESS8BIT_DATAHL(0)
        }
    }
    impl core::fmt::Debug for ACCESS8BIT_DATAHL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ACCESS8BIT_DATAHL")
                .field("DATAHL", &self.DATAHL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ACCESS8BIT_DATAHL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ACCESS8BIT_DATAHL {
                DATAHL: u8,
            }
            let proxy = ACCESS8BIT_DATAHL {
                DATAHL: self.DATAHL(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "CRC_DATAHU register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ACCESS8BIT_DATAHU(pub u32);
    impl ACCESS8BIT_DATAHU {
        #[inline(always)]
        pub const fn DATAHU(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATAHU(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for ACCESS8BIT_DATAHU {
        #[inline(always)]
        fn default() -> ACCESS8BIT_DATAHU {
            ACCESS8BIT_DATAHU(0)
        }
    }
    impl core::fmt::Debug for ACCESS8BIT_DATAHU {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ACCESS8BIT_DATAHU")
                .field("DATAHU", &self.DATAHU())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ACCESS8BIT_DATAHU {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ACCESS8BIT_DATAHU {
                DATAHU: u8,
            }
            let proxy = ACCESS8BIT_DATAHU {
                DATAHU: self.DATAHU(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "CRC_DATALL register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ACCESS8BIT_DATALL(pub u32);
    impl ACCESS8BIT_DATALL {
        #[inline(always)]
        pub const fn DATALL(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATALL(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for ACCESS8BIT_DATALL {
        #[inline(always)]
        fn default() -> ACCESS8BIT_DATALL {
            ACCESS8BIT_DATALL(0)
        }
    }
    impl core::fmt::Debug for ACCESS8BIT_DATALL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ACCESS8BIT_DATALL")
                .field("DATALL", &self.DATALL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ACCESS8BIT_DATALL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ACCESS8BIT_DATALL {
                DATALL: u8,
            }
            let proxy = ACCESS8BIT_DATALL {
                DATALL: self.DATALL(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "CRC_DATALU register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ACCESS8BIT_DATALU(pub u32);
    impl ACCESS8BIT_DATALU {
        #[inline(always)]
        pub const fn DATALU(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATALU(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for ACCESS8BIT_DATALU {
        #[inline(always)]
        fn default() -> ACCESS8BIT_DATALU {
            ACCESS8BIT_DATALU(0)
        }
    }
    impl core::fmt::Debug for ACCESS8BIT_DATALU {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ACCESS8BIT_DATALU")
                .field("DATALU", &self.DATALU())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ACCESS8BIT_DATALU {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ACCESS8BIT_DATALU {
                DATALU: u8,
            }
            let proxy = ACCESS8BIT_DATALU {
                DATALU: self.DATALU(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CTRL(pub u32);
    impl CTRL {
        #[inline(always)]
        pub const fn TCRC(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TCRC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn WAS(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WAS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn FXOR(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FXOR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn TOTR(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_TOTR(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
        #[inline(always)]
        pub const fn TOT(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_TOT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for CTRL {
        #[inline(always)]
        fn default() -> CTRL {
            CTRL(0)
        }
    }
    impl core::fmt::Debug for CTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CTRL")
                .field("TCRC", &self.TCRC())
                .field("WAS", &self.WAS())
                .field("FXOR", &self.FXOR())
                .field("TOTR", &self.TOTR())
                .field("TOT", &self.TOT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CTRL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CTRL {
                TCRC: bool,
                WAS: bool,
                FXOR: bool,
                TOTR: u8,
                TOT: u8,
            }
            let proxy = CTRL {
                TCRC: self.TCRC(),
                WAS: self.WAS(),
                FXOR: self.FXOR(),
                TOTR: self.TOTR(),
                TOT: self.TOT(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "CRC_CTRLHU register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CTRL_ACCESS8BIT_CTRLHU(pub u32);
    impl CTRL_ACCESS8BIT_CTRLHU {
        #[inline(always)]
        pub const fn TCRC(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TCRC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn WAS(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WAS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn FXOR(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FXOR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn TOTR(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_TOTR(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[inline(always)]
        pub const fn TOT(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_TOT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
    }
    impl Default for CTRL_ACCESS8BIT_CTRLHU {
        #[inline(always)]
        fn default() -> CTRL_ACCESS8BIT_CTRLHU {
            CTRL_ACCESS8BIT_CTRLHU(0)
        }
    }
    impl core::fmt::Debug for CTRL_ACCESS8BIT_CTRLHU {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CTRL_ACCESS8BIT_CTRLHU")
                .field("TCRC", &self.TCRC())
                .field("WAS", &self.WAS())
                .field("FXOR", &self.FXOR())
                .field("TOTR", &self.TOTR())
                .field("TOT", &self.TOT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CTRL_ACCESS8BIT_CTRLHU {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CTRL_ACCESS8BIT_CTRLHU {
                TCRC: bool,
                WAS: bool,
                FXOR: bool,
                TOTR: u8,
                TOT: u8,
            }
            let proxy = CTRL_ACCESS8BIT_CTRLHU {
                TCRC: self.TCRC(),
                WAS: self.WAS(),
                FXOR: self.FXOR(),
                TOTR: self.TOTR(),
                TOT: self.TOT(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Data"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DATA(pub u32);
    impl DATA {
        #[inline(always)]
        pub const fn LL(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_LL(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn LU(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_LU(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[inline(always)]
        pub const fn HL(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_HL(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[inline(always)]
        pub const fn HU(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_HU(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for DATA {
        #[inline(always)]
        fn default() -> DATA {
            DATA(0)
        }
    }
    impl core::fmt::Debug for DATA {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DATA")
                .field("LL", &self.LL())
                .field("LU", &self.LU())
                .field("HL", &self.HL())
                .field("HU", &self.HU())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DATA {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct DATA {
                LL: u8,
                LU: u8,
                HL: u8,
                HU: u8,
            }
            let proxy = DATA {
                LL: self.LL(),
                LU: self.LU(),
                HL: self.HL(),
                HU: self.HU(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Polynomial"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GPOLY(pub u32);
    impl GPOLY {
        #[inline(always)]
        pub const fn LOW(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_LOW(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[inline(always)]
        pub const fn HIGH(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_HIGH(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for GPOLY {
        #[inline(always)]
        fn default() -> GPOLY {
            GPOLY(0)
        }
    }
    impl core::fmt::Debug for GPOLY {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GPOLY")
                .field("LOW", &self.LOW())
                .field("HIGH", &self.HIGH())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GPOLY {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct GPOLY {
                LOW: u16,
                HIGH: u16,
            }
            let proxy = GPOLY {
                LOW: self.LOW(),
                HIGH: self.HIGH(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "CRC_GPOLYH register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GPOLY_ACCESS16BIT_GPOLYH(pub u32);
    impl GPOLY_ACCESS16BIT_GPOLYH {
        #[inline(always)]
        pub const fn GPOLYH(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_GPOLYH(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for GPOLY_ACCESS16BIT_GPOLYH {
        #[inline(always)]
        fn default() -> GPOLY_ACCESS16BIT_GPOLYH {
            GPOLY_ACCESS16BIT_GPOLYH(0)
        }
    }
    impl core::fmt::Debug for GPOLY_ACCESS16BIT_GPOLYH {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GPOLY_ACCESS16BIT_GPOLYH")
                .field("GPOLYH", &self.GPOLYH())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GPOLY_ACCESS16BIT_GPOLYH {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct GPOLY_ACCESS16BIT_GPOLYH {
                GPOLYH: u16,
            }
            let proxy = GPOLY_ACCESS16BIT_GPOLYH {
                GPOLYH: self.GPOLYH(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "CRC_GPOLYL register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GPOLY_ACCESS16BIT_GPOLYL(pub u32);
    impl GPOLY_ACCESS16BIT_GPOLYL {
        #[inline(always)]
        pub const fn GPOLYL(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_GPOLYL(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for GPOLY_ACCESS16BIT_GPOLYL {
        #[inline(always)]
        fn default() -> GPOLY_ACCESS16BIT_GPOLYL {
            GPOLY_ACCESS16BIT_GPOLYL(0)
        }
    }
    impl core::fmt::Debug for GPOLY_ACCESS16BIT_GPOLYL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GPOLY_ACCESS16BIT_GPOLYL")
                .field("GPOLYL", &self.GPOLYL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GPOLY_ACCESS16BIT_GPOLYL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct GPOLY_ACCESS16BIT_GPOLYL {
                GPOLYL: u16,
            }
            let proxy = GPOLY_ACCESS16BIT_GPOLYL {
                GPOLYL: self.GPOLYL(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "CRC_GPOLYHL register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GPOLY_ACCESS8BIT_GPOLYHL(pub u32);
    impl GPOLY_ACCESS8BIT_GPOLYHL {
        #[inline(always)]
        pub const fn GPOLYHL(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_GPOLYHL(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for GPOLY_ACCESS8BIT_GPOLYHL {
        #[inline(always)]
        fn default() -> GPOLY_ACCESS8BIT_GPOLYHL {
            GPOLY_ACCESS8BIT_GPOLYHL(0)
        }
    }
    impl core::fmt::Debug for GPOLY_ACCESS8BIT_GPOLYHL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GPOLY_ACCESS8BIT_GPOLYHL")
                .field("GPOLYHL", &self.GPOLYHL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GPOLY_ACCESS8BIT_GPOLYHL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct GPOLY_ACCESS8BIT_GPOLYHL {
                GPOLYHL: u8,
            }
            let proxy = GPOLY_ACCESS8BIT_GPOLYHL {
                GPOLYHL: self.GPOLYHL(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "CRC_GPOLYHU register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GPOLY_ACCESS8BIT_GPOLYHU(pub u32);
    impl GPOLY_ACCESS8BIT_GPOLYHU {
        #[inline(always)]
        pub const fn GPOLYHU(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_GPOLYHU(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for GPOLY_ACCESS8BIT_GPOLYHU {
        #[inline(always)]
        fn default() -> GPOLY_ACCESS8BIT_GPOLYHU {
            GPOLY_ACCESS8BIT_GPOLYHU(0)
        }
    }
    impl core::fmt::Debug for GPOLY_ACCESS8BIT_GPOLYHU {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GPOLY_ACCESS8BIT_GPOLYHU")
                .field("GPOLYHU", &self.GPOLYHU())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GPOLY_ACCESS8BIT_GPOLYHU {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct GPOLY_ACCESS8BIT_GPOLYHU {
                GPOLYHU: u8,
            }
            let proxy = GPOLY_ACCESS8BIT_GPOLYHU {
                GPOLYHU: self.GPOLYHU(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "CRC_GPOLYLL register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GPOLY_ACCESS8BIT_GPOLYLL(pub u32);
    impl GPOLY_ACCESS8BIT_GPOLYLL {
        #[inline(always)]
        pub const fn GPOLYLL(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_GPOLYLL(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for GPOLY_ACCESS8BIT_GPOLYLL {
        #[inline(always)]
        fn default() -> GPOLY_ACCESS8BIT_GPOLYLL {
            GPOLY_ACCESS8BIT_GPOLYLL(0)
        }
    }
    impl core::fmt::Debug for GPOLY_ACCESS8BIT_GPOLYLL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GPOLY_ACCESS8BIT_GPOLYLL")
                .field("GPOLYLL", &self.GPOLYLL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GPOLY_ACCESS8BIT_GPOLYLL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct GPOLY_ACCESS8BIT_GPOLYLL {
                GPOLYLL: u8,
            }
            let proxy = GPOLY_ACCESS8BIT_GPOLYLL {
                GPOLYLL: self.GPOLYLL(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "CRC_GPOLYLU register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GPOLY_ACCESS8BIT_GPOLYLU(pub u32);
    impl GPOLY_ACCESS8BIT_GPOLYLU {
        #[inline(always)]
        pub const fn GPOLYLU(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_GPOLYLU(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for GPOLY_ACCESS8BIT_GPOLYLU {
        #[inline(always)]
        fn default() -> GPOLY_ACCESS8BIT_GPOLYLU {
            GPOLY_ACCESS8BIT_GPOLYLU(0)
        }
    }
    impl core::fmt::Debug for GPOLY_ACCESS8BIT_GPOLYLU {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GPOLY_ACCESS8BIT_GPOLYLU")
                .field("GPOLYLU", &self.GPOLYLU())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GPOLY_ACCESS8BIT_GPOLYLU {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct GPOLY_ACCESS8BIT_GPOLYLU {
                GPOLYLU: u8,
            }
            let proxy = GPOLY_ACCESS8BIT_GPOLYLU {
                GPOLYLU: self.GPOLYLU(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
}
