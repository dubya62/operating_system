//! The purpose of this program is to contain different disk drivers
//! with raw io abilities.

// TODO: This implementation should be significantly cleaned up.
// For right now, I just want something that can interact with the disk at all


use crate::file::pci::{inb, inw, inl, outb, outw, outl};



/// Structure to hold ports for ATA registers
struct AtaRegisters {
    data: u16, // read/write data on disk
    error: u16, // check for errors/set features
    sector_count: u16, // number of sectors to use
    lba_low: u16, // low 8 bits of lba addr
    lba_mid: u16, // mid 8 bits of lba addr
    lba_high: u16, // high 8 bits of lba addr
    drive: u16, // drive selector
    command: u16, // write command to controller or read status
    control: u16, // perform resets
}

impl AtaRegisters {
    /// Constructor
    pub fn new(bar0: u16, bar1: u16) -> Self {
        return AtaRegisters {
            data: bar0,
            error: bar0 + 1,
            sector_count: bar0 + 2,
            lba_low: bar0 + 3,
            lba_mid: bar0 + 4,
            lba_high: bar0 + 5,
            drive: bar0 + 6,
            command: bar0 + 7,
            control: bar1 + 2,
        };
    }

}

pub struct DiskDriver {
    class: u32,
    subclass: u32,
    programming_interface: u32,

    bar0: u32,
    bar1: u32,
    bar2: u32,
    bar3: u32,
    bar4: u32,
    bar5: u32,

    regs: AtaRegisters,

}

impl DiskDriver {
    /// constructor
    pub fn new(class: u32, subclass: u32, programming_interface: u32, bar0: u32, bar1: u32, bar2: u32, bar3: u32, bar4: u32, bar5: u32) -> Self {
        return DiskDriver {
            class: class,
            subclass: subclass,
            programming_interface: programming_interface,

            bar0: bar0,
            bar1: bar1,
            bar2: bar2,
            bar3: bar3,
            bar4: bar4,
            bar5: bar5,

            regs: AtaRegisters::new(bar0 as u16, bar1 as u16),
        };
    }

    /// wait for the drive to not be busy
    pub fn wait_ready(&self) {
        // wait for the busy bit to clear
        println!("Waiting for BSY bit to clear...");
        while inb(self.regs.command) & 0x80 != 0 {}
        println!("Drive is no longer busy!");
    }

    /// select the drive and options
    pub fn select_drive(&self, lba: usize) {
        let head: u8 = ((lba & 0xF000000) >> 24) as u8;
        let slavebit: u8 = 0; // 0 = master drive
        let value: u8 = 0xE0 | (slavebit << 4) | head;
        outb(value, self.regs.drive);
    }

    // TODO: allow this to use either 24 bit or 48 bit lba (currently only 24 bit)
    /// set the lba to a certain position
    pub fn set_lba(&self, lba: usize) {
        // write the low value
        outw(self.regs.lba_low, (lba & 0xFF) as u16);
        // write the mid value
        outw(self.regs.lba_mid, ((lba & 0xFF00) >> 8) as u16);
        // write the high value
        outw(self.regs.lba_high, ((lba & 0xFF0000) >> 16) as u16);
    }

    /// set the sector count
    pub fn set_sector_count(&self, sectors: u8) {
        // write the count to the correct port
        outb(sectors, self.regs.sector_count);
    }

    /// write to disk from buffer
    /// lba: logical block address of sector
    /// buffer: buffer to get data to write from
    pub fn write_sector(&self, lba: usize, buffer: &[u8]) {
        // wait for the drive to be ready
        self.wait_ready();

        // set lba to correct position
        self.set_lba(lba);
        
    }
    
    /// read from disk into buffer
    /// lba: logical block address of sector
    /// buffer: buffer to write data to
    pub fn read_sector(&self, lba: usize, buffer: &mut [u8]) {
        // wait for the drive to be ready
        self.wait_ready();

        // select drive
        self.select_drive(lba);

        // set lba to correct position
        self.set_lba(lba);

        // set sector count to correct value
        self.set_sector_count(1u8);

        // send the read command
        println!("Sending read command...");
        outb(0x20, self.regs.command);

        // do 256 reads per sector since
        // the register is u16
        println!("Reading data...");
        for i in 0..256 {
            // wait for DRQ bit
            while inb(self.regs.command) & 0x08 != 0 {}
            // read data and write it to the buffer
            let curr: u16 = inw(self.regs.data);
            buffer[i] = ((curr & 0xFF00) >> 8) as u8;
            buffer[i+1] = (curr & 0xFF) as u8;
            //println!("curr = {}", curr);
        }
        println!("Finished Reading sector!");

        /*
        println!("{}", self.bar0);
        println!("{}", self.bar1);
        println!("{}", self.bar2);
        println!("{}", self.bar3);
        println!("{}", self.bar4);
        println!("{}", self.bar5);
        */

    }


    /// write using master bus
    pub fn master_write_sector(&self) {
        // get base address of master ports
        let addr: u32 = self.bar4 & 0xFFFFFF00;
    }

    /// read using master bus
    pub fn master_read_sector(&self) {
        // get base address of master ports
        let addr: u32 = self.bar4 & 0xFFFFFF00;
        

        
    }

}





