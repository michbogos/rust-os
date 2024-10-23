use lazy_static::lazy_static;
use spin::Mutex;
use x86_64::instructions::port::PortGeneric;
use x86_64::structures::port::{PortRead, PortWrite};
use core::fmt;
use core;
use core::ops::Div;

pub struct VGAAdress{
    adress:*mut u16,
}

pub struct VGABuffer{
    pub size:usize,
    pub idx:usize,
    pub adress:VGAAdress,
}

unsafe impl Send for VGAAdress {}

// Possibly use volatile
impl VGABuffer{
    pub fn print(&mut self, msg:&str, fg:VGACol, bg:VGACol){
        for c in msg.bytes(){
            if c == '\n' as u8{
                self.idx += 80-(self.idx%80);
            }
            else{
                unsafe {
                    *self.adress.adress.offset(self.idx as isize)=(vga_char(c, fg, bg)) as u16;
                }
                self.idx=(self.idx+1)%self.size;
            }
        }
    }
}


impl fmt::Write for VGABuffer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.print(s, VGACol::LWHITE, VGACol::BLACK);
        update_cursor((self.idx%80) as i32, -1+(self.idx/80) as i32);
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(||{VGA.lock().write_fmt(args).unwrap()});
}

lazy_static!{
pub static ref VGA: Mutex<VGABuffer> = Mutex::new(VGABuffer {size:25*80,idx:0,adress:VGAAdress{adress:0xb8000 as *mut u16}});
}

#[derive(Debug, Copy, Clone)]
pub enum VGACol {
    BLACK,
    BLUE,
    GREEN,
    CYAN,
    RED,
    PINK,
    YELLOW,
    WHITE,
    LBLACK,
    LBLUE,
    LGREEN,
    LCYAN,
    LRED,
    LPINK,
    LYELLOW,
    LWHITE
}

pub fn vga_char(symbol:u8, fg:VGACol, bg:VGACol)->u16{
    let mut res:u16 = symbol as u16;
    res = res|(((fg as u8) as u16)<<8) as u16;
    res = res|(((bg as u8) as u16)<<12) as u16;
    return res;
}


pub fn enable_cursor(start: u8, end:u8){
    use x86_64::instructions::port::Port;
    let mut x3D4: Port<u8> = Port::new(0x3D4);
    let mut x3D5: Port<u8> = Port::new(0x3D5);
    unsafe{
        x3D4.write(0x0A);
        let mut x3D5_val = x3D5.read();
        x3D5.write((x3D5_val & 0xC0) | start);

        x3D4.write(0x0B);
        x3D5_val = x3D5.read();
        x3D5.write((x3D5_val & 0xE0) | end);

    }
}

pub fn update_cursor(x:i32, y:i32){
    use x86_64::instructions::port::Port;
    let mut x3D4: Port<u8> = Port::new(0x3D4);
    let mut x3D5: Port<u8> = Port::new(0x3D5);
	let pos:u16 = (y * 80 + x) as u16;

    unsafe{
	    x3D4.write(0x0F);
	    x3D5.write((pos & 0xFF) as u8);
	    x3D4.write(0x0E);
	    x3D5.write(((pos >> 8) & 0xFF) as u8);
    }
}

#[test_case]
fn test_println_simple() {
    println!("test_println_simple output");
}

#[test_case]
fn test_println_many() {
    for _ in 0..200 {
        println!("test_println_many output");
    }
}