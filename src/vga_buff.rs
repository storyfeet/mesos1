
#[allow(dead_code)]
#[derive(Debug,Copy,Clone,PartialEq,Eq)]
#[repr(u8)]
pub enum Color{
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub struct ColorCode(u8);

impl ColorCode{
    pub fn new(fg:Color,bg:Color)->ColorCode{
        ColorCode((bg as u8) << 4 | fg as u8)
    }
}

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
#[repr(C)]
struct ScreenChar{
    c:u8,
    color:ColorCode,
}

const BUF_WIDTH:usize = 80;
const BUF_HEIGHT:usize = 25;

struct Buffer{
    chars:[[ScreenChar;BUF_WIDTH];BUF_HEIGHT],
}

pub struct Writer{
    column_pos:usize,
    color_code:ColorCode,
    buffer:&'static mut Buffer,
}

impl Writer{
    pub fn new()->Self{
        Writer{
            color_code:ColorCode::new(Color::Yellow,Color::Green),
            column_pos:0,
            buffer: unsafe{&mut *(0xb8000 as *mut Buffer)},
        }

    }

    pub fn write_byte(&mut self,b:u8){
        match b {
            b'\n'=>self.new_line(),
            b_val=>{
                if self.column_pos >= BUF_WIDTH {
                    self.new_line();
                }
                let row = BUF_HEIGHT - 1;
                let col = self.column_pos;

                self.buffer.chars[row][col] = ScreenChar{
                    c:b,
                    color:self.color_code,
                };
                self.column_pos += 1;
            }
        }
    }

    pub fn write_str(&mut self,s:&str){
        for c in s.bytes(){
            match c{
                0x20...0x7e | b'\n' => self.write_byte(c),
                _other => self.write_byte(0xfe),
            }
        }
    }

    pub fn new_line(&mut self){}
}
