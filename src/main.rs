use geometry::{Sphere, Triangle};
use vec3::Vec3;
use hit::World;
use camera::Camera;
mod vec3;
mod ray;
mod geometry;
mod hit;
mod camera;

fn main() {

    //Init camera 
    let mut cam = Camera::new(400, 16.0/9.0,1.0,2.0);
    cam.init();

    //Init World
    let mut world = World::new();
    world.add_object(Box::new(Sphere::new(Vec3(0.0,0.0,-1.0), 0.5)));
    world.add_object(Box::new(Sphere::new(Vec3(0.0,-100.5,-1.0), 100.0)));
    world.add_object(Box::new(Triangle::new(Vec3(-2.0, -1.0, -2.0), Vec3(2.0,-1.0,-2.0), Vec3(0.0,2.0,-2.0))));

    //Render image and write to ppm
    //TODO: implement support for other image formats
    cam.render(&world);
    cam.image().out_ppm("out.ppm");

}
