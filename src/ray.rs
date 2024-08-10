use crate::vec3;
use vec3::Vec3;

#[derive(Clone, Copy)]
pub struct Ray{pub origin: Vec3, pub dir: Vec3}

impl Ray {
	pub fn new(o: Vec3, d: Vec3) -> Ray{
		Ray{origin: o, dir: d}
	}
	pub fn at(&self, t: f64) -> Vec3 {
		self.origin + (self.dir * t)
	}
}