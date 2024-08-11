use crate::hit::World;
use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hit::Hittable;
use crate::geometry::*;
use std::fs;

#[derive(Debug,Clone)]
pub struct Camera {

	pub image_width: i64,
    pub aspect_ratio: f64,
    pub focal_length: f64,
    pub viewport_height: f64,

    image_height: i64,
    img_data: Image, 
    viewport_width: f64,
    camera_center: Vec3,
    viewport_hor: Vec3,
    viewport_ver: Vec3,
    pixel_delta_hor: Vec3,
    pixel_delta_ver: Vec3,
    vw_ul_corner: Vec3,
    cntr_frst_px: Vec3,

}

impl Camera {
	pub fn new(width: i64, asp_ratio: f64, focal_len: f64, view_height: f64) -> Self {
		Self{ 
			image_width: width,
			aspect_ratio: asp_ratio,
			focal_length: focal_len,
			viewport_height: view_height,
		
			image_height: 0,
			img_data: Image::new(0,0), 
			viewport_width: 0.0,
			camera_center: Vec3::zero(),
			viewport_hor: Vec3::zero(),
			viewport_ver: Vec3::zero(),
			pixel_delta_hor: Vec3::zero(),
			pixel_delta_ver: Vec3::zero(),
			vw_ul_corner: Vec3::zero(),
			cntr_frst_px: Vec3::zero(),
		}
	}
	pub fn init(&mut self) {
		self.viewport_height = 2.0;
		self.focal_length = 1.0;
		self.image_height = (self.image_width as f64/self.aspect_ratio) as i64;
		self.viewport_width = self.viewport_height * (self.image_width as f64/self.image_height as f64);
		self.camera_center = Vec3(0.0,0.0,0.0);
	
		self.viewport_hor = Vec3(self.viewport_width,0.0,0.0);
		self.viewport_ver = Vec3(0.0,-self.viewport_height,0.0);
		self.pixel_delta_hor = self.viewport_hor/self.image_width as f64;
		self.pixel_delta_ver = self.viewport_ver/self.image_height as f64;
		
		self.vw_ul_corner = self.camera_center- Vec3(0.0,0.0,self.focal_length) - self.viewport_hor/2.0 - self.viewport_ver/2.0;
		self.cntr_frst_px = self.vw_ul_corner + ((self.pixel_delta_hor + self.pixel_delta_ver) *0.5);
		self.img_data.h = self.image_height;
		self.img_data.w = self.image_width;
	}
	pub fn render(&mut self, w: &World) {
		for j in 0..self.image_height {
			for i in 0..self.image_width {
				let px_cntr = self.cntr_frst_px + (self.pixel_delta_hor * i as f64) + (self.pixel_delta_ver * j as f64);
				let ray_dir = px_cntr - self.camera_center;
				let px_ray = Ray::new(self.camera_center,ray_dir);
	
				let px_color = Self::color_at(px_ray, &w);
				self.img_data.data.push(px_color);
			}   
		}
	}
	fn color_at(r: Ray, w: &World) -> Vec3 {
		let possible_hit = w.hit(r, Interval::new(0.0, f64::INFINITY));
		match possible_hit {
			Some(h) => {
				let normal = h.norm;
				return  (normal+1.0) *0.5;
			}
			None => {
				let unit_dir = r.dir.normalize();
				let a = 0.5 * (unit_dir.1 + 1.0);
				return (Vec3(1.0,1.0,1.0)*(1.0-a)) + Vec3(0.3,0.6,1.0) * a;
			}
		}
	}
	pub fn image(self) -> Image {
		self.img_data
	}
}

#[derive(Debug,Clone)]
pub struct Image {
	h: i64,
	w: i64,
	data: Vec<Vec3>
}

impl Image {
	pub fn new(height: i64, width: i64) -> Self {
		Self {
			h: height,
			w: width,
			data: Vec::new()
		}
	}
	pub fn out_ppm(self,file_name: &str) {
		let mut out :Vec<String> = vec![format!("P3\n{} {}\n255",self.w,self.h)];
		self.data.iter().for_each(|p| out.push(p.write_color()));
		fs::write(file_name,out.join("\n")).unwrap();
	}
}