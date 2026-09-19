const MAGIC: u32 = 0x1BADB002; // Multiboot v1
const FLAGS: u32 = 0;
const CHECKSUM: u32 = 0u32.wrapping_sub(MAGIC + FLAGS);

#[repr(C, align(8))]
pub struct Multiboot {
    magic: u32,
    flags: u32,
    checksum: u32,
}

#[used]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".multiboot")]
pub static HEADER: Multiboot = Multiboot {
    magic: MAGIC,
    flags: FLAGS,
    checksum: CHECKSUM,
};
