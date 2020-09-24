use volatile::Volatile;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar{
    character: u8,
    color: u8,
}

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; 80]; 25],
}

pub fn writer(s: &str) {
    let buffer = unsafe { &mut *(0xb8000 as *mut Buffer) };

    let row = 24;
    let mut col = 0;

    let color = 0xf;

    for char in s.bytes() {
        buffer.chars[row][col].write(ScreenChar {
            character: char,
            color: color,
        });

        col += 1;
    }
}
