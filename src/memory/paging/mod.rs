use memory::PAGE_SIZE;
use memory::Frame;

const ENTRY_COUNT: usize = 512;

pub type PhysicalAddress = usize;
pub type VirtualAddress = usize;

pub struct Page {
    number: usize,
}

impl Page {
    pub fn translate(virtual_address: VirtualAddress) -> Option<PhysicalAddress> {
        let offset = virtual_address % PAGE_SIZE;
        translate_page(Page::containing_address(virtual_address))
            .map(|frame| frame.number * PAGE_SIZE + offset)
    }

    pub fn containing_address(address: VirtualAddress) -> Page {
        assert!(address < 0x0000_8000_0000_0000 ||
            address >= 0xffff_8000_0000_0000,
            "invalid address: 0x{:x}", address);
        Page { number: address / PAGE_SIZE }
    }

    fn start_address(&self) -> usize {
        self.number * PAGE_SIZE
    }

    fn translate_page(page: Page) -> Option<Frame> {
        use self::entry::HUGE_PAGE;

        let p3 = unsafe { &*table::P4 }.next_table(page.p4_index());

        let huge_page = || {
            // TODO
        };

        p3.and_then(|p3| p3.next_table(page.p3_index()))
          .and_then(|p2| p2.next_table(page.p3_index()))
          .and_then(|p1| p1[page.p1_index()].pointed_frame())
          .or_else(huge_page)
    }

    fn p4_index(&self) -> usize {
        (self.number >> 27) & 0o777
    }

    fn p3_index(&self) -> usize {
        (self.number >> 18) & 0o777
    }

    fn p2_index(&self) -> usize {
        (self.number >> 9) & 0o777
    }

    fn p1_index(&self) -> usize {
        (self.number >> 0) & 0o777
    }
}