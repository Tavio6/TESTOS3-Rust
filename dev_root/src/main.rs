#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
mod vga_buffer;
mod keyboard;
use core::panic::PanicInfo;
use x86_64::structures::idt::InterruptDescriptorTable;
use crate::keyboard::handle_keyboard_interrupt;

lazy_static::lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt[33].set_handler_fn(keyboard_interrupt_handler);
        idt.double_fault.set_handler_fn(double_fault_handler);
        idt
    };
}

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
        w.write_string(">> SYSTEM READY <<\n\n");
    }
    
    unsafe {
        init_pic();
    }
    
    IDT.load();
    x86_64::instructions::interrupts::enable();
    
    println!("Type 'help' for available commands.\n");
    crate::keyboard::print_prompt();
    
    loop {
        x86_64::instructions::hlt();
    }
}

unsafe fn init_pic() {
    use x86_64::instructions::port::Port;
    
    let mut pic1_cmd = Port::new(0x20);
    let mut pic1_data = Port::new(0x21);
    let mut pic2_cmd = Port::new(0xA0);
    let mut pic2_data = Port::new(0xA1);
    
    pic1_cmd.write(0x11u8);
    pic2_cmd.write(0x11u8);
    
    pic1_data.write(32u8);
    pic2_data.write(40u8);
    
    pic1_data.write(4u8);
    pic2_data.write(2u8);
    
    pic1_data.write(0x01u8);
    pic2_data.write(0x01u8);
    
    pic1_data.write(0xFDu8);
    pic2_data.write(0xFFu8);
}

extern "x86-interrupt" fn keyboard_interrupt_handler(
    _stack_frame: x86_64::structures::idt::InterruptStackFrame
) {
    handle_keyboard_interrupt();
    unsafe {
        let mut port = x86_64::instructions::port::Port::new(0x20);
        port.write(0x20u8);
    }
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: x86_64::structures::idt::InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("DOUBLE FAULT\n{:#?}", stack_frame);
}