//! The purpose of this file is for device detection
//! Specifically, this file deals with PCI bus enumeration
//! to detect connected devices


// FIXME: having problem where vendor data is always 0x1111
// it is probably something to do with the offset
// reference: https://forum.osdev.org/viewtopic.php?t=42725

/*
x86
in eax, dx
out dx, eax 
*/

use core::arch::asm;

/// perform low level port input (reading from port)
pub fn inl(port: u16) -> u32 {
    let result: u32;
    unsafe {
        asm!(
            "mov dx, {0:x}",
            "in eax, dx",
            "mov {1:e}, eax",
            in(reg) port,
            out(reg) result,
        );
    }
    return result;
}

/// perform low level port output (writing to port)
pub fn outl(value: u32, port: u16) {
    unsafe {
        asm!(
            "mov eax, {0:e}",
            "mov dx, {1:x}",
            "out dx, eax",
            in(reg) value,
            in(reg) port,
        );
    }
}

/// A pci address as a structure
/// Allows easier access to each element of the address
pub struct PciAddress {
    bus_number: u32,
    device_number: u32,
    function_number: u32,
    register_number: u32,
}

impl PciAddress {
    /// consturctor
    pub fn new(bus_number: u32, device_number: u32, function_number: u32, register_number: u32) -> Self {
        return PciAddress {
            bus_number: bus_number,
            device_number: device_number,
            function_number: function_number,
            register_number: register_number,
        };
    }

    /// function to create a 32-bit pci address
    pub fn create_address(&self) -> u32 {
        return 
            (1 << 31) |
            (self.bus_number << 16) |
            (self.device_number << 11) |
            (self.function_number << 8) |
            (self.register_number << 2);
    }

    // function to create an instance from an address
    /*
    pub fn parse_address(address: u32) -> Self {
        return new(
            (address & ),
        );

    }
    */

}

/// Check to see if a certain device is connected
/// Return 0xFFFF if this device is not connected
pub fn check_for_device(bus: u32, device: u32, function: u32) -> u32 {
    // reimplemented here for speed (hopefully)
    let addr: u32 = 
            (1 << 31) |
            (bus << 16) |
            (device << 11) |
            (function << 8);
    // write to pci address port
    outl(addr, 0xCF8);
    // read and return from data port
    return inl(0xCFC);
}

// FIXME: see note at top of file
pub fn enumerate_pci() {
    println!("Enumerating pci addresses...");
    let mut curr: u32;
    for bus in 0..256 {
        for device in 0..32 {
            for function in 0..8 {
                curr = check_for_device(bus, device, function);
                if curr != 0xFFFF {
                    println!("Device found: {}", curr);
                }
            }
        }
    }
    println!("Finished enumerating addresses!");
}







