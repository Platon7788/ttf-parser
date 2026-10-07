#![no_main]

libfuzzer_sys::fuzz_target!(|data: &[u8]| {
    if let Ok(face) = ttf_parser::Face::parse(data, 0) {
        // One sampled glyph per input bounds harness work independently of glyph count.
        let id = data.last().copied().map(u16::from).unwrap_or(0)
            | (u16::from(data.first().copied().unwrap_or(0)) << 8);
        for id in [0, id, face.number_of_glyphs().saturating_sub(1)] {
            let _ = face.outline_glyph(ttf_parser::GlyphId(id), &mut Builder(0));
        }
    }
});

struct Builder(usize);

impl ttf_parser::OutlineBuilder for Builder {
    #[inline]
    fn move_to(&mut self, _: f32, _: f32) {
        self.0 += 1;
    }

    #[inline]
    fn line_to(&mut self, _: f32, _: f32) {
        self.0 += 1;
    }

    #[inline]
    fn quad_to(&mut self, _: f32, _: f32, _: f32, _: f32) {
        self.0 += 2;
    }

    #[inline]
    fn curve_to(&mut self, _: f32, _: f32, _: f32, _: f32, _: f32, _: f32) {
        self.0 += 3;
    }

    #[inline]
    fn close(&mut self) {
        self.0 += 1;
    }
}
