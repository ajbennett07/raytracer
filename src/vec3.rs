
pub type Color = Vec3;

#[derive(Clone,Copy,Debug)]
pub struct Vec3(pub f64,pub f64,pub f64);

impl Vec3 {
	pub fn length(&self) -> f64 {
		(self.0.powi(2) + self.1.powi(2) + self.2.powi(2)).sqrt()
	}
	pub fn normalize(&self) -> Vec3 {
		*self/self.length()
	}
	pub fn dot(&self, rhs: Vec3) -> f64 {
		(self.0*rhs.0) + (self.1*rhs.1) + (self.2*rhs.2)
	}
	pub fn cross(&self, rhs: Vec3) -> Vec3 {
		Vec3(self.1*rhs.2 - self.2*rhs.1, self.2*rhs.0 - self.0*rhs.2, self.0*rhs.1 - self.1*rhs.0)
	}
	pub fn zero() -> Self {
		Self(0.0,0.0,0.0)
	}
}

impl std::ops::Add for Vec3 {

	type Output = Self;
	fn add(self, rhs: Self) -> Self::Output {
		Vec3(self.0+rhs.0,self.1+rhs.1,self.2+rhs.2)
	}
}

impl std::ops::Add<f64> for Vec3 {

	type Output = Self;
	fn add(self, rhs: f64) -> Self::Output {
		Vec3(self.0+rhs,self.1+rhs,self.2+rhs)
	}
}

impl std::ops::Sub for Vec3 {

	type Output = Self;
	fn sub(self, rhs: Self) -> Self::Output {
		Vec3(self.0-rhs.0,self.1-rhs.1,self.2-rhs.2)
	}
}

impl std::ops::Mul<f64> for Vec3 {

	type Output = Self;

	fn mul(self, rhs: f64) -> Self::Output {
		Vec3(self.0*rhs,self.1*rhs,self.2*rhs)
	}

}

impl std::ops::Div<f64> for Vec3 {

	type Output = Self;

	fn div(self, rhs: f64) -> Self::Output {
		Vec3(self.0/rhs,self.1/rhs,self.2/rhs)
	}
}

impl std::ops::Div<Self> for Vec3 {
	type Output = Self;

	fn div(self, rhs: Self) -> Self::Output {
		Vec3(self.0/rhs.0,self.1/rhs.1,self.2/rhs.2)
	}
}

impl Color {
	pub fn write_color(&self) -> String {
		let r = self.0 * 255.0;
		let g = self.1 * 255.0;
		let b = self.2 * 255.0;
		format!("{:.0} {1:.0} {2:.0}",r,g,b)
	}
}