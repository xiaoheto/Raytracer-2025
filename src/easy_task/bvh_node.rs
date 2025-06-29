// use std::arch::x86_64::_mm_lfence;
// use crate::easy_task::aabb::Aabb;
// use crate::easy_task::hittable::{HitRecord, Hittable};
// use crate::easy_task::hittable_list::HittableList;
// use crate::easy_task::interval::Interval;
// use crate::easy_task::ray::Ray;
// use crate::tools::rtweekend::random_int;
// use std::rc::Rc;
// use std::sync::atomic::Ordering::SeqCst;
// use crate::easy_task::aabb;
//
// #[derive(Clone)]
// pub struct BvhNode {
//     left: Rc<dyn Hittable>,
//     right: Rc<dyn Hittable>,
//     bbox: Aabb,
// }
//
// impl BvhNode {
//     pub fn new_list(list:&mut HittableList) -> Self {
//         let len = list.objects.len();
//         Self::new(&mut list.objects,0,len)
//     }
//
//     pub fn new(src_objects: &mut Vec<Rc<dyn Hittable>>, start: usize, end: usize) -> Self { // 接受 &mut Vec
//         let mut bbox = aabb::EMPTY;
//         for object_index in start..end{
//             bbox = Aabb::new_aabb(bbox,src_objects[object_index].bounding_box());
//         }
//         let axis = bbox.longest_axis();
//         let comparator = match axis {
//             0 => Self::box_x_compare,
//             1 => Self::box_y_compare,
//             _ => Self::box_z_compare,
//         };
//         let object_span = end - start;
//
//         if object_span == 1 {
//             // 只有一个物体, 不再克隆
//             if let Some(obj) = src_objects.get(start).cloned() {
//                 Self { // 创建节点并且返回
//                     left: obj.clone(),
//                     right: obj.clone(),
//                     bbox
//                 }
//             } else {
//                 // handle the error: potentially, return a default null node or panic
//                 panic!("Invalid start index in BvhNode::new");
//             }
//
//
//         } else if object_span == 2 {
//             // 两个物体
//             if comparator(src_objects[start].clone(), src_objects[start + 1].clone()) == std::cmp::Ordering::Less { // 不再克隆, 获取.clone
//                 Self {
//                     left: src_objects[start].clone(),
//                     right: src_objects[start + 1].clone(),
//                     bbox
//                 }
//             } else {
//                 Self {
//                     left: src_objects[start + 1].clone(),
//                     right: src_objects[start].clone(),
//                     bbox
//                 }
//             }
//         } else {
//             // 多个物体，需要排序和递归
//             src_objects[start..end].sort_by(|a, b| {
//                 comparator(a.clone(), b.clone()) //  使用.clone
//             });
//
//             let mid = start + object_span / 2;
//
//             let left = Rc::new(Self::new(src_objects, start, mid));
//             let right = Rc::new(Self::new(src_objects, mid, end));
//             Self {
//                 left,
//                 right,
//                 bbox,
//             }
//         }
//     }
//
//     fn box_compare(a: Rc<dyn Hittable>, b: Rc<dyn Hittable>, axis_index: i32) -> std::cmp::Ordering {
//         let box_a = a.bounding_box();
//         let box_b = b.bounding_box();
//         box_a
//             .axis_interval(axis_index)
//             .min
//             .partial_cmp(&box_b.axis_interval(axis_index).min)
//             .unwrap_or(std::cmp::Ordering::Equal)
//     }
//
//     fn box_x_compare(a: Rc<dyn Hittable>, b: Rc<dyn Hittable>) -> std::cmp::Ordering {
//         Self::box_compare(a, b, 0)
//     }
//     fn box_y_compare(a: Rc<dyn Hittable>, b: Rc<dyn Hittable>) -> std::cmp::Ordering {
//         Self::box_compare(a, b, 1)
//     }
//     fn box_z_compare(a: Rc<dyn Hittable>, b: Rc<dyn Hittable>) -> std::cmp::Ordering {
//         Self::box_compare(a, b, 2)
//     }
// }
//
// impl Hittable for BvhNode{
//     fn hit(&self, r: Ray, ray_t: &mut Interval, rec: &mut HitRecord) -> bool {
//         if !self.bbox.hit(r, ray_t) {
//             return false;
//         }
//
//         let hit_left = self.left.hit(r, ray_t, rec);
//         // 这里修正了区间
//         let hit_right = self.right.hit(
//             r,
//             &mut Interval::new(ray_t.min, if hit_left { rec.t } else { ray_t.max }),
//             rec,
//         );
//         hit_left || hit_right
//     }
//
//     fn bounding_box(&self) -> Aabb {
//         self.bbox
//     }
// }
