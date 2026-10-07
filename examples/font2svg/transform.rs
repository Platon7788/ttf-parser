use ttf_parser::Transform;

/// Resolve the gradient into outline coordinates using a finite affine inverse.
/// A singular outline has no invertible coordinate system and must not emit a gradient.
pub fn paint_transform(outline: Transform, gradient: Transform) -> Option<Transform> {
    let [a, b, c, d, e, f] = [
        outline.a, outline.b, outline.c, outline.d, outline.e, outline.f,
    ]
    .map(f64::from);
    let determinant = a * d - b * c;
    if determinant == 0.0 || !determinant.is_finite() {
        return None;
    }
    let inverse = [
        d / determinant,
        -b / determinant,
        -c / determinant,
        a / determinant,
        (c * f - d * e) / determinant,
        (b * e - a * f) / determinant,
    ];
    let [a, b, c, d, e, f] = inverse;
    let [ga, gb, gc, gd, ge, gf] = [
        gradient.a, gradient.b, gradient.c, gradient.d, gradient.e, gradient.f,
    ]
    .map(f64::from);
    let values = [
        a * ga + c * gb,
        b * ga + d * gb,
        a * gc + c * gd,
        b * gc + d * gd,
        a * ge + c * gf + e,
        b * ge + d * gf + f,
    ]
    .map(|v| v as f32);
    if !values.iter().all(|v| v.is_finite()) {
        return None;
    }
    Some(Transform::new(
        values[0], values[1], values[2], values[3], values[4], values[5],
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inverse_composition_matches_point_mapping() {
        for i in 1..128 {
            let i = i as f32;
            let outline = Transform::new(i + 1.0, 0.25, -0.5, i + 2.0, i, -i);
            let gradient = Transform::new(2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
            let relative = paint_transform(outline, gradient).unwrap();
            let restored = Transform::combine(outline, relative);
            for (got, expected) in [
                restored.a, restored.b, restored.c, restored.d, restored.e, restored.f,
            ]
            .into_iter()
            .zip([
                gradient.a, gradient.b, gradient.c, gradient.d, gradient.e, gradient.f,
            ]) {
                assert!((got - expected).abs() < 0.0001, "{got} != {expected}");
            }
        }
    }

    #[test]
    fn unrepresentable_transforms_are_rejected() {
        let gradient = Transform::default();
        assert!(paint_transform(Transform::new_scale(0.0, 1.0), gradient).is_none());
        assert!(paint_transform(Transform::new(1.0, 2.0, 2.0, 4.0, 0.0, 0.0), gradient).is_none());
        assert!(paint_transform(Transform::new_scale(f32::NAN, 1.0), gradient).is_none());
        assert!(paint_transform(Transform::new_scale(f32::INFINITY, 1.0), gradient).is_none());
        assert!(paint_transform(Transform::new_scale(f32::from_bits(1), 1.0), gradient).is_none());
    }
}
