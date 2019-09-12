//! Register information and driver to program xAPIC, X2APIC and I/O APIC

pub mod ioapic;
pub mod x2apic;
pub mod xapic;

#[repr(u64)]
pub enum DeliveryMode {
    /// Delivers the interrupt specified in the vector field to the target processor or processors.
    Fixed = 0b000,
    /// Same as fixed mode, except that the interrupt is delivered to the processor executing at the
    /// lowest priority among the set of processors specified in the destination field. The ability
    /// for a processor to send a lowest priority IPI is model specific and should be avoided by
    /// BIOS and operating system software.
    LowestPriority = 0b001,
    /// Delivers an SMI interrupt to the target processor or processors.
    /// The vector field must be programmed to 00H for future compatibility.
    SMI = 0b010,
    /// Reserved
    _Reserved = 0b11,
    /// Delivers an NMI interrupt to the target processor or processors.
    /// The vector information is ignored.
    NMI = 0b100,
    /// Delivers an INIT request to the target processor or processors, which causes them to perform an INIT.
    Init = 0b101,
    /// Sends a special start-up IPI (called a SIPI) to the target processor or processors.
    /// The vector typically points to a start-up routine that is part of the
    /// BIOS boot-strap code (see Section 8.4, Multiple-Processor (MP) Initialization). I
    /// PIs sent with this delivery mode are not automatically retried if the source
    /// APIC is unable to deliver it. It is up to the software to deter- mine if the
    /// SIPI was not successfully delivered and to reissue the SIPI if necessary.
    StartUp = 0b110,
}

#[repr(u64)]
pub enum DestinationMode {
    Physical = 0,
    Logical = 1,
}

#[repr(u64)]
pub enum DeliveryStatus {
    Idle = 0,
    SendPending = 1,
}

#[repr(u64)]
pub enum Level {
    Deassert = 0,
    Assert = 1,
}

#[repr(u64)]
pub enum TriggerMode {
    Edge = 0,
    Level = 1,
}

#[repr(u64)]
pub enum DestinationShorthand {
    NoShorthand = 0b00,
    Myself = 0b01,
    AllIncludingSelf = 0b10,
    AllExcludingSelf = 0b11,
}

pub struct Icr(u64);

impl Icr {
    pub fn new(
        vector: u8,
        destination: u8,
        destination_shorthand: DestinationShorthand,
        delivery_mode: DeliveryMode,
        destination_mode: DestinationMode,
        delivery_status: DeliveryStatus,
        level: Level,
        trigger_mode: TriggerMode,
    ) -> Icr {
        Icr((destination as u64) << 56
            | (destination_shorthand as u64) << 18
            | (trigger_mode as u64) << 15
            | (level as u64) << 14
            | (delivery_status as u64) << 12
            | (destination_mode as u64) << 11
            | (delivery_mode as u64) << 8
            | (vector as u64))
    }

    pub fn lower(&self) -> u32 {
        self.0 as u32
    }

    pub fn upper(&self) -> u32 {
        (self.0 >> 32) as u32
    }
}
