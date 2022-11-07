#![allow(non_camel_case_types)]
#![allow(dead_code)] // Only for development; TODO: Look up how to do optional compilation

// Object File Format
// -----------------------------
// [ELF header]                  # DOING
// -----------------------------
// [Program header table] Option # TODO
// -----------------------------
// [Section / Segment]*          # TODO
// -----------------------------
// [Section header table] Option # TODO


// Data Representation with expected C alignment
type Elf32_Addr  = u32; // Unsigned program address
type Elf32_Half  = u16; // Unsigned medium integer
type Elf32_Off   = u32; // Unsigned file offset
type Elf32_Sword = i32; // Signed large integer
type Elf32_Word  = u32; // Unsigned large integer
type Elf32_Byte  = u8;  // Unsigned small integer

// ELF HEADER
// SPECIFICATION
pub const EI_NIDENT: usize = 16;

// e_ident specifications
// [0][1][2][3][4][5][6][7][8][9][A][B][C][D][E][F]

// [0-3]; magic value
const ELFMAG0: u8 = 0x7f;
const ELFMAG1: u8 = 'E' as u8;
const ELFMAG2: u8 = 'L' as u8;
const ELFMAG3: u8 = 'F' as u8;

// [4]; EI_Class
enum FileClass {
    ELFCLASSNONE, // 0, Invalid Class
    ELFCLASS32,   // 1, 32-bit Objects
    ELFCLASS64,   // 2, 64-bit Objects
}

enum DataEncoding {
    ELFDATANONE, // Invalid data encoding
    ELFDATA2LSB, // Little Endian
    ELFDATA2MSB, // Big Endian
}

// e_type specification

enum FileType {
    ET_NONE,   // No file type
    ET_REL,    // Relocatable file
    ET_DYN,    // Shared object file
    ET_CORE,   // Core file, unspecified, reserved
    ET_LOPROC, // Processor-specific, lower bound, inclusive, 0xff00
    ET_HIPROC, // Processor-specific, upper bound, inclusive, 0xffff
}

// OBJECT
#[repr(C)]
pub struct Elf32_Ehdr {
		pub e_ident:     [u8; EI_NIDENT], // DOING
		pub e_type:      Elf32_Half,      // TODO
		pub e_machine:   Elf32_Half,      // TODO
		pub e_version:   Elf32_Word,      // TODO
		pub e_entry:     Elf32_Addr,      // TODO
		pub e_phoff:     Elf32_Off,       // TODO
		pub e_shoff:     Elf32_Off,       // TODO
		pub e_flags:     Elf32_Word,      // TODO
		pub e_ehsize:    Elf32_Half,      // TODO
		pub e_phentsize: Elf32_Half,      // TODO
		pub e_ph_num:    Elf32_Half,      // TODO
		pub e_shentsize: Elf32_Half,      // TODO
		pub e_shnum:     Elf32_Half,      // TODO
		pub e_shstrndx:  Elf32_Half,      // TODO
}

// e_machine specification
// e_version specification
// e_entry specification
// e_phoff specification
// e_shoff specification
// e_flags specification
// e_ehsize specification
// e_phentsize specification
// e_phnum specification
// e_shentsize specification
// e_shnum specification
// e_shstrndx specification
