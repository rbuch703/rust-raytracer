use crate::{
    math::{Axis, Bounded3D, BoundingBox, GeometryHitRecord, Triangle, Vec3},
    scene_objects::Geometry3D,
};

pub struct KDTreeNodeSubdivision {
    axis: Axis,
    position: f64,
    negative_child: KDTreeNode,
    positive_child: KDTreeNode,
}

impl KDTreeNodeSubdivision {
    fn hit_recursive(
        &self,
        ray_src: &Vec3,
        ray_dir: &Vec3,
        best_hit: &mut Option<GeometryHitRecord>,
    ) {
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
            return;
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
                return;
            };

            dist
        };

        let other_halfspace_may_have_better_hit = match best_hit {
            // only possible if other half space is closer than the best hit yet
            Some(best_hit) => distance_to_split_plane < best_hit.distance,
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
        if let Some(best_hit) = best_hit {
            assert!(best_hit.distance >= distance_to_split_plane);

            let mut best_hit_child = None;
            other_child.hit_recursive(&ray_src, ray_dir, &mut best_hit_child);
            if let Some(child_hit) = best_hit_child
                && (distance_to_split_plane + child_hit.distance < best_hit.distance)
            {
                *best_hit = GeometryHitRecord {
                    distance: distance_to_split_plane + child_hit.distance,
                    normal: child_hit.normal,
                };
            }
        } else {
            other_child.hit_recursive(&ray_src, ray_dir, best_hit);
            if let Some(best_hit) = best_hit {
                // Account for the distance from the original ray source to the split plane, which
                // is not included in the child hit record.
                best_hit.distance += distance_to_split_plane;
            }
        }
    }
}

pub struct KDTreeNode {
    children: Vec<Triangle<Vec3>>,
    subdivision: Option<Box<KDTreeNodeSubdivision>>,
}

const MAX_ITEMS_PER_NODE: usize = 64;

impl KDTreeNode {
    pub fn new(items: Vec<Triangle<Vec3>>) -> Self {
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

    pub fn hit_recursive(
        &self,
        ray_src: &Vec3,
        ray_dir: &Vec3,
        best_hit: &mut Option<GeometryHitRecord>,
    ) {
        for triangle in &self.children {
            if let Some(this_hit) = triangle.hit(ray_src, ray_dir) {
                *best_hit = Some(match best_hit {
                    Some(prev) => prev.min(this_hit),
                    None => this_hit,
                });
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
