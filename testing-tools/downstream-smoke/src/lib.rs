#[cfg(test)]
mod tests {
    use ab_glyph::{Font, FontRef, FontVec, ScaleFont};
    use owned_ttf_parser::AsFaceRef;

    #[test]
    fn existing_font_wrappers_use_the_fork_without_api_changes() {
        let bytes = include_bytes!("../../../tests/fonts/demo.ttf");
        let direct = ttf_parser::Face::parse(bytes, 0).unwrap();
        let owned = owned_ttf_parser::OwnedFace::from_vec(bytes.to_vec(), 0).unwrap();
        let borrowed = FontRef::try_from_slice(bytes).unwrap();
        let font_vec = FontVec::try_from_vec(bytes.to_vec()).unwrap();
        for c in ['A', 'B', '\0', '\u{10ffff}'] {
            let expected = direct.glyph_index(c);
            assert_eq!(owned.as_face_ref().glyph_index(c), expected);
            let id = expected.map_or(0, |id| id.0);
            assert_eq!(borrowed.glyph_id(c).0, id);
            assert_eq!(font_vec.glyph_id(c).0, id);
        }
        let id = borrowed.glyph_id('A');
        assert!(borrowed.outline_glyph(id.with_scale(24.0)).is_some());
        assert!(borrowed.as_scaled(24.0).h_advance(id) > 0.0);
        assert_eq!(borrowed.units_per_em(), font_vec.units_per_em());
    }
}
