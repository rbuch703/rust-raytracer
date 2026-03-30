use std::collections::HashMap;

use crate::math::{Triangle, Vec3};

pub fn compute_vertex_normals(triangles: &[(Vec3, Vec3, Vec3)]) -> Vec<Triangle> {
    // Represent a vertex position as a key suitable for a HashMap by reinterpreting
    // the three f64 components as their raw bits. Two vertices are considered the
    // same iff they share exactly the same bit pattern in all three components.
    fn key(v: Vec3) -> (u64, u64, u64) {
        (v.x.to_bits(), v.y.to_bits(), v.z.to_bits())
    }

    // Accumulate angle-weighted face normals into each unique vertex.
    let mut accum: HashMap<(u64, u64, u64), Vec3> = HashMap::new();

    for &(v1, v2, v3) in triangles {
        let face_normal = (v2 - v1).cross(v3 - v1).normalized();

        // Skip degenerate triangles that produce a NaN normal.
        if face_normal.x.is_nan() {
            continue;
        }

        // The weight for each vertex is the interior angle of the triangle at
        // that vertex, computed via the dot product of the two adjacent edge
        // directions.
        let angle_at = |corner: Vec3, a: Vec3, b: Vec3| {
            (a - corner)
                .normalized()
                .dot((b - corner).normalized())
                .clamp(-1.0, 1.0)
                .acos()
        };

        let a1 = angle_at(v1, v2, v3);
        let a2 = angle_at(v2, v1, v3);
        let a3 = angle_at(v3, v1, v2);

        for (v, angle) in [(v1, a1), (v2, a2), (v3, a3)] {
            let entry = accum.entry(key(v)).or_insert(Vec3::new(0.0, 0.0, 0.0));
            *entry = *entry + face_normal * angle;
        }
    }

    // Build Triangle objects, substituting the smoothed vertex normals.
    triangles
        .iter()
        .map(|&(v1, v2, v3)| {
            let n1 = accum[&key(v1)].normalized();
            let n2 = accum[&key(v2)].normalized();
            let n3 = accum[&key(v3)].normalized();
            Triangle { v1, v2, v3, n1, n2, n3 }
        })
        .collect()
}

pub fn parse_obj(filename: &str) -> std::io::Result<Vec<(Vec3, Vec3, Vec3)>> {
    let mut vertices = Vec::new();
    let mut faces = Vec::<(usize, usize, usize)>::new();

    for l in std::fs::read_to_string(filename)?.lines() {
        let line = l.trim();
        if line.is_empty() {
            // skip emtpy lines
            continue;
        }

        if line.starts_with('#') {
            // skip comment lines
            continue;
        }

        let mut parts = line.split(' ');
        let mut next = || {
            parts
                .next()
                .ok_or(std::io::Error::from(std::io::ErrorKind::Other))
        };
        let indicator = next()?;
        match indicator {
            "f" => {
                faces.push((
                    next()?.parse().expect("int"),
                    next()?.parse().expect("int"),
                    next()?.parse().expect("int"),
                ));
            }
            "v" => {
                vertices.push(Vec3::new(
                    next()?.parse().expect("f64"),
                    next()?.parse().expect("f64"),
                    next()?.parse().expect("f64"),
                ));
            }
            _ => panic!("Unexpected indicator {}", indicator),
        }
        if let Ok(el) = next() {
            panic!("Found trailing element {el}");
        }
    }

    // Obj face indices are one-based, so shift to zero-based array indices
    Ok(faces
        .into_iter()
        .map(|(i1, i2, i3)| (vertices[i1 - 1], vertices[i2 - 1], vertices[i3 - 1]))
        .collect())
}
