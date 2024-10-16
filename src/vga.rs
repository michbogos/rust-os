pub struct VGABuffer{
    pub size:usize,
    pub idx:usize,
    pub adress:* mut u16,
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
    L_BLACK,
    L_BLUE,
    L_GREEN,
    L_CYAN,
    L_RED,
    L_PINK,
    L_YELLOW,
    L_WHITE
}

pub fn vga_char(symbol:u8, fg:VGACol, bg:VGACol)->u16{
    let mut res:u16 = symbol as u16;
    res = res|(((fg as u8) as u16)<<8) as u16;
    res = res|(((bg as u8) as u16)<<12) as u16;
    return res;
}

pub fn print(buffer:  &mut VGABuffer, msg:&str, fg:VGACol, bg:VGACol){
    for (c) in msg.bytes(){
        unsafe {
            *buffer.adress.offset(buffer.idx as isize)=(vga_char(c, fg, bg)) as u16;
        }
        buffer.idx+= 1;
    }
}