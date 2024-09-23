//! The purpose of this program is to contain different disk drivers
//! with raw io abilities.


pub struct DiskDriver {
    class: u32,
    subclass: u32,
    programming_interface: u32,
}

impl DiskDriver {
    /// constructor
    pub fn new(class: u32, subclass: u32, programming_interface: u32) -> Self {
        return DiskDriver {
            class: class,
            subclass: subclass,
            programming_interface: programming_interface,
        };
    }

    /// write to disk from buffer
    /// lba: logical block address of sector
    /// offset: offset in bits
    /// buffer: buffer to get data to write from
    pub fn write(lba: usize, offset: u32, buffer: &[u8]) {
        

    }
    
    /// read from disk into buffer
    /// lba: logical block address of sector
    /// offset: offset in bits
    /// buffer: buffer to write data to
    pub fn read(lba: usize, offset: u32, buffer: &mut [u8]) {

    }

}





