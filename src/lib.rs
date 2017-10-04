#![feature(lang_items)]
#![feature(const_fn)]
#![feature(unique)]
#![feature(const_unique_new)]
#![no_std]
extern crate rlibc;
extern crate volatile;
extern crate spin;

#[macro_use]
extern crate bitflags;

#[macro_use]
mod vga_buffer;
mod multiboot2;

#[no_mangle]
pub extern fn rust_main() {
    vga_buffer::clear_screen();
    for i in 0..10000 {
        println!("{}", i);
    }

    loop{}
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! {loop{}}