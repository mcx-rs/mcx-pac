#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (d5ec99b 2024-12-16))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct POWERQUAD {
    ptr: *mut u8,
}
unsafe impl Send for POWERQUAD {}
unsafe impl Sync for POWERQUAD {}
impl POWERQUAD {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn OUTBASE(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn OUTFORMAT(self) -> crate::common::Reg<regs::OUTFORMAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn TMPBASE(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn TMPFORMAT(self) -> crate::common::Reg<regs::TMPFORMAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn INABASE(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn INAFORMAT(self) -> crate::common::Reg<regs::INAFORMAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn INBBASE(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn INBFORMAT(self) -> crate::common::Reg<regs::INBFORMAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[inline(always)]
    pub const fn CONTROL(self) -> crate::common::Reg<regs::CONTROL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[inline(always)]
    pub const fn LENGTH(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[inline(always)]
    pub const fn CPPRE(self) -> crate::common::Reg<regs::CPPRE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[inline(always)]
    pub const fn MISC(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x010cusize) as _) }
    }
    #[inline(always)]
    pub const fn CURSORY(self) -> crate::common::Reg<regs::CURSORY, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
    }
    #[inline(always)]
    pub const fn CORDIC_X(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0180usize) as _) }
    }
    #[inline(always)]
    pub const fn CORDIC_Y(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0184usize) as _) }
    }
    #[inline(always)]
    pub const fn CORDIC_Z(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0188usize) as _) }
    }
    #[inline(always)]
    pub const fn ERRSTAT(self) -> crate::common::Reg<regs::ERRSTAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x018cusize) as _) }
    }
    #[inline(always)]
    pub const fn INTREN(self) -> crate::common::Reg<regs::INTREN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0190usize) as _) }
    }
    #[inline(always)]
    pub const fn EVENTEN(self) -> crate::common::Reg<regs::EVENTEN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0194usize) as _) }
    }
    #[inline(always)]
    pub const fn INTRSTAT(self) -> crate::common::Reg<regs::INTRSTAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0198usize) as _) }
    }
    #[inline(always)]
    pub const fn GPREG(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn COMPREG(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0240usize + n * 4usize) as _) }
    }
}
pub mod regs {
    #[doc = "Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CONTROL(pub u32);
    impl CONTROL {
        #[inline(always)]
        pub const fn DECODE_OPCODE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_DECODE_OPCODE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn DECODE_MACHINE(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_DECODE_MACHINE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[inline(always)]
        pub const fn INST_BUSY(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INST_BUSY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for CONTROL {
        #[inline(always)]
        fn default() -> CONTROL {
            CONTROL(0)
        }
    }
    impl core::fmt::Debug for CONTROL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CONTROL")
                .field("DECODE_OPCODE", &self.DECODE_OPCODE())
                .field("DECODE_MACHINE", &self.DECODE_MACHINE())
                .field("INST_BUSY", &self.INST_BUSY())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CONTROL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CONTROL {
                DECODE_OPCODE: u8,
                DECODE_MACHINE: u8,
                INST_BUSY: bool,
            }
            let proxy = CONTROL {
                DECODE_OPCODE: self.DECODE_OPCODE(),
                DECODE_MACHINE: self.DECODE_MACHINE(),
                INST_BUSY: self.INST_BUSY(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Coprocessor Prescale"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CPPRE(pub u32);
    impl CPPRE {
        #[inline(always)]
        pub const fn CPPRE_IN(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_CPPRE_IN(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn CPPRE_OUT(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_CPPRE_OUT(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[inline(always)]
        pub const fn CPPRE_SAT(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CPPRE_SAT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn CPPRE_SAT8(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CPPRE_SAT8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for CPPRE {
        #[inline(always)]
        fn default() -> CPPRE {
            CPPRE(0)
        }
    }
    impl core::fmt::Debug for CPPRE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CPPRE")
                .field("CPPRE_IN", &self.CPPRE_IN())
                .field("CPPRE_OUT", &self.CPPRE_OUT())
                .field("CPPRE_SAT", &self.CPPRE_SAT())
                .field("CPPRE_SAT8", &self.CPPRE_SAT8())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CPPRE {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CPPRE {
                CPPRE_IN: u8,
                CPPRE_OUT: u8,
                CPPRE_SAT: bool,
                CPPRE_SAT8: bool,
            }
            let proxy = CPPRE {
                CPPRE_IN: self.CPPRE_IN(),
                CPPRE_OUT: self.CPPRE_OUT(),
                CPPRE_SAT: self.CPPRE_SAT(),
                CPPRE_SAT8: self.CPPRE_SAT8(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Cursory"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CURSORY(pub u32);
    impl CURSORY {
        #[inline(always)]
        pub const fn CURSORY(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CURSORY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for CURSORY {
        #[inline(always)]
        fn default() -> CURSORY {
            CURSORY(0)
        }
    }
    impl core::fmt::Debug for CURSORY {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CURSORY")
                .field("CURSORY", &self.CURSORY())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CURSORY {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CURSORY {
                CURSORY: bool,
            }
            let proxy = CURSORY {
                CURSORY: self.CURSORY(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Error Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ERRSTAT(pub u32);
    impl ERRSTAT {
        #[inline(always)]
        pub const fn OVERFLOW(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OVERFLOW(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn NAN(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NAN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn FIXEDOVERFLOW(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FIXEDOVERFLOW(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn UNDERFLOW(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_UNDERFLOW(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn BUSERROR(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BUSERROR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for ERRSTAT {
        #[inline(always)]
        fn default() -> ERRSTAT {
            ERRSTAT(0)
        }
    }
    impl core::fmt::Debug for ERRSTAT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ERRSTAT")
                .field("OVERFLOW", &self.OVERFLOW())
                .field("NAN", &self.NAN())
                .field("FIXEDOVERFLOW", &self.FIXEDOVERFLOW())
                .field("UNDERFLOW", &self.UNDERFLOW())
                .field("BUSERROR", &self.BUSERROR())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ERRSTAT {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ERRSTAT {
                OVERFLOW: bool,
                NAN: bool,
                FIXEDOVERFLOW: bool,
                UNDERFLOW: bool,
                BUSERROR: bool,
            }
            let proxy = ERRSTAT {
                OVERFLOW: self.OVERFLOW(),
                NAN: self.NAN(),
                FIXEDOVERFLOW: self.FIXEDOVERFLOW(),
                UNDERFLOW: self.UNDERFLOW(),
                BUSERROR: self.BUSERROR(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Event Enable"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EVENTEN(pub u32);
    impl EVENTEN {
        #[inline(always)]
        pub const fn EVENT_OFLOW(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EVENT_OFLOW(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn EVENT_NAN(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EVENT_NAN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn EVENT_FIXED(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EVENT_FIXED(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn EVENT_UFLOW(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EVENT_UFLOW(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn EVENT_BERR(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EVENT_BERR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn EVENT_COMP(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EVENT_COMP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for EVENTEN {
        #[inline(always)]
        fn default() -> EVENTEN {
            EVENTEN(0)
        }
    }
    impl core::fmt::Debug for EVENTEN {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EVENTEN")
                .field("EVENT_OFLOW", &self.EVENT_OFLOW())
                .field("EVENT_NAN", &self.EVENT_NAN())
                .field("EVENT_FIXED", &self.EVENT_FIXED())
                .field("EVENT_UFLOW", &self.EVENT_UFLOW())
                .field("EVENT_BERR", &self.EVENT_BERR())
                .field("EVENT_COMP", &self.EVENT_COMP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EVENTEN {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct EVENTEN {
                EVENT_OFLOW: bool,
                EVENT_NAN: bool,
                EVENT_FIXED: bool,
                EVENT_UFLOW: bool,
                EVENT_BERR: bool,
                EVENT_COMP: bool,
            }
            let proxy = EVENTEN {
                EVENT_OFLOW: self.EVENT_OFLOW(),
                EVENT_NAN: self.EVENT_NAN(),
                EVENT_FIXED: self.EVENT_FIXED(),
                EVENT_UFLOW: self.EVENT_UFLOW(),
                EVENT_BERR: self.EVENT_BERR(),
                EVENT_COMP: self.EVENT_COMP(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Input A Format"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct INAFORMAT(pub u32);
    impl INAFORMAT {
        #[inline(always)]
        pub const fn INA_FORMATINT(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_INA_FORMATINT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[inline(always)]
        pub const fn INA_FORMATEXT(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_INA_FORMATEXT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[inline(always)]
        pub const fn INA_SCALER(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_INA_SCALER(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for INAFORMAT {
        #[inline(always)]
        fn default() -> INAFORMAT {
            INAFORMAT(0)
        }
    }
    impl core::fmt::Debug for INAFORMAT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("INAFORMAT")
                .field("INA_FORMATINT", &self.INA_FORMATINT())
                .field("INA_FORMATEXT", &self.INA_FORMATEXT())
                .field("INA_SCALER", &self.INA_SCALER())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for INAFORMAT {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct INAFORMAT {
                INA_FORMATINT: u8,
                INA_FORMATEXT: u8,
                INA_SCALER: u8,
            }
            let proxy = INAFORMAT {
                INA_FORMATINT: self.INA_FORMATINT(),
                INA_FORMATEXT: self.INA_FORMATEXT(),
                INA_SCALER: self.INA_SCALER(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Input B Format"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct INBFORMAT(pub u32);
    impl INBFORMAT {
        #[inline(always)]
        pub const fn INB_FORMATINT(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_INB_FORMATINT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[inline(always)]
        pub const fn INB_FORMATEXT(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_INB_FORMATEXT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[inline(always)]
        pub const fn INB_SCALER(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_INB_SCALER(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for INBFORMAT {
        #[inline(always)]
        fn default() -> INBFORMAT {
            INBFORMAT(0)
        }
    }
    impl core::fmt::Debug for INBFORMAT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("INBFORMAT")
                .field("INB_FORMATINT", &self.INB_FORMATINT())
                .field("INB_FORMATEXT", &self.INB_FORMATEXT())
                .field("INB_SCALER", &self.INB_SCALER())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for INBFORMAT {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct INBFORMAT {
                INB_FORMATINT: u8,
                INB_FORMATEXT: u8,
                INB_SCALER: u8,
            }
            let proxy = INBFORMAT {
                INB_FORMATINT: self.INB_FORMATINT(),
                INB_FORMATEXT: self.INB_FORMATEXT(),
                INB_SCALER: self.INB_SCALER(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Interrupt Enable"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct INTREN(pub u32);
    impl INTREN {
        #[inline(always)]
        pub const fn INTR_OFLOW(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INTR_OFLOW(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn INTR_NAN(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INTR_NAN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn INTR_FIXED(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INTR_FIXED(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn INTR_UFLOW(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INTR_UFLOW(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn INTR_BERR(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INTR_BERR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn INTR_COMP(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INTR_COMP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for INTREN {
        #[inline(always)]
        fn default() -> INTREN {
            INTREN(0)
        }
    }
    impl core::fmt::Debug for INTREN {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("INTREN")
                .field("INTR_OFLOW", &self.INTR_OFLOW())
                .field("INTR_NAN", &self.INTR_NAN())
                .field("INTR_FIXED", &self.INTR_FIXED())
                .field("INTR_UFLOW", &self.INTR_UFLOW())
                .field("INTR_BERR", &self.INTR_BERR())
                .field("INTR_COMP", &self.INTR_COMP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for INTREN {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct INTREN {
                INTR_OFLOW: bool,
                INTR_NAN: bool,
                INTR_FIXED: bool,
                INTR_UFLOW: bool,
                INTR_BERR: bool,
                INTR_COMP: bool,
            }
            let proxy = INTREN {
                INTR_OFLOW: self.INTR_OFLOW(),
                INTR_NAN: self.INTR_NAN(),
                INTR_FIXED: self.INTR_FIXED(),
                INTR_UFLOW: self.INTR_UFLOW(),
                INTR_BERR: self.INTR_BERR(),
                INTR_COMP: self.INTR_COMP(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Interrupt Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct INTRSTAT(pub u32);
    impl INTRSTAT {
        #[inline(always)]
        pub const fn INTR_STAT(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INTR_STAT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for INTRSTAT {
        #[inline(always)]
        fn default() -> INTRSTAT {
            INTRSTAT(0)
        }
    }
    impl core::fmt::Debug for INTRSTAT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("INTRSTAT")
                .field("INTR_STAT", &self.INTR_STAT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for INTRSTAT {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct INTRSTAT {
                INTR_STAT: bool,
            }
            let proxy = INTRSTAT {
                INTR_STAT: self.INTR_STAT(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Output Format"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OUTFORMAT(pub u32);
    impl OUTFORMAT {
        #[inline(always)]
        pub const fn OUT_FORMATINT(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_OUT_FORMATINT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[inline(always)]
        pub const fn OUT_FORMATEXT(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_OUT_FORMATEXT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[inline(always)]
        pub const fn OUT_SCALER(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_OUT_SCALER(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for OUTFORMAT {
        #[inline(always)]
        fn default() -> OUTFORMAT {
            OUTFORMAT(0)
        }
    }
    impl core::fmt::Debug for OUTFORMAT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OUTFORMAT")
                .field("OUT_FORMATINT", &self.OUT_FORMATINT())
                .field("OUT_FORMATEXT", &self.OUT_FORMATEXT())
                .field("OUT_SCALER", &self.OUT_SCALER())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OUTFORMAT {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct OUTFORMAT {
                OUT_FORMATINT: u8,
                OUT_FORMATEXT: u8,
                OUT_SCALER: u8,
            }
            let proxy = OUTFORMAT {
                OUT_FORMATINT: self.OUT_FORMATINT(),
                OUT_FORMATEXT: self.OUT_FORMATEXT(),
                OUT_SCALER: self.OUT_SCALER(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Temporary Format"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TMPFORMAT(pub u32);
    impl TMPFORMAT {
        #[inline(always)]
        pub const fn TMP_FORMATINT(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_TMP_FORMATINT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[inline(always)]
        pub const fn TMP_FORMATEXT(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_TMP_FORMATEXT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[inline(always)]
        pub const fn TMP_SCALER(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_TMP_SCALER(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for TMPFORMAT {
        #[inline(always)]
        fn default() -> TMPFORMAT {
            TMPFORMAT(0)
        }
    }
    impl core::fmt::Debug for TMPFORMAT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TMPFORMAT")
                .field("TMP_FORMATINT", &self.TMP_FORMATINT())
                .field("TMP_FORMATEXT", &self.TMP_FORMATEXT())
                .field("TMP_SCALER", &self.TMP_SCALER())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TMPFORMAT {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct TMPFORMAT {
                TMP_FORMATINT: u8,
                TMP_FORMATEXT: u8,
                TMP_SCALER: u8,
            }
            let proxy = TMPFORMAT {
                TMP_FORMATINT: self.TMP_FORMATINT(),
                TMP_FORMATEXT: self.TMP_FORMATEXT(),
                TMP_SCALER: self.TMP_SCALER(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
}
