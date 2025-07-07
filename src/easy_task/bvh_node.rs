use crate::easy_task::aabb;
use crate::easy_task::aabb::Aabb;
use crate::easy_task::hittable::{HitRecord, Hittable};
use crate::easy_task::hittable_list::HittableList;
use crate::easy_task::interval::Interval;
use crate::easy_task::ray::Ray;
use std::sync::Arc;

#[derive(Clone)]
pub struct BvhNode {
    left: Arc<dyn Hittable + Sync + Send>,
    right: Arc<dyn Hittable + Sync + Send>,
    bbox: Aabb,
}

impl BvhNode {
    #[allow(dead_code)]
    pub fn new_list(list: &mut HittableList) -> Self {
        BvhNode::new(&mut list.objects.clone(), 0, list.objects.len() as i32)
    }

    pub fn new(objects: &mut Vec<Arc<dyn Hittable + Send + Sync>>, start: i32, end: i32) -> Self {
        let mut bbox = aabb::EMPTY;
        for object_index in start..end {
            bbox = Aabb::new_aabb(&bbox, objects[object_index as usize].bounding_box());
        }

        let axis = bbox.longest_axis();

        let comparator = match axis {
            0 => Self::box_x_compare,
            1 => Self::box_y_compare,
            _ => Self::box_z_compare,
        };

        let object_span = end - start;

        if object_span == 1 {
            Self {
                left: objects[start as usize].clone(),
                right: objects[start as usize].clone(),
                bbox: *objects[start as usize].bounding_box(),
            }
        } else if object_span == 2 {
            Self {
                left: objects[start as usize].clone(),
                right: objects[(start + 1) as usize].clone(),
                bbox: Aabb::new_aabb(
                    objects[start as usize].bounding_box(),
                    objects[(start + 1) as usize].bounding_box(),
                ),
            }
        } else {
            objects[start as usize..end as usize].sort_by(comparator);

            let mid = start + object_span / 2;
            let left = Arc::new(Self::new(objects, start, mid));
            let right = Arc::new(Self::new(objects, mid, end));
            let bbox = Aabb::new_aabb(left.bounding_box(), right.bounding_box());
            Self { left, right, bbox }
        }
    }

    fn box_compare(
        a: &Arc<dyn Hittable + Sync + Send>,
        b: &Arc<dyn Hittable + Sync + Send>,
        axis_index: usize,
    ) -> std::cmp::Ordering {
        a.bounding_box()
            .axis_interval(axis_index)
            .min
            .partial_cmp(&b.bounding_box().axis_interval(axis_index).min)
            .unwrap()
    }

    fn box_x_compare(
        a: &Arc<dyn Hittable + Sync + Send>,
        b: &Arc<dyn Hittable + Sync + Send>,
    ) -> std::cmp::Ordering {
        Self::box_compare(a, b, 0)
    }

    fn box_y_compare(
        a: &Arc<dyn Hittable + Sync + Send>,
        b: &Arc<dyn Hittable + Sync + Send>,
    ) -> std::cmp::Ordering {
        Self::box_compare(a, b, 1)
    }

    fn box_z_compare(
        a: &Arc<dyn Hittable + Sync + Send>,
        b: &Arc<dyn Hittable + Sync + Send>,
    ) -> std::cmp::Ordering {
        Self::box_compare(a, b, 2)
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        let mut ray_t = *ray_t;
        if !self.bbox.hit(r, &mut ray_t) {
            return false;
        }

        let hit_left = self.left.hit(r, &ray_t, rec);
        let hit_right = self.right.hit(
            r,
            &Interval::new(ray_t.min, if hit_left { rec.t } else { ray_t.max }),
            rec,
        );

        hit_left || hit_right
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
