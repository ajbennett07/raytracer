use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::hit::*;

pub struct Interval {
	pub a: f64,
	pub b: f64,
}

#[allow(dead_code)]
impl Interval {
	pub fn empty() -> Self {
		Self {a: f64::INFINITY, b: f64::NEG_INFINITY}
	}
	pub fn line() -> Self {
		Self {a: f64::NEG_INFINITY, b: f64::INFINITY}
	}
	pub fn new(min: f64, max: f64) -> Self {
		Self{a:min, b: max}
	}
	pub fn contains(&self, x: f64) -> bool {
		if (x <= self.b) && (x >= self.a) {
			return true;
		}
		false
	}
	pub fn surrounds(&self, x: f64) -> bool {
		if (x < self.b) && (x > self.a) {
			return true;
		}
		false
	}
	pub fn length(&self) -> f64 {
		self.b - self.a
	}
}

pub struct Sphere {
	center: Vec3,
	radius: f64
}

impl Sphere {
	pub fn new(cent: Vec3, r: f64) -> Self {
		Self {
			center: cent,
			radius: r
		}
	}
}

impl Hittable for Sphere {
	fn hit(&self, r: Ray, t_int: Interval) -> Option<Hit> {
		let oc = self.center - r.origin;
		let a = r.dir.length().powi(2);
		let h = r.dir.dot(oc);
		let c = oc.length().powi(2) - self.radius.powi(2);
		let disc = h.powi(2) - a*c;
		if disc < 0.0 {
			return None;
		}
		let mut ret = Hit::new();
		if !t_int.surrounds(h - disc.sqrt() / a) {
			if !t_int.surrounds((h + disc.sqrt()) / a) {
				return None;
			}
			ret.t = (h + disc.sqrt()) / a;
		}
		else {
			ret.t = (h - disc.sqrt()) / a;
		}
		ret.point = r.at(ret.t);
		ret.norm = (ret.point - self.center) / self.radius;
		ret.norm = ret.set_face_normal(&r);
		Some(ret)
	}
}


pub struct Triangle {
	v1: Vec3,
	v2: Vec3,
	v3: Vec3
}

impl Triangle {
	pub fn new(one: Vec3, two: Vec3, three: Vec3) -> Self {
		Self { v1: one, v2: two, v3: three}
	}
}

impl Hittable for Triangle {
	// Moeller-Trumbore Intersection Algorithm
	fn hit(&self, r: Ray, t_int: Interval) -> Option<Hit> {
		let e1 = self.v2 - self.v1;
		let e2 = self.v3 - self.v1;

		let ray_cross_bs_2 = r.dir.cross(e2);
		let det = e1.dot(ray_cross_bs_2);

		if det > -f64::EPSILON && det < f64::EPSILON {
			return None;
		}

		let inv_det = 1.0 / det;
		let s = r.origin - self.v1;
		let u = inv_det * s.dot(ray_cross_bs_2);
		if u < 0.0 || u > 1.0 {
			return None;
		}

		let s_cross_bs_1 = s.cross(e1);
		let v = inv_det * r.dir.dot(s_cross_bs_1);
		if v < 0.0 || u + v > 1.0 {
			return None;
		}

		let t = inv_det * e2.dot(s_cross_bs_1);

		if t_int.contains(t) {
			return Some(
				Hit {
					norm: e1.cross(e2).normalize(),
					point: r.at(t),
					t: t
				}
			)
		}
		None
	}
}