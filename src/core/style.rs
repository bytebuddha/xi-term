use tui::style::Color;

pub fn u32_to_color(argb_color: u32) -> Color {
    let r = ((argb_color & 0x00ff_0000) >> 16) as u8;
    let g = ((argb_color & 0x0000_ff00) >> 8) as u8;
    let b = (argb_color & 0x0000_00ff) as u8;
    Color::Rgb(r, g, b)
}
