use crate::vec3::Vec3;


struct Camera {

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
			img_data: Image::new(), 
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
		self.viewport_width = self.viewport_height * (self.image_width as f64/self.image_height as f64);
		self.camera_center = Vec3(0.0,0.0,0.0);
	
		self.viewport_hor = Vec3(self.viewport_width,0.0,0.0);
		self.viewport_ver = Vec3(0.0,-self.viewport_height,0.0);
		self.pixel_delta_hor = self.viewport_hor/self.image_width as f64;
		self.pixel_delta_ver = self.viewport_ver/self.image_height as f64;
		
		self.vw_ul_corner = self.camera_center- Vec3(0.0,0.0,self.focal_length) - self.viewport_hor/2.0 - self.viewport_ver/2.0;
		self.cntr_frst_px = self.vw_ul_corner + ((self.pixel_delta_hor + self.pixel_delta_ver) *0.5);
	
	}
	pub fn render(&mut self) {

	}
}

struct Image {
	data: Vec<Vec3>
}

impl Image {
	pub fn new() -> Self {
		Self {
			data: Vec::new()
		}
	}
}