const CHARS: &[char] = &[
    '\u{0}',
    'A',
    'Ф',
    '\u{D7FF}',
    '\u{10FFFF}',
];

fn main() {
    afl::fuzz!(|data: &[u8]| {
        if let Ok(font) = ttf_parser::Font::from_data(data, 0) {
            for c in CHARS {
                let _ = font.glyph_index(*c);
            }
        }
    });
}