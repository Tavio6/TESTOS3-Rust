use pc_keyboard::{layouts::Us104Key, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use spin::Mutex;
use x86_64::instructions::port::Port;
use crate::vga_buffer::{WRITER, Color, ColorCode};

lazy_static::lazy_static! {
    static ref KEYBOARD: Mutex<Keyboard<Us104Key, ScancodeSet1>> =
        Mutex::new(Keyboard::new(
            ScancodeSet1::new(),
            Us104Key,
            HandleControl::Ignore
        ));
    
    static ref INPUT_BUFFER: Mutex<InputBuffer> = Mutex::new(InputBuffer::new());
}

pub struct InputBuffer {
    buffer: [u8; 256],
    len: usize,
}

impl InputBuffer {
    const fn new() -> Self {
        Self {
            buffer: [0; 256],
            len: 0,
        }
    }

    pub fn push(&mut self, byte: u8) {
        if self.len < 256 {
            self.buffer[self.len] = byte;
            self.len += 1;
        }
    }

    pub fn pop(&mut self) {
        if self.len > 0 {
            self.len -= 1;
        }
    }

    pub fn clear(&mut self) {
        self.len = 0;
    }

    pub fn as_str(&self) -> &str {
        core::str::from_utf8(&self.buffer[..self.len]).unwrap_or("")
    }
}

pub fn handle_keyboard_interrupt() {
    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };
    let mut keyboard = KEYBOARD.lock();
    
    if let Ok(Some(event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(event) {
            match key {
                DecodedKey::Unicode(c) => {
                    let mut writer = WRITER.lock();
                    writer.color_code = ColorCode::new(Color::White, Color::Black);
                    
                    match c {
                        '\n' => {
                            drop(writer);
                            handle_command();
                        }
                        '\x08' => {
                            let mut input = INPUT_BUFFER.lock();
                            if input.len > 0 {
                                input.pop();
                                writer.backspace();
                            }
                        }
                        _ => {
                            let mut input = INPUT_BUFFER.lock();
                            input.push(c as u8);
                            drop(input);
                            writer.write_byte(c as u8);
                        }
                    }
                }
                DecodedKey::RawKey(_) => {}
            }
        }
    }
}

fn handle_command() {
    let mut input = INPUT_BUFFER.lock();
    let command = input.as_str().trim();
    
    let mut writer = WRITER.lock();
    writer.write_byte(b'\n');
    
    match command {
        "help" => {
            writer.color_code = ColorCode::new(Color::LightCyan, Color::Black);
            writer.write_string("Available commands:\n");
            writer.write_string("  help    - Show this message\n");
            writer.write_string("  clear   - Clear screen\n");
            writer.write_string("  echo    - Echo text\n");
            writer.write_string("  about   - About TESTOS3\n");
        }
        "clear" => {
            for row in 0..25 {
                writer.clear_row(row);
            }
            writer.column_position = 0;
        }
        "about" => {
            writer.color_code = ColorCode::new(Color::Yellow, Color::Black);
            writer.write_string("TESTOS3 - A simple OS written in Rust by Tavi_o6 :D\n");
        }
        cmd if cmd.starts_with("echo ") => {
            writer.color_code = ColorCode::new(Color::LightGreen, Color::Black);
            writer.write_string(&cmd[5..]);
            writer.write_byte(b'\n');
        }
        "" => {}
        _ => {
            writer.color_code = ColorCode::new(Color::LightRed, Color::Black);
            writer.write_string("Unknown command: ");
            writer.write_string(command);
            writer.write_string("\nType 'help' for available commands.\n");
        }
    }
    
    input.clear();
    drop(input);
    drop(writer);
    
    print_prompt();
}

pub fn print_prompt() {
    let mut writer = WRITER.lock();
    writer.color_code = ColorCode::new(Color::LightGreen, Color::Black);
    writer.write_string("> ");
}