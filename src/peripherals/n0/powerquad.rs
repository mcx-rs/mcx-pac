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
    pub const fn OUTBASE(self) -> crate::common::Reg<regs::OUTBASE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn OUTFORMAT(self) -> crate::common::Reg<regs::OUTFORMAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn TMPBASE(self) -> crate::common::Reg<regs::TMPBASE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn TMPFORMAT(self) -> crate::common::Reg<regs::TMPFORMAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn INABASE(self) -> crate::common::Reg<regs::INABASE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn INAFORMAT(self) -> crate::common::Reg<regs::INAFORMAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn INBBASE(self) -> crate::common::Reg<regs::INBBASE, crate::common::RW> {
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
    pub const fn LENGTH(self) -> crate::common::Reg<regs::LENGTH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[inline(always)]
    pub const fn CPPRE(self) -> crate::common::Reg<regs::CPPRE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[inline(always)]
    pub const fn MISC(self) -> crate::common::Reg<regs::MISC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x010cusize) as _) }
    }
    #[inline(always)]
    pub const fn CURSORY(self) -> crate::common::Reg<regs::CURSORY, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
    }
    #[inline(always)]
    pub const fn CORDIC_X(self) -> crate::common::Reg<regs::CORDIC_X, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0180usize) as _) }
    }
    #[inline(always)]
    pub const fn CORDIC_Y(self) -> crate::common::Reg<regs::CORDIC_Y, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0184usize) as _) }
    }
    #[inline(always)]
    pub const fn CORDIC_Z(self) -> crate::common::Reg<regs::CORDIC_Z, crate::common::RW> {
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
    pub const fn GPREG(self, n: usize) -> crate::common::Reg<regs::GPREG, crate::common::RW> {
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
    #[doc = "Compute Register Bank n"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct COMPREG(pub u32);
    impl COMPREG {
        #[inline(always)]
        pub const fn COMPREG(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_COMPREG(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for COMPREG {
        #[inline(always)]
        fn default() -> COMPREG {
            COMPREG(0)
        }
    }
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
    #[doc = "CORDIC Input X"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CORDIC_X(pub u32);
    impl CORDIC_X {
        #[inline(always)]
        pub const fn CORDIC_X(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_CORDIC_X(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for CORDIC_X {
        #[inline(always)]
        fn default() -> CORDIC_X {
            CORDIC_X(0)
        }
    }
    #[doc = "CORDIC Input Y"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CORDIC_Y(pub u32);
    impl CORDIC_Y {
        #[inline(always)]
        pub const fn CORDIC_Y(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_CORDIC_Y(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for CORDIC_Y {
        #[inline(always)]
        fn default() -> CORDIC_Y {
            CORDIC_Y(0)
        }
    }
    #[doc = "CORDIC Input Z"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CORDIC_Z(pub u32);
    impl CORDIC_Z {
        #[inline(always)]
        pub const fn CORDIC_Z(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_CORDIC_Z(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for CORDIC_Z {
        #[inline(always)]
        fn default() -> CORDIC_Z {
            CORDIC_Z(0)
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
    #[doc = "General Purpose Register Bank n"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GPREG(pub u32);
    impl GPREG {
        #[inline(always)]
        pub const fn GPREG(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_GPREG(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for GPREG {
        #[inline(always)]
        fn default() -> GPREG {
            GPREG(0)
        }
    }
    #[doc = "Input A Base"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct INABASE(pub u32);
    impl INABASE {
        #[inline(always)]
        pub const fn INABASE(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_INABASE(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for INABASE {
        #[inline(always)]
        fn default() -> INABASE {
            INABASE(0)
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
    #[doc = "Input B Base"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct INBBASE(pub u32);
    impl INBBASE {
        #[inline(always)]
        pub const fn INBBASE(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_INBBASE(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for INBBASE {
        #[inline(always)]
        fn default() -> INBBASE {
            INBBASE(0)
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
    #[doc = "Length"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LENGTH(pub u32);
    impl LENGTH {
        #[inline(always)]
        pub const fn INST_LENGTH(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_INST_LENGTH(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for LENGTH {
        #[inline(always)]
        fn default() -> LENGTH {
            LENGTH(0)
        }
    }
    #[doc = "Miscellaneous"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MISC(pub u32);
    impl MISC {
        #[inline(always)]
        pub const fn INST_MISC(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_INST_MISC(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for MISC {
        #[inline(always)]
        fn default() -> MISC {
            MISC(0)
        }
    }
    #[doc = "Output Base"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OUTBASE(pub u32);
    impl OUTBASE {
        #[inline(always)]
        pub const fn OUTBASE(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_OUTBASE(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for OUTBASE {
        #[inline(always)]
        fn default() -> OUTBASE {
            OUTBASE(0)
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
    #[doc = "Temporary Base"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TMPBASE(pub u32);
    impl TMPBASE {
        #[inline(always)]
        pub const fn TMPBASE(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_TMPBASE(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for TMPBASE {
        #[inline(always)]
        fn default() -> TMPBASE {
            TMPBASE(0)
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
}
