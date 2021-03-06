use core::fmt;

use self::header::{Tag, TagIter};
pub use self::boot_loader_name::BootLoaderNameTag;
pub use self::elf_sections::{ElfSectionsTag, ElfSection, ElfSectionIter, ElfSectionType, ElfSectionFlags, StringTable};
pub use self::elf_sections::{ELF_SECTION_WRITABLE, ELF_SECTION_ALLOCATED, ELF_SECTION_EXECUTABLE};
pub use self::memory_map::{MemoryMapTag, MemoryArea, MemoryAreaIter};
pub use self::module::{ModuleTag, ModuleIter};
pub use self::command_line::CommandLineTag;

mod header;
mod boot_loader_name;
mod elf_sections;
mod memory_map;
mod module;
mod command_line;

pub unsafe fn load(address: usize) -> &'static BootInformation {
    if !cfg!(test) {
        assert_eq!(0, address & 0b111);
    }
    let multiboot = &*(address as *const BootInformation);
    assert_eq!(0, multiboot.total_size & 0b111);
    assert!(multiboot.has_valid_end_tag());
    multiboot
}

#[repr(C)]
pub struct BootInformation {
    pub total_size: u32,
    _reserved: u32,
    first_tag: Tag,
}

impl BootInformation {
    pub fn start_address(&self) -> usize {
        self as *const _ as usize
    }

    pub fn end_address(&self) -> usize {
        self.start_address() + self.total_size()
    }

    pub fn total_size(&self) -> usize {
        self.total_size as usize
    }

    pub fn elf_sections_tag(&self) -> Option<&'static ElfSectionsTag> {
        self.get_tag(9).map(|tag| unsafe { &*(tag as *const Tag as *const ElfSectionsTag) })
    }

    pub fn memory_map_tag(&self) -> Option<&'static MemoryMapTag> {
        self.get_tag(6).map(|tag| unsafe { &*(tag as *const Tag as *const MemoryMapTag) })
    }

    pub fn module_tags(&self) -> ModuleIter {
        module::module_iter(self.tags())
    }

    pub fn boot_loader_name_tag(&self) -> Option<&'static BootLoaderNameTag> {
        self.get_tag(2).map(|tag| unsafe { &*(tag as *const Tag as *const BootLoaderNameTag) })
    }

    pub fn command_line_tag(&self) -> Option<&'static CommandLineTag> {
        self.get_tag(1).map(|tag| unsafe { &*(tag as *const Tag as *const CommandLineTag)})
    }

    fn has_valid_end_tag(&self) -> bool {
        const END_TAG: Tag = Tag{typ:0, size:8};

        let self_ptr = self as *const _;
        let end_tag_addr = self_ptr as usize + (self.total_size - END_TAG.size) as usize;
        let end_tag = unsafe{&*(end_tag_addr as *const Tag)};

        end_tag.typ == END_TAG.typ && end_tag.size == END_TAG.size
    }

    fn get_tag(&self, typ: u32) -> Option<&'static Tag> {
        self.tags().find(|tag| tag.typ == typ)
    }

    fn tags(&self) -> TagIter {
        TagIter { current: &self.first_tag as *const _ }
    }
}

impl fmt::Debug for BootInformation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "multiboot information")?;

        writeln!(f, "S: {:#010X}, E: {:#010X}, L: {:#010X}",
            self.start_address(), self.end_address(), self.total_size())?;

        if let Some(boot_loader_name_tag) = self.boot_loader_name_tag() {
            writeln!(f, "boot loader name: {}", boot_loader_name_tag.name())?;
        }

        if let Some(command_line_tag) = self.command_line_tag() {
            writeln!(f, "command line: {}", command_line_tag.command_line())?;
        }

        if let Some(memory_map_tag) = self.memory_map_tag() {
            writeln!(f, "memory areas:")?;
            for area in memory_map_tag.memory_areas() {
                writeln!(f, "    S: {:#010X}, E: {:#010X}, L: {:#010X}",
                    area.start_address(), area.end_address(), area.size())?;
            }
        }

        if let Some(elf_sections_tag) = self.elf_sections_tag() {
            let string_table = elf_sections_tag.string_table();
            writeln!(f, "kernel sections:")?;
            for s in elf_sections_tag.sections() {
                writeln!(f, "    name: {:15}, S: {:#08X}, E: {:#08X}, L: {:#08X}, F: {:#04X}",
                    string_table.section_name(s), s.start_address(),
                    s.start_address() + s.size(), s.size(), s.flags().bits())?;
            }
        }

        writeln!(f, "module tags:")?;
        for mt in self.module_tags() {
            writeln!(f, "    name: {:15}, S: {:#010X}, E: {:#010X}",
                mt.name(), mt.start_address(), mt.end_address())?;
        }

        Ok(())
    }
}