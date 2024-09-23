//! The purpose of this file is for device detection
//! Specifically, this file deals with PCI bus enumeration
//! to detect connected devices


/*
x86
in eax, dx
out dx, eax 
*/

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
    pub fn create_address(&self, offset: u32) -> u32 {
        return 
            (1 << 31) |
            (self.bus_number << 16) |
            (self.device_number << 11) |
            (self.function_number << 8) |
            (self.register_number << 2) |
            (offset & 0xFC);
    }

}


/*
/// Dealing with class codes to dynamically load drivers
pub fn debug_device(bus: u32, device: u32, function: u32) {
    // create address struct
    let test_addr: PciAddress = PciAddress::new(bus, device, function, 0);

    let class_addr: u32 = test_addr.create_address(0x0B);
    
    // write to pci address port
    outl(class_addr, PCI_ADDRESS_PORT);
    // read and return from data port
    let result: u32 = inl(PCI_DATA_PORT);

    let class_code = result >> 24;
    let subclass_code = (result & 0x00FF0000) >> 16;
    let programming_interface = (result & 0x0000FF00) >> 8;

    println!("CC: {}    SCC: {}    PI: {}", class_code, subclass_code, programming_interface);

}
*/


// PCI config reference: https://wiki.osdev.org/PCI
/// A single hardware device that can be interacted with
pub struct Device {
    // used to calculate address
    bus: u32, 
    device: u32, 
    function: u32,

    address: u32, // pci address of the device (created from bus, device, and function) 
                  
    // configuration loaded from PCI port:
    device_id: u16, // identifies particular device
    vendor_id: u16, // identifies manufacturer of device
    
    status: u16, // a greister sued to record status information
    command: u16, // controls device's ability to generate and respond to PCI cycles

    class_code: u8, // specifies the type of function the device performs
    subclass_code: u8,

    programming_if: u8,
    revision_id: u8,

    bist: u8,
    header_type: u8,
    latency_timer: u8,
    cache_line_size: u8,

}

impl Device {
    /// constructor
    /*
    pub fn new(bus: u32, device: u32, function: u32) -> Self {
        return Device {
            bus: bus,
            device: device,
            function: function,

            address: 0,

            device_id: 0,
            vendor_id: 0,
        };
    }
    */

    pub fn load_config() {

    }
}


/// Main pci controller for the kernel
/// handles enumeration and drivers for connected devices
pub struct Pci {
    addresses: Vec<u32>,
    devices: Vec<Device>,
}

impl Pci {
    /// constructor
    pub fn new() -> Self {
        return Pci {
            addresses: Vec::new(),
            devices: Vec::new(),
        };
    }

    /// Check to see if a certain device is connected
    /// Return 0xFFFF if this device is not connected
    pub fn check_for_device(addr: u32) -> u32 {
        // write to pci address port
        outl(addr, PCI_ADDRESS_PORT);
        // read and return from data port
        return inl(PCI_DATA_PORT);
    }

    /// enumerate all possible pci devices and print their IDs
    pub fn enumerate_pci(&mut self) {
        println!("Enumerating pci addresses...");
        let mut curr: u32;
        for bus in 0..256 {
            for device in 0..32 {
                for function in 0..8 {
                    // calculate the address
                    let addr: u32 = 
                            (1 << 31) |
                            (bus << 16) |
                            (device << 11) |
                            (function << 8);

                    curr = Self::check_for_device(addr);

                    // check if this is a valid device
                    let vendor_id = curr & 0xFFFF;
                    if vendor_id != 0xFFFF {
                        // add its address to the list of devices
                        self.addresses.push(addr);

                        let device_id = (curr & 0xFFFF0000) >> 16;
                        println!("VendorID: {}    DeviceId: {}", vendor_id, device_id);
                    }

                }
            }
        }
        println!("Finished enumerating addresses!");
    }

    /// Fill device vector with Device structures based on the enumerated pci config
    pub fn create_devices(&mut self) {
        for i in 0..self.addresses.len() {
            //self.create_device(self.addresses[i]);
        }
    }

    /// enumerate pci devices and add their configuration
    pub fn init(&mut self) {
        self.enumerate_pci();
    
        self.create_devices();
    }

}






