use crc::{Crc, CRC_32_ISCSI};

const CRC32C: Crc<u32> = Crc::<u32>::new(&CRC_32_ISCSI);

fn main() {
    let data = b"Testing without index\n";
    let c = CRC32C.checksum(data);
    println!("Raw CRC32C: 0x{:08x}", c);

    let result = c.rotate_right(15).wrapping_add(0xa282ead8);
    println!("Transformed CRC: 0x{:08x}", result);

    println!("\nExpected from Go: 0x668b4327");

    if result == 0x668b4327 {
        println!("✓ CRC transformation is CORRECT");
    } else {
        println!("✗ CRC transformation is INCORRECT");
    }
}
