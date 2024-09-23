//! The purpose of this file is to provide functionality
//! to interact with a FAT 32 filesystem.

/*
Research gathered about the FAT 32 filesystem:
Reference: https://www.pjrc.com/tech/8051/ide/fat32.html

The first step is to read the first sector (Volume ID) of the FAT32 filesystem


Directory data is organized in 32 byte records,
meaning each sector holds exactly 16 records

*/

use core::convert::TryInto;

/// structure for a single directory record
struct DirectoryRecord {
    short_filename: [u8; 11],
    attrib_byte: u8,
    cluster_high: u16,
    cluster_low: u16,
    file_size: u32,
}

impl DirectoryRecord {
    /// constructor
    pub fn new(
        short_filename: &[u8; 11],
        attrib_byte: u8,
        cluster_high: u16,
        cluster_low: u16,
        file_size: u32,
    ) -> Self {
        return DirectoryRecord {
            short_filename: *short_filename,
            attrib_byte: attrib_byte,
            cluster_high: cluster_high,
            cluster_low: cluster_low,
            file_size: file_size,
        };
    }

    /// constructor based on 32 byte record
    pub fn create_from_buffer(buffer: &[u8; 32]) -> Self {
        return Self::new(
            &buffer[0..11].try_into().expect("Somehow 11 is not 11..."),
            buffer[0x0B],
            ((buffer[0x14] as u16) << 8) | (buffer[0x15] as u16),
            ((buffer[0x1A] as u16) << 8) | (buffer[0x1B] as u16),
            ((buffer[0x1C] as u32) << 24)
                | ((buffer[0x1D] as u32) << 16)
                | ((buffer[0x1E] as u32) << 8)
                | (buffer[0x1F] as u32),
        );
    }
}

/// number of bytes expected per sector
const EXPECTED_BYTES_PER_SECTOR: u16 = 512;
/// number of FATS expected per filesystem
const EXPECTED_FATS: u8 = 2;
/// signature expected at the end of VolumeID sector
const EXPECTED_SIGNATURE: u16 = 0xAA55;

/// Structure to store FAT 32 filesystem information in memory
pub struct Fat32 {
    first_sector: usize, // lba address of the first sector
    bytes_per_sector: u16,
    sectors_per_cluster: u8,
    reserved_sectors: u16,
    fats: u8,
    sectors_per_fat: u32,
    root_cluster: u32,
    signature: u16,
}

/*
// FAT32 VolumeID offsets
const FAT_32_OFFSETS: Fat32 = Fat32 {
    first_sector: 0, // not a real offset (just a placeholder)
    bytes_per_sector: 0x0B,
    sectors_per_cluster: 0x0D,
    reserved_sectors: 0x0E,
    fats: 0x10,
    sectors_per_fat: 0x24,
    root_cluster: 0x2C,
    signature: 0x1FE,
};

// FAT32 VolumeID sizes (in bits)
const FAT_32_SIZES: Fat32 = Fat32 {
    first_sector: 0,
    bytes_per_sector: 16,
    sectors_per_cluster: 8,
    reserved_sectors: 16,
    fats: 8,
    sectors_per_fat: 32,
    root_cluster: 32,
    signature: 16,
};
*/

impl Fat32 {
    /// constructor
    pub fn new(
        first_sector: usize,
        bytes_per_sector: u16,
        sectors_per_cluster: u8,
        reserved_sectors: u16,
        fats: u8,
        sectors_per_fat: u32,
        root_cluster: u32,
        signature: u16,
    ) -> Self {
        return Fat32 {
            first_sector: first_sector,
            bytes_per_sector: bytes_per_sector,
            sectors_per_cluster: sectors_per_cluster,
            reserved_sectors: reserved_sectors,
            fats: fats,
            sectors_per_fat: sectors_per_fat,
            root_cluster: root_cluster,
            signature: signature,
        };
    }

    /// function to check bytes_per_sector, fats, and signature
    /// return true if the filesystem is valid, false otherwise
    pub fn check_filesystem(&self) -> bool {
        println!("Checking Fat32 filesystem...");
        // check the bytes per sector
        if self.bytes_per_sector != EXPECTED_BYTES_PER_SECTOR {
            println!(
                "FAT32 filesystem has {} bytes per sector. Expected {}",
                self.bytes_per_sector, EXPECTED_BYTES_PER_SECTOR
            );
            return false;
        }
        // check the number of fats
        if self.fats != EXPECTED_FATS {
            println!(
                "FAT32 filesystem has {} FATS. Expected {}",
                self.fats, EXPECTED_FATS
            );
            return false;
        }
        // check the signature
        if self.signature != EXPECTED_SIGNATURE {
            println!(
                "FAT32 filesystem's signature: {:#x}. Expected {:#x}",
                self.signature, EXPECTED_SIGNATURE
            );
            return false;
        }
        println!("FAT32 filesystem is valid!");
        return true;
    }

    /// function to calculate lba address of a cluster
    /// cluster counting starts at 2
    pub fn get_cluster_address(&self, cluster: u32) -> usize {
        return self.first_sector + (((cluster - 2) * (self.sectors_per_cluster as u32)) as usize);
    }

    /// function to list the root directory
    pub fn list_root(&self) {
        // first, get the cluster address of the root sector
        let root_address: usize = self.get_cluster_address(self.root_cluster);

        // read 'self.sectors_per_cluster' sector using lba address
    }
}

pub fn test() {
    println!("Testing Fat32.");
    let test_fat = Fat32::new(0, 512, 3, 0x20, 2, 20, 0x2, 0xAA55);

    test_fat.check_filesystem();
}
