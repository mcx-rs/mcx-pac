#[derive(Debug)]
pub struct Metadata {
    pub name: &'static str,
    pub nvic_priority_bits: u8,
    pub interrupts: &'static [Interrupt],
    pub peripherals: &'static [Peripheral],
    pub pins: &'static [Pin],
}

#[derive(Debug)]
pub struct Interrupt {
    pub name: &'static str,
    pub description: &'static str,
    pub value: u32,
}

#[derive(Debug)]
pub struct Peripheral {
    pub name: &'static str,
    pub address: u64,
    pub control: Option<PeripheralControl>,
    pub pins: &'static [PeripheralPin],
}

#[derive(Debug)]
pub enum PeripheralControl {
    MRCC {
        reg: u8,
        bit: u8,
        rst: bool,
        clk: bool,
    },
    SYSCON,
}

#[derive(Debug)]
pub struct PeripheralPin {
    pub pin: &'static str,
    pub signal: &'static str,
    pub mux: Option<u8>,
}

#[derive(Debug)]
pub struct Pin {
    pub name: &'static str,
}
