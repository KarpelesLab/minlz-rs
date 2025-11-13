use crc::{Crc, CRC_32_ISCSI};

const CRC32C: Crc<u32> = Crc::<u32>::new(&CRC_32_ISCSI);

fn main() {
    let data = b"Testing without index\n";
    let c = CRC32C.checksum(data);
    println!("Raw CRC32C: 0x{:08x}", c);

    let manual = ((c >> 15) | (c << 17)).wrapping_add(0xa282ead8);
    println!("Manual bit ops: 0x{:08x}", manual);

    let rotate = c.rotate_right(15).wrapping_add(0xa282ead8);
    println!("Rotate right: 0x{:08x}", rotate);

    println!("\nExpected from Go: 0x668b4327");

    if manual == 0x668b4327 {
        println!("✓ Manual bit ops is CORRECT");
    }
    if rotate == 0x668b4327 {
        println!("✓ Rotate right is CORRECT");
    }
}
