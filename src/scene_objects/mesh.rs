use crate::{
    math::{Axis, Bounded3D, BoundingBox, Range, Vec3}, scene_objects::{HitRecord, Material, Object3D}
};

pub struct KDTreeNodeSubdivision {
    axis: Axis,
    position: f64,
    negative_child: KDTreeNode,
    positive_child: KDTreeNode,
}

impl KDTreeNodeSubdivision {
    fn hit_recursive(&self, ray_src: &Vec3, ray_dir: &Vec3, best_hit: &mut Option<(f64, Vec3)>) {
        if ray_src.get(self.axis) == self.position {
            /* Edge case: if the ray starts *on* the split plane then both half-spaces must be
            checked, because both may contain geometry that lies exactly on the split plane. */

            self.negative_child
                .hit_recursive(ray_src, ray_dir, best_hit);
            self.positive_child
                .hit_recursive(ray_src, ray_dir, best_hit);
            // no other checks are necessary, early-terminate
            return;
        }

        let (start_child, other_child) = if ray_src.get(self.axis) < self.position {
            (&self.negative_child, &self.positive_child)
        } else {
            (&self.positive_child, &self.negative_child)
        };

        // first test the halfspace that the ray originates in
        start_child.hit_recursive(ray_src, ray_dir, best_hit);

        let ray_component = ray_dir.get(self.axis);
        let distance_to_split_plane = if ray_component == 0.0 {
            /* If ray is parallel to the split plane and does not lie *on* the plane (checked in
            the first condition of this function) => can never cross into the other half-space. */
            return
        } else {
            /* `self.position - ray_src.get(self.axis)` is the `self.axis` component of the distance
            from `ray_src` to the split plane. Dividing this by `ray_component` gives the number of times
            that `ray_component` fits into that distance. Since `ray_dir` is normalized, this number
            corresponds exactly to the length of the ray section from `ray_src` to the split plane.
            */
            let dist = (self.position - ray_src.get(self.axis)) / ray_component;
            if dist < 0.0 {
                /* If `dist` is < 0.0 then the ray points away from the split plane, so it will
                never reach that plane nor the other half space. */
                return
            };
            
            dist
        };

        let other_halfspace_may_have_better_hit = match best_hit {
            // only possible if other half space is closer than the best hit yet
            Some((best_hit_distance, _)) => distance_to_split_plane < *best_hit_distance,
            None => true, // no hit yet -> any hit in other halfspace would be better
        };

        if !other_halfspace_may_have_better_hit {
            //no need to test the other half-space
            return;
        }

        /* Move ray source forward onto the split plane, since the part of the ray behind this
        point has already been tested. This updated ray position is necessary for recursive calls
        to accurately determine through which of its half-spaces the ray passes. */
        let ray_src = ray_src + ray_dir * distance_to_split_plane;
        if let Some((best_distance, normal)) = best_hit {
            assert!(*best_distance >= distance_to_split_plane);

            let mut best_hit_child = None;
            other_child.hit_recursive(&ray_src, ray_dir, &mut best_hit_child);
            if let Some((child_distance, child_normal)) = best_hit_child && (distance_to_split_plane + child_distance < *best_distance) {
                *best_distance = distance_to_split_plane + child_distance;
                *normal = child_normal;
            }
        } else {
            other_child.hit_recursive(&ray_src, ray_dir, best_hit);
            best_hit.as_mut().map(|(best_distance, _)| *best_distance += distance_to_split_plane);
        }
    }
}

struct KDTreeNode {
    children: Vec<Triangle<Vec3>>,
    subdivision: Option<Box<KDTreeNodeSubdivision>>,
}

const MAX_ITEMS_PER_NODE: usize = 64;

impl KDTreeNode {
    fn new(items: Vec<Triangle<Vec3>>) -> Self {
        if items.len() < MAX_ITEMS_PER_NODE {
            return KDTreeNode {
                children: items,
                subdivision: None,
            };
        }

        let bounds = BoundingBox::from_items(&items).expect("non-empty list");

        // Split along the axis with the largest range
        let split_axis = if bounds.x.size() >= bounds.y.size() && bounds.x.size() >= bounds.z.size()
        {
            Axis::X
        } else if bounds.y.size() >= bounds.z.size() {
            Axis::Y
        } else {
            Axis::Z
        };

        let mid = (bounds.get(split_axis).min + bounds.get(split_axis).max) / 2.0;
        let mut items_here = Vec::new();
        let mut items_negative = Vec::new();
        let mut items_positive = Vec::new();

        for item in items {
            let bounds = item.bounds();

            if bounds.get(split_axis).max < mid {
                // entirely in negative half-space
                items_negative.push(item);
            } else if bounds.get(split_axis).min >= mid {
                // entirely in positive half-space
                items_positive.push(item);
            } else {
                // overlaps with dividing line
                items_here.push(item)
            }
        }

        KDTreeNode {
            children: items_here,
            subdivision: Some(Box::new(KDTreeNodeSubdivision {
                axis: split_axis,
                position: mid,
                negative_child: KDTreeNode::new(items_negative),
                positive_child: KDTreeNode::new(items_positive),
            })),
        }
    }

    fn _bounding_box(&self) -> Option<BoundingBox> {
        BoundingBox::from_items(&self.children)
    }

    fn hit_recursive(&self, ray_src: &Vec3, ray_dir: &Vec3, best_hit: &mut Option<(f64, Vec3)>) {
        for triangle in &self.children {
            if let Some(hit_pos) = triangle.intersects(*ray_src, *ray_dir) {
                let distance = (ray_src - &hit_pos).len();
                let this_is_best = match best_hit {
                    Some((best_distance, _)) => *best_distance > distance,
                    None => true,
                };
                if this_is_best {
                    let e1 = triangle.v3 - triangle.v1;
                    let e2 = triangle.v2 - triangle.v1;
                    *best_hit = Some((distance, -e1.cross(e2).normalized()));
                }
            }
        }

        if let Some(subdivision) = &self.subdivision {
            subdivision.hit_recursive(ray_src, ray_dir, best_hit);
        }
    }

    /*
    fn bounding_box_recursive(&self) -> BoundingBox {
        let mut res = BoundingBox::from_items(&self.children);
        if let Some(sub) = &self.subdivision {
            res |= sub.child_nodes[0].bounding_box_recursive();
            res |= sub.child_nodes[1].bounding_box_recursive();
        }
        res
    }*/

    fn _print_stats_recursive(&self, depth: usize) {
        let spaces: String = (0..depth * 4).map(|_| ' ').collect();

        println!("{}{}", spaces, self.children.len());
        if let Some(sub) = &self.subdivision {
            sub.negative_child._print_stats_recursive(depth + 1);
            sub.positive_child._print_stats_recursive(depth + 1);
        }
    }

    /*
     fn draw_recursive(&self, context: &cairo::Context, depth: usize) {
         for item in &self.children {
            context.set_line_width(0.1);
            context.set_source_rgb(1.0, 0.0, 0.0);
            item.draw(context);
         }

         if let Some(sub) = &self.subdivision {
             sub.child_nodes[0].draw_recursive(context, depth + 1);
             sub.child_nodes[1].draw_recursive(context, depth + 1);

            let bounds = self.bounding_box_recursive();
            if let BoundingBox::Valid(bounds) = bounds {
                match sub.axis {
                    Axis::X => {
                        context.move_to(sub.position, bounds.y.min);
                        context.line_to(sub.position, bounds.y.max);
                    }
                    Axis::Y => {
                        context.move_to(bounds.x.min, sub.position);
                        context.line_to(bounds.x.max, sub.position);
                    }
                }
                context.set_line_width(1.0 - 0.05 * (depth as f64));
                context.set_source_rgb(0.0, 1.0 - 0.05 * (depth as f64), 0.05 * (depth as f64));
                context.stroke().expect("valid stroke");
            }
            //draw_bounds(context, node.bounding_box_recursive(), 0.0, 1.0 - (depth as f64)/10.0, 0.5);
        }
    }
    */
}

impl Bounded3D for Vec3 {
    fn bounds(&self) -> BoundingBox {
        BoundingBox {
            x: Range::new(self.x),
            y: Range::new(self.y),
            z: Range::new(self.z),
        }
    }
}

pub struct Triangle<T> {
    pub v1: T,
    pub v2: T,
    pub v3: T,
}

impl<T> Triangle<T> {
    pub fn new(v1: T, v2: T, v3: T) -> Self {
        Triangle { v1, v2, v3 }
    }
}

impl Triangle<Vec3> {
    // Taken from https://en.wikipedia.org/wiki/M%C3%B6ller%E2%80%93Trumbore_intersection_algorithm
    fn intersects(&self, ray_origin: Vec3, ray_direction: Vec3) -> Option<Vec3> {
        let e1 = self.v2 - self.v1;
        let e2 = self.v3 - self.v1;

        let ray_cross_e2 = ray_direction.cross(e2);
        let det = e1.dot(ray_cross_e2);

        if det > -f64::EPSILON && det < f64::EPSILON {
            return None; // This ray is parallel to this triangle.
        }

        let inv_det = 1.0 / det;
        let s = ray_origin - self.v1;
        let u = inv_det * s.dot(ray_cross_e2);
        if !(0.0..=1.0).contains(&u) {
            return None;
        }

        let s_cross_e1 = s.cross(e1);
        let v = inv_det * ray_direction.dot(s_cross_e1);
        if v < 0.0 || u + v > 1.0 {
            return None;
        }
        // At this stage we can compute t to find out where the intersection point is on the line.
        let t = inv_det * e2.dot(s_cross_e1);

        if t > f64::EPSILON {
            // ray intersection
            Some(ray_origin + ray_direction * t)
        } else {
            // This means that there is a line intersection but not a ray intersection.
            None
        }
    }
}

impl Bounded3D for Triangle<Vec3> {
    fn bounds(&self) -> BoundingBox {
        self.v1.bounds() | self.v2.bounds() | self.v3.bounds()
    }
}

pub fn parse_obj(filename: &str) -> std::io::Result<Vec<Triangle<Vec3>>> {
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
        .map(|(i1, i2, i3)| Triangle::new(vertices[i1 - 1], vertices[i2 - 1], vertices[i3 - 1]))
        .collect())
}

pub struct TriangleMesh {
    geometry: KDTreeNode,
    material: Material,
    bounds: BoundingBox,
}

fn map_vertices<T, U, F>(src: &[Triangle<T>], trans: F) -> Vec<Triangle<U>>
where
    F: Fn(&T) -> U,
{
    src.iter()
        .map(|Triangle { v1, v2, v3 }| Triangle::new(trans(v1), trans(v2), trans(v3)))
        .collect()
}

impl TriangleMesh {
    pub fn from_obj_file(file_name: &str, material: Material) -> std::io::Result<TriangleMesh> {
        let triangles = parse_obj(file_name)?;

        let triangles = map_vertices(&triangles, |v| {
            Vec3::new(
                v.x * 10000.0,
                v.y * -10000.0 + 500.0,
                -v.z * 10000.0 + 1500.0,
            )
        });

        // The mesh is only valid if it has at least one triangle (Otherwise, no bounds can be established)
        let mut bounds = triangles
            .first()
            .ok_or(std::io::Error::from(std::io::ErrorKind::InvalidInput))?
            .v1
            .bounds();
        for t in &triangles {
            bounds |= t.v1;
            bounds |= t.v2;
            bounds |= t.v3;
        }
        Ok(TriangleMesh {
            geometry: KDTreeNode::new(triangles),
            material,
            bounds,
        })
    }
}

impl Object3D for TriangleMesh {
    fn hit(&self, ray_src: &Vec3, ray_dir: &Vec3) -> Option<HitRecord<'_>> {
        let mut best_hit = None;
        self.geometry.hit_recursive(ray_src, ray_dir, &mut best_hit);

        best_hit.map(|(distance, normal)| HitRecord {
            distance,
            object: self,
            normal,
        })
    }

    fn get_material(&self) -> &Material {
        &self.material
    }
}

impl Bounded3D for TriangleMesh {
    fn bounds(&self) -> BoundingBox {
        self.bounds
    }
}
