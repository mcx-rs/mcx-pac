pub struct Metadata {
    pub nvic_priority_bits: u8,
    pub peripherals: &'static [Peripheral],
    pub interrupts: &'static [Interrupt],
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Peripheral {
    pub name: &'static str,
    pub base_address: u32,
    pub block: &'static str,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Interrupt {
    pub name: &'static str,
    pub description: &'static str,
    pub value: u32,
}
