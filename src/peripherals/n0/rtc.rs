#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (0303941 2025-02-18))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RTC {
    ptr: *mut u8,
}
unsafe impl Send for RTC {}
unsafe impl Sync for RTC {}
impl RTC {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn YEARMON(self) -> crate::common::Reg<regs::YEARMON, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn DAYS(self) -> crate::common::Reg<regs::DAYS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02usize) as _) }
    }
    #[inline(always)]
    pub const fn HOURMIN(self) -> crate::common::Reg<regs::HOURMIN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn SECONDS(self) -> crate::common::Reg<regs::SECONDS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06usize) as _) }
    }
    #[inline(always)]
    pub const fn ALM_YEARMON(self) -> crate::common::Reg<regs::ALM_YEARMON, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn ALM_DAYS(self) -> crate::common::Reg<regs::ALM_DAYS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ausize) as _) }
    }
    #[inline(always)]
    pub const fn ALM_HOURMIN(self) -> crate::common::Reg<regs::ALM_HOURMIN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn ALM_SECONDS(self) -> crate::common::Reg<regs::ALM_SECONDS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0eusize) as _) }
    }
    #[inline(always)]
    pub const fn CTRL(self) -> crate::common::Reg<regs::CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn STATUS(self) -> crate::common::Reg<regs::STATUS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x12usize) as _) }
    }
    #[inline(always)]
    pub const fn ISR(self) -> crate::common::Reg<regs::ISR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn IER(self) -> crate::common::Reg<regs::IER, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x16usize) as _) }
    }
    #[inline(always)]
    pub const fn RTC_TEST2(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[inline(always)]
    pub const fn DST_HOUR(self) -> crate::common::Reg<regs::DST_HOUR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x22usize) as _) }
    }
    #[inline(always)]
    pub const fn DST_MONTH(self) -> crate::common::Reg<regs::DST_MONTH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[inline(always)]
    pub const fn DST_DAY(self) -> crate::common::Reg<regs::DST_DAY, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x26usize) as _) }
    }
    #[inline(always)]
    pub const fn COMPEN(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[inline(always)]
    pub const fn SUBSECOND_CTRL(
        self,
    ) -> crate::common::Reg<regs::SUBSECOND_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0800usize) as _) }
    }
    #[inline(always)]
    pub const fn SUBSECOND_CNT(self) -> crate::common::Reg<regs::SUBSECOND_CNT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0804usize) as _) }
    }
    #[inline(always)]
    pub const fn WAKE_TIMER_CTRL(
        self,
    ) -> crate::common::Reg<regs::WAKE_TIMER_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0c00usize) as _) }
    }
    #[inline(always)]
    pub const fn WAKE_TIMER_CNT(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0c0cusize) as _) }
    }
}
pub mod regs {
    #[doc = "Days Alarm"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ALM_DAYS(pub u16);
    impl ALM_DAYS {
        #[inline(always)]
        pub const fn ALM_DAY(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_ALM_DAY(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u16) & 0x1f) << 0usize);
        }
    }
    impl Default for ALM_DAYS {
        #[inline(always)]
        fn default() -> ALM_DAYS {
            ALM_DAYS(0)
        }
    }
    impl core::fmt::Debug for ALM_DAYS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ALM_DAYS")
                .field("ALM_DAY", &self.ALM_DAY())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ALM_DAYS {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ALM_DAYS {{ ALM_DAY: {=u8:?} }}", self.ALM_DAY())
        }
    }
    #[doc = "Hours and Minutes Alarm"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ALM_HOURMIN(pub u16);
    impl ALM_HOURMIN {
        #[inline(always)]
        pub const fn ALM_MIN(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_ALM_MIN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u16) & 0x3f) << 0usize);
        }
        #[inline(always)]
        pub const fn ALM_HOUR(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_ALM_HOUR(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u16) & 0x1f) << 8usize);
        }
    }
    impl Default for ALM_HOURMIN {
        #[inline(always)]
        fn default() -> ALM_HOURMIN {
            ALM_HOURMIN(0)
        }
    }
    impl core::fmt::Debug for ALM_HOURMIN {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ALM_HOURMIN")
                .field("ALM_MIN", &self.ALM_MIN())
                .field("ALM_HOUR", &self.ALM_HOUR())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ALM_HOURMIN {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ALM_HOURMIN {{ ALM_MIN: {=u8:?}, ALM_HOUR: {=u8:?} }}",
                self.ALM_MIN(),
                self.ALM_HOUR()
            )
        }
    }
    #[doc = "Seconds Alarm"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ALM_SECONDS(pub u16);
    impl ALM_SECONDS {
        #[inline(always)]
        pub const fn ALM_SEC(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_ALM_SEC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u16) & 0x3f) << 0usize);
        }
        #[inline(always)]
        pub const fn DEC_SEC(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DEC_SEC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn INC_SEC(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INC_SEC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
    }
    impl Default for ALM_SECONDS {
        #[inline(always)]
        fn default() -> ALM_SECONDS {
            ALM_SECONDS(0)
        }
    }
    impl core::fmt::Debug for ALM_SECONDS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ALM_SECONDS")
                .field("ALM_SEC", &self.ALM_SEC())
                .field("DEC_SEC", &self.DEC_SEC())
                .field("INC_SEC", &self.INC_SEC())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ALM_SECONDS {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ALM_SECONDS {{ ALM_SEC: {=u8:?}, DEC_SEC: {=bool:?}, INC_SEC: {=bool:?} }}",
                self.ALM_SEC(),
                self.DEC_SEC(),
                self.INC_SEC()
            )
        }
    }
    #[doc = "Year and Months Alarm"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ALM_YEARMON(pub u16);
    impl ALM_YEARMON {
        #[inline(always)]
        pub const fn ALM_MON(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_ALM_MON(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn ALM_YEAR(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_ALM_YEAR(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
        }
    }
    impl Default for ALM_YEARMON {
        #[inline(always)]
        fn default() -> ALM_YEARMON {
            ALM_YEARMON(0)
        }
    }
    impl core::fmt::Debug for ALM_YEARMON {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ALM_YEARMON")
                .field("ALM_MON", &self.ALM_MON())
                .field("ALM_YEAR", &self.ALM_YEAR())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ALM_YEARMON {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ALM_YEARMON {{ ALM_MON: {=u8:?}, ALM_YEAR: {=u8:?} }}",
                self.ALM_MON(),
                self.ALM_YEAR()
            )
        }
    }
    #[doc = "Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CTRL(pub u16);
    impl CTRL {
        #[inline(always)]
        pub const fn FINEEN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FINEEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn COMP_EN(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_COMP_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn ALM_MATCH(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_ALM_MATCH(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u16) & 0x03) << 2usize);
        }
        #[inline(always)]
        pub const fn DST_EN(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DST_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn SWR(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SWR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn CLK_SEL(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CLK_SEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn CLKO_DIS(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CLKO_DIS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn CLKOUT(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_CLKOUT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 13usize)) | (((val as u16) & 0x03) << 13usize);
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
                .field("FINEEN", &self.FINEEN())
                .field("COMP_EN", &self.COMP_EN())
                .field("ALM_MATCH", &self.ALM_MATCH())
                .field("DST_EN", &self.DST_EN())
                .field("SWR", &self.SWR())
                .field("CLK_SEL", &self.CLK_SEL())
                .field("CLKO_DIS", &self.CLKO_DIS())
                .field("CLKOUT", &self.CLKOUT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CTRL {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "CTRL {{ FINEEN: {=bool:?}, COMP_EN: {=bool:?}, ALM_MATCH: {=u8:?}, DST_EN: {=bool:?}, SWR: {=bool:?}, CLK_SEL: {=bool:?}, CLKO_DIS: {=bool:?}, CLKOUT: {=u8:?} }}" , self . FINEEN () , self . COMP_EN () , self . ALM_MATCH () , self . DST_EN () , self . SWR () , self . CLK_SEL () , self . CLKO_DIS () , self . CLKOUT ())
        }
    }
    #[doc = "Days and Day-of-Week Counters"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DAYS(pub u16);
    impl DAYS {
        #[inline(always)]
        pub const fn DAY_CNT(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_DAY_CNT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u16) & 0x1f) << 0usize);
        }
        #[inline(always)]
        pub const fn DOW(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_DOW(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u16) & 0x07) << 8usize);
        }
    }
    impl Default for DAYS {
        #[inline(always)]
        fn default() -> DAYS {
            DAYS(0)
        }
    }
    impl core::fmt::Debug for DAYS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DAYS")
                .field("DAY_CNT", &self.DAY_CNT())
                .field("DOW", &self.DOW())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DAYS {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DAYS {{ DAY_CNT: {=u8:?}, DOW: {=u8:?} }}",
                self.DAY_CNT(),
                self.DOW()
            )
        }
    }
    #[doc = "Daylight Saving Day"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DST_DAY(pub u16);
    impl DST_DAY {
        #[inline(always)]
        pub const fn DST_END_DAY(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_DST_END_DAY(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u16) & 0x1f) << 0usize);
        }
        #[inline(always)]
        pub const fn DST_START_DAY(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_DST_START_DAY(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u16) & 0x1f) << 8usize);
        }
    }
    impl Default for DST_DAY {
        #[inline(always)]
        fn default() -> DST_DAY {
            DST_DAY(0)
        }
    }
    impl core::fmt::Debug for DST_DAY {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DST_DAY")
                .field("DST_END_DAY", &self.DST_END_DAY())
                .field("DST_START_DAY", &self.DST_START_DAY())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DST_DAY {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DST_DAY {{ DST_END_DAY: {=u8:?}, DST_START_DAY: {=u8:?} }}",
                self.DST_END_DAY(),
                self.DST_START_DAY()
            )
        }
    }
    #[doc = "Daylight Saving Hour"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DST_HOUR(pub u16);
    impl DST_HOUR {
        #[inline(always)]
        pub const fn DST_END_HOUR(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_DST_END_HOUR(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u16) & 0x1f) << 0usize);
        }
        #[inline(always)]
        pub const fn DST_START_HOUR(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_DST_START_HOUR(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u16) & 0x1f) << 8usize);
        }
    }
    impl Default for DST_HOUR {
        #[inline(always)]
        fn default() -> DST_HOUR {
            DST_HOUR(0)
        }
    }
    impl core::fmt::Debug for DST_HOUR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DST_HOUR")
                .field("DST_END_HOUR", &self.DST_END_HOUR())
                .field("DST_START_HOUR", &self.DST_START_HOUR())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DST_HOUR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DST_HOUR {{ DST_END_HOUR: {=u8:?}, DST_START_HOUR: {=u8:?} }}",
                self.DST_END_HOUR(),
                self.DST_START_HOUR()
            )
        }
    }
    #[doc = "Daylight Saving Month"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DST_MONTH(pub u16);
    impl DST_MONTH {
        #[inline(always)]
        pub const fn DST_END_MONTH(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_DST_END_MONTH(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn DST_START_MONTH(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_DST_START_MONTH(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u16) & 0x0f) << 8usize);
        }
    }
    impl Default for DST_MONTH {
        #[inline(always)]
        fn default() -> DST_MONTH {
            DST_MONTH(0)
        }
    }
    impl core::fmt::Debug for DST_MONTH {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DST_MONTH")
                .field("DST_END_MONTH", &self.DST_END_MONTH())
                .field("DST_START_MONTH", &self.DST_START_MONTH())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DST_MONTH {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DST_MONTH {{ DST_END_MONTH: {=u8:?}, DST_START_MONTH: {=u8:?} }}",
                self.DST_END_MONTH(),
                self.DST_START_MONTH()
            )
        }
    }
    #[doc = "Hours and Minutes Counters"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HOURMIN(pub u16);
    impl HOURMIN {
        #[inline(always)]
        pub const fn MIN_CNT(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_MIN_CNT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u16) & 0x3f) << 0usize);
        }
        #[inline(always)]
        pub const fn HOUR_CNT(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_HOUR_CNT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u16) & 0x1f) << 8usize);
        }
    }
    impl Default for HOURMIN {
        #[inline(always)]
        fn default() -> HOURMIN {
            HOURMIN(0)
        }
    }
    impl core::fmt::Debug for HOURMIN {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HOURMIN")
                .field("MIN_CNT", &self.MIN_CNT())
                .field("HOUR_CNT", &self.HOUR_CNT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HOURMIN {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "HOURMIN {{ MIN_CNT: {=u8:?}, HOUR_CNT: {=u8:?} }}",
                self.MIN_CNT(),
                self.HOUR_CNT()
            )
        }
    }
    #[doc = "Interrupt Enable"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IER(pub u16);
    impl IER {
        #[inline(always)]
        pub const fn ALM_IE(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ALM_IE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn DAY_IE(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DAY_IE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn HOUR_IE(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HOUR_IE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn MIN_IE(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_MIN_IE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn IE_1HZ(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IE_1HZ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn IE_2HZ(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IE_2HZ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn IE_4HZ(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IE_4HZ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn IE_8HZ(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IE_8HZ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn IE_16HZ(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IE_16HZ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn IE_32HZ(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IE_32HZ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn IE_64HZ(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IE_64HZ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn IE_128HZ(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IE_128HZ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn IE_256HZ(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IE_256HZ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn IE_512HZ(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IE_512HZ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for IER {
        #[inline(always)]
        fn default() -> IER {
            IER(0)
        }
    }
    impl core::fmt::Debug for IER {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IER")
                .field("ALM_IE", &self.ALM_IE())
                .field("DAY_IE", &self.DAY_IE())
                .field("HOUR_IE", &self.HOUR_IE())
                .field("MIN_IE", &self.MIN_IE())
                .field("IE_1HZ", &self.IE_1HZ())
                .field("IE_2HZ", &self.IE_2HZ())
                .field("IE_4HZ", &self.IE_4HZ())
                .field("IE_8HZ", &self.IE_8HZ())
                .field("IE_16HZ", &self.IE_16HZ())
                .field("IE_32HZ", &self.IE_32HZ())
                .field("IE_64HZ", &self.IE_64HZ())
                .field("IE_128HZ", &self.IE_128HZ())
                .field("IE_256HZ", &self.IE_256HZ())
                .field("IE_512HZ", &self.IE_512HZ())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IER {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "IER {{ ALM_IE: {=bool:?}, DAY_IE: {=bool:?}, HOUR_IE: {=bool:?}, MIN_IE: {=bool:?}, IE_1HZ: {=bool:?}, IE_2HZ: {=bool:?}, IE_4HZ: {=bool:?}, IE_8HZ: {=bool:?}, IE_16HZ: {=bool:?}, IE_32HZ: {=bool:?}, IE_64HZ: {=bool:?}, IE_128HZ: {=bool:?}, IE_256HZ: {=bool:?}, IE_512HZ: {=bool:?} }}" , self . ALM_IE () , self . DAY_IE () , self . HOUR_IE () , self . MIN_IE () , self . IE_1HZ () , self . IE_2HZ () , self . IE_4HZ () , self . IE_8HZ () , self . IE_16HZ () , self . IE_32HZ () , self . IE_64HZ () , self . IE_128HZ () , self . IE_256HZ () , self . IE_512HZ ())
        }
    }
    #[doc = "Interrupt Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ISR(pub u16);
    impl ISR {
        #[inline(always)]
        pub const fn ALM_IS(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ALM_IS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn DAY_IS(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DAY_IS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn HOUR_IS(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HOUR_IS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn MIN_IS(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_MIN_IS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn IS_1HZ(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IS_1HZ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn IS_2HZ(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IS_2HZ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn IS_4HZ(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IS_4HZ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn IS_8HZ(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IS_8HZ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn IS_16HZ(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IS_16HZ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn IS_32HZ(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IS_32HZ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn IS_64HZ(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IS_64HZ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn IS_128HZ(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IS_128HZ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn IS_256HZ(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IS_256HZ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn IS_512HZ(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IS_512HZ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for ISR {
        #[inline(always)]
        fn default() -> ISR {
            ISR(0)
        }
    }
    impl core::fmt::Debug for ISR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ISR")
                .field("ALM_IS", &self.ALM_IS())
                .field("DAY_IS", &self.DAY_IS())
                .field("HOUR_IS", &self.HOUR_IS())
                .field("MIN_IS", &self.MIN_IS())
                .field("IS_1HZ", &self.IS_1HZ())
                .field("IS_2HZ", &self.IS_2HZ())
                .field("IS_4HZ", &self.IS_4HZ())
                .field("IS_8HZ", &self.IS_8HZ())
                .field("IS_16HZ", &self.IS_16HZ())
                .field("IS_32HZ", &self.IS_32HZ())
                .field("IS_64HZ", &self.IS_64HZ())
                .field("IS_128HZ", &self.IS_128HZ())
                .field("IS_256HZ", &self.IS_256HZ())
                .field("IS_512HZ", &self.IS_512HZ())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ISR {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "ISR {{ ALM_IS: {=bool:?}, DAY_IS: {=bool:?}, HOUR_IS: {=bool:?}, MIN_IS: {=bool:?}, IS_1HZ: {=bool:?}, IS_2HZ: {=bool:?}, IS_4HZ: {=bool:?}, IS_8HZ: {=bool:?}, IS_16HZ: {=bool:?}, IS_32HZ: {=bool:?}, IS_64HZ: {=bool:?}, IS_128HZ: {=bool:?}, IS_256HZ: {=bool:?}, IS_512HZ: {=bool:?} }}" , self . ALM_IS () , self . DAY_IS () , self . HOUR_IS () , self . MIN_IS () , self . IS_1HZ () , self . IS_2HZ () , self . IS_4HZ () , self . IS_8HZ () , self . IS_16HZ () , self . IS_32HZ () , self . IS_64HZ () , self . IS_128HZ () , self . IS_256HZ () , self . IS_512HZ ())
        }
    }
    #[doc = "Seconds Counters"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SECONDS(pub u16);
    impl SECONDS {
        #[inline(always)]
        pub const fn SEC_CNT(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_SEC_CNT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u16) & 0x3f) << 0usize);
        }
    }
    impl Default for SECONDS {
        #[inline(always)]
        fn default() -> SECONDS {
            SECONDS(0)
        }
    }
    impl core::fmt::Debug for SECONDS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SECONDS")
                .field("SEC_CNT", &self.SEC_CNT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SECONDS {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SECONDS {{ SEC_CNT: {=u8:?} }}", self.SEC_CNT())
        }
    }
    #[doc = "Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct STATUS(pub u16);
    impl STATUS {
        #[inline(always)]
        pub const fn INVAL_BIT(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INVAL_BIT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn WRITE_PROT_EN(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WRITE_PROT_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn CMP_INT(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CMP_INT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn WE(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_WE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u16) & 0x03) << 6usize);
        }
        #[inline(always)]
        pub const fn BUS_ERR(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BUS_ERR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn CMP_DONE(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CMP_DONE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
    }
    impl Default for STATUS {
        #[inline(always)]
        fn default() -> STATUS {
            STATUS(0)
        }
    }
    impl core::fmt::Debug for STATUS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("STATUS")
                .field("INVAL_BIT", &self.INVAL_BIT())
                .field("WRITE_PROT_EN", &self.WRITE_PROT_EN())
                .field("CMP_INT", &self.CMP_INT())
                .field("WE", &self.WE())
                .field("BUS_ERR", &self.BUS_ERR())
                .field("CMP_DONE", &self.CMP_DONE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for STATUS {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "STATUS {{ INVAL_BIT: {=bool:?}, WRITE_PROT_EN: {=bool:?}, CMP_INT: {=bool:?}, WE: {=u8:?}, BUS_ERR: {=bool:?}, CMP_DONE: {=bool:?} }}" , self . INVAL_BIT () , self . WRITE_PROT_EN () , self . CMP_INT () , self . WE () , self . BUS_ERR () , self . CMP_DONE ())
        }
    }
    #[doc = "Subsecond Counter"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SUBSECOND_CNT(pub u32);
    impl SUBSECOND_CNT {
        #[inline(always)]
        pub const fn SUBSECOND_CNT(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_SUBSECOND_CNT(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for SUBSECOND_CNT {
        #[inline(always)]
        fn default() -> SUBSECOND_CNT {
            SUBSECOND_CNT(0)
        }
    }
    impl core::fmt::Debug for SUBSECOND_CNT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SUBSECOND_CNT")
                .field("SUBSECOND_CNT", &self.SUBSECOND_CNT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SUBSECOND_CNT {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SUBSECOND_CNT {{ SUBSECOND_CNT: {=u16:?} }}",
                self.SUBSECOND_CNT()
            )
        }
    }
    #[doc = "Subsecond Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SUBSECOND_CTRL(pub u32);
    impl SUBSECOND_CTRL {
        #[inline(always)]
        pub const fn SUB_SECOND_CNT_EN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SUB_SECOND_CNT_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for SUBSECOND_CTRL {
        #[inline(always)]
        fn default() -> SUBSECOND_CTRL {
            SUBSECOND_CTRL(0)
        }
    }
    impl core::fmt::Debug for SUBSECOND_CTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SUBSECOND_CTRL")
                .field("SUB_SECOND_CNT_EN", &self.SUB_SECOND_CNT_EN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SUBSECOND_CTRL {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SUBSECOND_CTRL {{ SUB_SECOND_CNT_EN: {=bool:?} }}",
                self.SUB_SECOND_CNT_EN()
            )
        }
    }
    #[doc = "Wake Timer Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WAKE_TIMER_CTRL(pub u32);
    impl WAKE_TIMER_CTRL {
        #[inline(always)]
        pub const fn WAKE_FLAG(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WAKE_FLAG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn CLR_WAKE_TIMER(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CLR_WAKE_TIMER(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn OSC_DIV_ENA(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OSC_DIV_ENA(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn INTR_EN(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INTR_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for WAKE_TIMER_CTRL {
        #[inline(always)]
        fn default() -> WAKE_TIMER_CTRL {
            WAKE_TIMER_CTRL(0)
        }
    }
    impl core::fmt::Debug for WAKE_TIMER_CTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("WAKE_TIMER_CTRL")
                .field("WAKE_FLAG", &self.WAKE_FLAG())
                .field("CLR_WAKE_TIMER", &self.CLR_WAKE_TIMER())
                .field("OSC_DIV_ENA", &self.OSC_DIV_ENA())
                .field("INTR_EN", &self.INTR_EN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for WAKE_TIMER_CTRL {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "WAKE_TIMER_CTRL {{ WAKE_FLAG: {=bool:?}, CLR_WAKE_TIMER: {=bool:?}, OSC_DIV_ENA: {=bool:?}, INTR_EN: {=bool:?} }}" , self . WAKE_FLAG () , self . CLR_WAKE_TIMER () , self . OSC_DIV_ENA () , self . INTR_EN ())
        }
    }
    #[doc = "Year and Month Counters"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct YEARMON(pub u16);
    impl YEARMON {
        #[inline(always)]
        pub const fn MON_CNT(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_MON_CNT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn YROFST(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_YROFST(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
        }
    }
    impl Default for YEARMON {
        #[inline(always)]
        fn default() -> YEARMON {
            YEARMON(0)
        }
    }
    impl core::fmt::Debug for YEARMON {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("YEARMON")
                .field("MON_CNT", &self.MON_CNT())
                .field("YROFST", &self.YROFST())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for YEARMON {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "YEARMON {{ MON_CNT: {=u8:?}, YROFST: {=u8:?} }}",
                self.MON_CNT(),
                self.YROFST()
            )
        }
    }
}
