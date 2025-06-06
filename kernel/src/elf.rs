#![allow(unused)]

//! ELF executables

use alloc::fmt::format;
use alloc::string::String;

use goblin::elf::section_header::SHT_SYMTAB;

#[cfg(target_arch = "aarch64")]
pub use goblin::elf64::{header, program_header, section_header, sym};

/// ELF
pub struct Elf<'a> {
    pub data: &'a [u8],
    header: &'a header::Header,
}

impl<'a> Elf<'a> {
    pub fn from(data: &'a [u8]) -> Result<Elf<'a>, String> {
        if data.len() < header::SIZEOF_EHDR {
            Err(format!(
                "Elf: not enough data: {} < {}",
                data.len(),
                header::SIZEOF_EHDR
            ))
        } else if &data[..header::SELFMAG] != header::ELFMAG {
            Err(format!(
                "Elf: Invalid magic: {:?} != {:?}",
                &data[..header::SELFMAG],
                header::ELFMAG
            ))
        } else if data.get(header::EI_CLASS) != Some(&header::ELFCLASS) {
            Err(format!(
                "Elf: Invalid architecture: {:?} != {:?}",
                data.get(header::EI_CLASS),
                header::ELFCLASS
            ))
        } else {
            Ok(Elf {
                data,
                header: unsafe { &*(data.as_ptr() as *const header::Header) },
            })
        }
    }

    pub fn sections(&'a self) -> ElfSections<'a> {
        ElfSections {
            data: self.data,
            header: self.header,
            i: 0,
        }
    }

    pub fn symbols(&'a self) -> Option<ElfSymbols<'a>> {
        let mut symtab_opt = None;
        for section in self.sections() {
            if section.sh_type == SHT_SYMTAB {
                symtab_opt = Some(section);
                break;
            }
        }
        if let Some(symtab) = symtab_opt {
            Some(ElfSymbols {
                data: self.data,
                symtab,
                i: 0,
            })
        } else {
            None
        }
    }
}

pub struct ElfSections<'a> {
    data: &'a [u8],
    header: &'a header::Header,
    i: usize,
}

impl<'a> Iterator for ElfSections<'a> {
    type Item = &'a section_header::SectionHeader;
    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self.header.e_shnum as usize {
            let item = unsafe {
                &*((self.data.as_ptr() as usize
                    + self.header.e_phoff as usize
                    + self.i * self.header.e_phentsize as usize)
                    as *const section_header::SectionHeader)
            };
            self.i += 1;
            Some(item)
        } else {
            None
        }
    }
}

pub struct ElfSegments<'a> {
    data: &'a [u8],
    header: &'a header::Header,
    i: usize,
}

impl<'a> Iterator for ElfSegments<'a> {
    type Item = &'a program_header::ProgramHeader;
    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self.header.e_phnum as usize {
            let item = unsafe {
                &*((self.data.as_ptr() as usize
                    + self.header.e_phoff as usize
                    + self.i * self.header.e_phentsize as usize)
                    as *const program_header::ProgramHeader)
            };
            self.i += 1;
            Some(item)
        } else {
            None
        }
    }
}

pub struct ElfSymbols<'a> {
    data: &'a [u8],
    symtab: &'a section_header::SectionHeader,
    i: usize,
}

impl<'a> Iterator for ElfSymbols<'a> {
    type Item = &'a sym::Sym;
    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self.symtab.sh_size as usize / sym::SIZEOF_SYM {
            let item = unsafe {
                &*((self.data.as_ptr() as usize
                    + self.symtab.sh_offset as usize
                    + self.i * sym::SIZEOF_SYM) as *const sym::Sym)
            };
            self.i += 1;
            Some(item)
        } else {
            None
        }
    }
}
