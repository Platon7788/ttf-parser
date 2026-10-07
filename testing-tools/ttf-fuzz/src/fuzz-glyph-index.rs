#![no_main]

const CHARS: &[char] = &['\u{0}', 'A', 'Ф', '0', '\u{D7FF}', '\u{10FFFF}'];

libfuzzer_sys::fuzz_target!(|data: &[u8]| {
    if let Ok(face) = ttf_parser::Face::parse(data, 0) {
        for c in CHARS {
            let _ = face.glyph_index(*c);
        }
    }
});
