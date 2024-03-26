use sdl2::pixels::Color;

pub const fn hex_rgb(hex: u32) -> Color {
    let r = ((hex >> 16) & 0xFF) as u8;
    let g = ((hex >> 8) & 0xFF) as u8;
    let b = (hex & 0xFF) as u8;

    Color::RGB(r, g, b)
}
