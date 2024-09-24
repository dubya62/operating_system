//! The purpose of this file is to handle all (normal/legacy) PCI
//! configuration for the kernel.
//!
//! It should allow the kernel to enumerate devices, load their configuration
//! space, and look for relevant drivers to control the device.

use core::arch::asm;
use alloc::vec::Vec;

use crate::file::disk;

const PCI_ADDRESS_PORT: u16 = 0xCF8;
const PCI_DATA_PORT: u16 = 0xCFC;



/// perform low level port input (reading from port)
pub fn inb(port: u16) -> u8 {
    let result: u8;
    unsafe {
        asm!(
            "in al, dx",
            in("dx") port,
            out("al") result,
        );
    }
    return result;
}

pub fn inw(port: u16) -> u16 {
    let result: u16;
    unsafe {
        asm!(
            "in ax, dx",
            in("dx") port,
            out("ax") result,
        );
    }
    return result;
}

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
pub fn outb(value: u8, port: u16) {
    unsafe {
        asm!(
            "out dx, al",
            in("al") value,
            in("dx") port,
        );
    }
}

pub fn outw(value: u16, port: u16) {
    unsafe {
        asm!(
            "out dx, ax",
            in("ax") value,
            in("dx") port,
        );
    }
}

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
                    let programming_interface: u32 = (class_codes >> 8) & 0xFF;
                    println!("CC: {}    SCC: {}    PI: {}", class_code, subclass_code, programming_interface);

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

    // FIXME: Implement this in a way better way 
    // (right now just need to be able to read/write to disk)
    /// Function to try to load ATA driver
    pub fn load_disk_driver(&self) -> disk::DiskDriver {
        // iterate through the addresses
        for i in 0..self.addresses.len() {
            // check each address to see if it needs a DiskDriver
            let class_codes: u32 = self.check_device(self.addresses[i].bus, self.addresses[i].device, self.addresses[i].function, 0x8);

            let class_code: u32 = class_codes >> 24;
            let subclass_code: u32 = (class_codes >> 16) & 0xFF;
            let programming_interface: u32 = (class_codes >> 8) & 0xFF;
            
            if class_code == 0x1 {
                if subclass_code == 0x1 {
                    // determine if device is in PCI native mode or not
                    let mut bar0: u32;
                    let mut bar1: u32;
                    let mut bar2: u32;
                    let mut bar3: u32;
                    let mut bar4: u32;
                    let mut bar5: u32;

                    if programming_interface & 0x1 == 0 {

                        println!("Here");
                        bar0 = self.check_device(self.addresses[i].bus, self.addresses[i].device, self.addresses[i].function, 0x10) & !(0xF);
                        bar1 = self.check_device(self.addresses[i].bus, self.addresses[i].device, self.addresses[i].function, 0x14) & !(0xF);
                        bar2 = self.check_device(self.addresses[i].bus, self.addresses[i].device, self.addresses[i].function, 0x18) & !(0xF);
                        bar3 = self.check_device(self.addresses[i].bus, self.addresses[i].device, self.addresses[i].function, 0x1C) & !(0xF);
                        bar4 = self.check_device(self.addresses[i].bus, self.addresses[i].device, self.addresses[i].function, 0x20) & !(0xF);
                        bar5 = self.check_device(self.addresses[i].bus, self.addresses[i].device, self.addresses[i].function, 0x24) & !(0xF);
                    } else {
                        println!("Device is in compatibility mode!");
                        bar0 = 0x1F0;
                        bar1 = 0x3f6;
                        bar2 = 0x170;
                        bar3 = 0x376;
                        bar4 = 0x0;
                        bar5 = 0x0;

                    }
                    println!("bar0: {}", bar0);
                    println!("bar1: {}", bar1);
                    println!("bar2: {}", bar2);
                    println!("bar3: {}", bar3);
                    println!("bar4: {}", bar4);
                    println!("bar5: {}", bar5);

                    return disk::DiskDriver::new(class_code, subclass_code, programming_interface, bar0, bar1, bar2, bar3, bar4, bar5);
                }
            }

        }

        return disk::DiskDriver::new(0, 0, 0, 0, 0, 0, 0, 0, 0);

    }

}










