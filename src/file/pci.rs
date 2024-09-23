//! The purpose of this file is to handle all (normal/legacy) PCI
//! configuration for the kernel.
//!
//! It should allow the kernel to enumerate devices, load their configuration
//! space, and look for relevant drivers to control the device.

use core::arch::asm;
use alloc::vec::Vec;


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
    return result;
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


/// structure to store Pci Address information
pub struct PciAddress {
    bus: u32,
    device: u32,
    function: u32,
}

impl PciAddress {
    pub fn new(bus: u32, device: u32, function: u32) -> Self {
        return PciAddress {
            bus: bus,
            device: device,
            function: function,
        };
    }

}


/// Main structure to interact with PCI
pub struct Pci {
    // Vector containing all active pci addresses
    addresses: Vec<PciAddress>,

}

impl Pci {
    /// constructor
    pub fn new() -> Self {
        return Pci {
            addresses: Vec::new(),
        }
    }

    /// function to check a single device address
    fn check_device(&self, bus: u32, device: u32, function: u32, offset: u32) -> u32 {
        let addr: u32 = (1 << 31) |
            (bus << 16) |
            (device << 11) |
            (function << 8) |
            (offset & 0xFC);
        // write the address to address port
        outl(addr, PCI_ADDRESS_PORT);
        // return data from data port
        return inl(PCI_DATA_PORT);
    }

    /// function to enumerate a single bus
    fn enumerate_bus(&mut self, bus: u32) {
        for device in 0..32 {
            for function in 0..8 {
                // get output of device
                let curr = self.check_device(bus, device, function, 0);

                // if the vendor is not 0xFFFF,
                // add the address information
                if curr & 0x0000FFFF != 0xFFFF {
                    let new_addr: PciAddress = PciAddress::new(bus, device, function);
                    self.addresses.push(new_addr);

                    // check if the device is a bridge
                    let class_codes: u32 = self.check_device(bus, device, function, 0x8);
                    // first 8 bits are class code
                    // second 8 bits are subclass code
                     
                    let class_code: u32 = class_codes >> 24;
                    let subclass_code: u32 = (class_codes >> 16) & 0xFF;
                    println!("CC: {}    SCC: {}", class_code, subclass_code);

                    let class_codes: u32 = self.check_device(bus, device, function, 0xC);

                    let header: u32 = (class_codes >> 16) & 0xFF;
                    println!("HEADER: {:#x}", header);
                    // TODO: enumerate pci-pci bridges

                }

            }
        }

    }

    /// Function to enumerate the PCI space
    pub fn enumerate_pci(&mut self) {
        // empty the vector of addresses
        self.addresses.truncate(0);
            
        // iterate through the first bus
        self.enumerate_bus(0);

    }

    

}










