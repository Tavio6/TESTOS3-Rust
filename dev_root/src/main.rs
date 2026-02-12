#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    use crate::vga_buffer::{WRITER, Color, ColorCode};

    {
        let mut w = WRITER.lock();
        w.color_code = ColorCode::new(Color::LightCyan, Color::Black);
        w.write_string("=== TESTOS3 BOOT SEQUENCE ===\n\n");
    }

    {
        let mut w = WRITER.lock();
        w.color_code = ColorCode::new(Color::Yellow, Color::Black);
        w.write_string("Hello World!! :3\n");
        w.write_string("All in working order!\n");
    }

    {
        let mut w = WRITER.lock();
        w.color_code = ColorCode::new(Color::LightGreen, Color::Black);
        w.write_string("Lorem ipsum dolor sit amet,\n");
        w.write_string("consectetur adipiscing elit.\n");
        w.write_string("Sed do eiusmod tempor incididunt ut labore.\n\n");
    }

    {
        let mut w = WRITER.lock();
        w.color_code = ColorCode::new(Color::Pink, Color::Black);
        w.write_string(">> SYSTEM READY <<\n");
    }

    loop {}
}