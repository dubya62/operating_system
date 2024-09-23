//! The purpose of this file is for device detection
//! Specifically, this file deals with PCI bus enumeration
//! to detect connected devices

/*
x86
in eax, dx
out dx, eax
*/

use core::arch::asm;

const PCI_ADDRESS_PORT: u16 = 0xCF8;
const PCI_DATA_PORT: u16 = 0xCFC;

/// perform low level port input (reading from port)
pub fn inl(port: u16) -> u32 {
    let result: u32;
    unsafe {
        asm!(
            "in eax, dx",
            in("dx") port,
            out("eax") result,
        );
    }
    result
}

/// perform low level port output (writing to port)
pub fn outl(value: u32, port: u16) {
    unsafe {
        asm!(
            "out dx, eax",
            in("eax") value,
            in("dx") port,
        );
    }
}

/// A pci address as a structure
/// Allows easier access to each element of the address
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct PciAddress {
    pub bus_number: u32,
    pub device_number: u32,
    pub function_number: u32,
    pub register_number: u32,
}

impl PciAddress {
    /// function to create a 32-bit pci address
    pub fn create_address(&self, offset: u32) -> u32 {
        (1 << 31)
            | (self.bus_number << 16)
            | (self.device_number << 11)
            | (self.function_number << 8)
            | (self.register_number << 2)
            | (offset & 0xFC)
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
    let addr = (1 << 31) | (bus << 16) | (device << 11) | (function << 8);
    // write to pci address port
    outl(addr, PCI_ADDRESS_PORT);
    // read and return from data port
    inl(PCI_DATA_PORT)
}

/// enumerate all possible pci devices and print their IDs
// TODO: Make this return an iterator so we can use this in code
pub fn enumerate_pci() {
    println!("Enumerating pci addresses...");
    let mut curr: u32;
    for bus in 0..256 {
        for device in 0..32 {
            for function in 0..8 {
                curr = check_for_device(bus, device, function);
                if curr & 0xFFFF != 0xFFFF {
                    println!("Device found: {}", curr);
                    let vendor_id = curr & 0xFFFF;
                    let device_id = (curr & 0xFFFF0000) >> 16;
                    println!("VendorID: {}", vendor_id);
                    println!("DeviceID: {}", device_id);
                }
            }
        }
    }
    println!("Finished enumerating addresses!");
}
