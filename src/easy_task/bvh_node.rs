use crate::easy_task::aabb::Aabb;
use crate::easy_task::hittable::{HitRecord, Hittable};
use crate::easy_task::hittable_list::HittableList;
use crate::easy_task::interval::Interval;
use crate::easy_task::ray::Ray;
use crate::tools::rtweekend::random_int;
use std::rc::Rc;

#[derive(Clone)]
pub struct BvhNode {
    left: Rc<dyn Hittable>,
    right: Rc<dyn Hittable>,
    bbox: Aabb,
}

impl BvhNode {
    #[allow(dead_code)]
    pub fn new_list(list: &mut HittableList) -> Self {
        let len = list.objects.len();
        Self::new(&mut list.objects, 0, len)
    }

    pub fn new(src_objects: &mut Vec<Rc<dyn Hittable>>, start: usize, end: usize) -> Self {
        let axis = random_int(0, 2);

        let comparator = match axis {
            0 => Self::box_x_compare,
            1 => Self::box_y_compare,
            _ => Self::box_z_compare,
        };

        let objects = src_objects;

        let object_span = end - start;

        if object_span == 1 {
            Self {
                left: objects[start].clone(),
                right: objects[start].clone(),
                bbox: (*objects[start].bounding_box()).clone(),
            }
        } else if object_span == 2 {
            if comparator(&objects[start], &objects[start + 1]) == std::cmp::Ordering::Less {
                Self {
                    left: objects[start].clone(),
                    right: objects[start + 1].clone(),
                    bbox: Aabb::new_aabb(
                        objects[start].bounding_box(),
                        objects[start + 1].bounding_box(),
                    ),
                }
            } else {
                Self {
                    left: objects[start + 1].clone(),
                    right: objects[start].clone(),
                    bbox: Aabb::new_aabb(
                        objects[start + 1].bounding_box(),
                        objects[start].bounding_box(),
                    ),
                }
            }
        } else {
            objects[start..end].sort_by(comparator);

            let mid = start + object_span / 2;
            let left = Rc::new(Self::new(objects, start, mid));
            let right = Rc::new(Self::new(objects, mid, end));
            let bbox = Aabb::new_aabb(left.bounding_box(), right.bounding_box());
            Self { left, right, bbox }
        }
    }
    fn box_compare(
        a: &Rc<dyn Hittable>,
        b: &Rc<dyn Hittable>,
        axis_index: usize,
    ) -> std::cmp::Ordering {
        a.bounding_box()
            .axis(axis_index)
            .min
            .partial_cmp(&b.bounding_box().axis(axis_index).min)
            .unwrap()
    }

    fn box_x_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> std::cmp::Ordering {
        Self::box_compare(a, b, 0)
    }

    fn box_y_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> std::cmp::Ordering {
        Self::box_compare(a, b, 1)
    }

    fn box_z_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> std::cmp::Ordering {
        Self::box_compare(a, b, 2)
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        let mut ray_t = ray_t.clone();
        if !self.bbox.hit(r, &mut ray_t) {
            return false;
        }

        let hit_left = self.left.hit(r, &ray_t, rec);
        let ray_t = Interval::new(ray_t.min, if hit_left { rec.t } else { ray_t.max });
        let hit_right = self.right.hit(r, &ray_t, rec);

        hit_left || hit_right
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
