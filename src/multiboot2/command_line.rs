#[derive(Debug)]
#[repr(packed)]
pub struct CommandLineTag {
    typ: u32,
    size: u32,
    string: u8,
}

impl CommandLineTag {
    pub fn command_line(&self) -> &str {
        use core::{mem,str,slice};
        unsafe {
            let strlen = self.size as usize - mem::size_of::<CommandLineTag>();
            str::from_utf8_unchecked(
                slice::from_raw_parts((&self.string) as *const u8, strlen)
            )
        }
    }
}