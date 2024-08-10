use crate::vec3::*;
use crate::ray::*;
use crate::geometry::Interval;
use std::cmp::{Ord,Eq};

pub trait Hittable {
	fn hit(&self, r: Ray, t_int: Interval) -> Option<Hit>;
}

#[derive(Clone, Debug)]
pub struct Hit {
	pub norm: Vec3,
	pub point: Vec3,
	pub t: f64
}

impl Hit {
	pub fn new() -> Self {
		Self {
			norm: Vec3::zero(),
			point: Vec3::zero(),
			t: 0.0,
		}
	}
	pub fn set_face_normal(&self, r: &Ray) -> Vec3 {
		if r.dir.dot(self.norm) < 0.0 {
			self.norm
		}
		else {
			self.norm * -1.0
		}
	}
}

impl PartialEq for Hit {
	fn eq(&self, other: &Hit) -> bool { self.t == other.t }

}

impl PartialOrd for Hit {
	fn partial_cmp(&self, other: &Hit) -> Option<std::cmp::Ordering> { 
		self.t.partial_cmp(&other.t)
	}
}

impl Eq for Hit {}

impl Ord for Hit {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.t.partial_cmp(&other.t).unwrap()
    }
}

pub struct World {
	objects: Vec<Box<dyn Hittable>>
}

impl World {
	pub fn new() -> Self {
		Self { objects: Vec::new() }
	}
	pub fn add_object(&mut self, h: Box<dyn Hittable> ) {
		self.objects.push(h);
	}
}

impl Hittable for World {
	fn hit(&self, r: Ray, t_int: Interval) -> Option<Hit> {
		let mut ret = Hit::new();
		let mut closest = t_int.b;
		let mut hit = false;
		for i in self.objects.iter() {
			match i.hit(r, Interval::new(t_int.a, closest)) {
				Some(h) => {
					hit = true;
					closest = h.t;
					ret = h;
				}
				None => {}
			}
		}
		if hit {
			return Some(ret);
		}
		None
	}
}