use ttf_parser::{Face, GlyphId, OutlineBuilder, RawFaceTables, Rect};

struct Sink;
impl OutlineBuilder for Sink {
    fn move_to(&mut self, _: f32, _: f32) {}
    fn line_to(&mut self, _: f32, _: f32) {}
    fn quad_to(&mut self, _: f32, _: f32, _: f32, _: f32) {}
    fn curve_to(&mut self, _: f32, _: f32, _: f32, _: f32, _: f32, _: f32) {}
    fn close(&mut self) {}
}

fn exercise(data: &[u8]) {
    let _ = ttf_parser::fonts_in_collection(data);
    let Ok(face) = Face::parse(data, 0) else {
        return;
    };
    #[cfg(feature = "variable-fonts")]
    let mut face = face;
    let _ = face.height();
    let _ = face.vertical_height();
    let r = face.global_bounding_box();
    let _ = (r.width(), r.height());
    for c in ['\0', 'A', 'Ф', '\u{d7ff}', '\u{e000}', '\u{10ffff}'] {
        let _ = face.glyph_index(c);
    }
    #[cfg(feature = "variable-fonts")]
    for value in [f32::MIN, -1.0, 0.0, f32::MAX, f32::NAN, f32::INFINITY] {
        let _ = face.set_variation(ttf_parser::Tag::from_bytes(b"wght"), value);
    }
    for id in [0, 1, 2, face.number_of_glyphs().saturating_sub(1), u16::MAX] {
        let id = GlyphId(id);
        let _ = face.outline_glyph(id, &mut Sink);
        let _ = face.glyph_bounding_box(id);
        let _ = face.glyph_hor_advance(id);
        let _ = face.glyph_hor_side_bearing(id);
        let _ = face.glyph_raster_image(id, u16::MAX);
        let _ = face.glyph_svg_image(id);
        #[cfg(feature = "glyph-names")]
        let _ = face.glyph_name(id);
    }
}

#[test]
fn rectangle_extremes_are_profile_independent() {
    let r = Rect {
        x_min: i16::MIN,
        y_min: i16::MIN,
        x_max: i16::MAX,
        y_max: i16::MAX,
    };
    assert_eq!(r.width(), i16::MAX);
    assert_eq!(r.height(), i16::MAX);
    let inverted = Rect {
        x_min: i16::MAX,
        y_min: i16::MAX,
        x_max: i16::MIN,
        y_max: i16::MIN,
    };
    assert_eq!(inverted.width(), i16::MIN);
    assert_eq!(inverted.height(), i16::MIN);
}

#[test]
fn face_metric_extremes_saturate() {
    let mut head = [0u8; 54];
    head[18..20].copy_from_slice(&1000u16.to_be_bytes());
    let mut metrics = [0u8; 36];
    metrics[..4].copy_from_slice(&0x0001_0000u32.to_be_bytes());
    metrics[4..6].copy_from_slice(&i16::MAX.to_be_bytes());
    metrics[6..8].copy_from_slice(&i16::MIN.to_be_bytes());
    let maxp = [0, 0, 0x50, 0, 0, 1];
    let face = Face::from_raw_tables(RawFaceTables {
        head: &head,
        hhea: &metrics,
        maxp: &maxp,
        vhea: Some(&metrics),
        ..Default::default()
    })
    .unwrap();
    assert_eq!(face.height(), i16::MAX);
    assert_eq!(face.vertical_height(), Some(i16::MAX));
}

#[test]
fn normalized_coordinates_accept_nonfinite_inputs_without_panicking() {
    for (input, expected) in [
        (f32::NAN, 0),
        (f32::INFINITY, 16384),
        (f32::NEG_INFINITY, -16384),
        (2.0, 16384),
        (-2.0, -16384),
        (0.5, 8192),
    ] {
        assert_eq!(
            ttf_parser::NormalizedCoordinate::from(input).get(),
            expected
        );
    }
}

#[test]
fn windows_metrics_use_unsigned_font_values() {
    let mut data = [0u8; 78];
    for raw in 0..=u16::MAX {
        data[74..76].copy_from_slice(&raw.to_be_bytes());
        data[76..78].copy_from_slice(&raw.to_be_bytes());
        let table = ttf_parser::os2::Table::parse(&data).unwrap();
        assert_eq!(
            i32::from(table.windows_ascender()),
            i32::from(raw).min(i32::from(i16::MAX))
        );
        assert_eq!(
            i32::from(table.windows_descender()),
            (-i32::from(raw)).max(i32::from(i16::MIN))
        );
    }
}

#[test]
fn public_lazy_arrays_have_bounded_counts_and_accept_zero_sized_types() {
    let bytes = vec![0; 65536];
    let array = ttf_parser::LazyArray16::<u8>::new(&bytes);
    assert_eq!(array.len(), u16::MAX);
    assert_eq!(array.last(), Some(0));
    assert_eq!(ttf_parser::LazyArray16::<()>::new(&[]).len(), 0);
    assert_eq!(ttf_parser::LazyArray32::<()>::new(&[]).len(), 0);
}

#[test]
fn hostile_font_smoke() {
    let seeds: &[&[u8]] = &[
        include_bytes!("fonts/demo.ttf"),
        include_bytes!("fonts/colr_1.ttf"),
        include_bytes!("fonts/colr_1_variable.ttf"),
        include_bytes!("fonts/bitmap.otb"),
    ];
    for seed in seeds {
        exercise(seed);
        for len in (0..seed.len()).step_by((seed.len() / 128).max(1)) {
            exercise(&seed[..len]);
        }
        for i in 0..256 {
            let mut bytes = seed.to_vec();
            let offset = (i * 7919) % bytes.len();
            bytes[offset] ^= 0xff;
            exercise(&bytes);
        }
    }
    for len in 0..256 {
        exercise(&vec![0xff; len]);
        exercise(&vec![0; len]);
    }
}
