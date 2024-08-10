use std::fs;
use geometry::{Sphere, Triangle,Interval};
use ray::Ray;
use vec3::Vec3;
use hit::{Hittable,World};
mod vec3;
mod ray;
mod geometry;
mod hit;

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

fn main() {

    let image_width = 400;
    let aspect_ratio = 16.0/9.0;
    let image_height = (image_width as f64/aspect_ratio) as i32;

    let mut img_data :Vec<String> = vec![format!("P3\n{image_width} {image_height}\n255\n")];

    let viewport_height = 2.0;
    let focal_length = 1.0;
    let viewport_width = viewport_height * (image_width as f64/image_height as f64);
    let camera_center = Vec3(0.0,0.0,0.0);

    let viewport_hor = Vec3(viewport_width,0.0,0.0);
    let viewport_ver = Vec3(0.0,-viewport_height,0.0);
    let pixel_delta_hor = viewport_hor/image_width as f64;
    let pixel_delta_ver = viewport_ver/image_height as f64;
    
    let vw_ul_corner = camera_center- Vec3(0.0,0.0,focal_length) - viewport_hor/2.0 - viewport_ver/2.0;
    let cntr_frst_px = vw_ul_corner + ((pixel_delta_hor + pixel_delta_ver) *0.5);

    //Init World
    let mut world = World::new();
    world.add_object(Box::new(Sphere::new(Vec3(0.0,0.0,-1.0), 0.5)));
    world.add_object(Box::new(Sphere::new(Vec3(0.0,-100.5,-1.0), 100.0)));
    world.add_object(Box::new(Triangle::new(Vec3(-2.0, -1.0, -2.0), Vec3(2.0,-1.0,-2.0), Vec3(0.0,2.0,-2.0))));

    //Write pixel values
    for j in 0..image_height {
        for i in 0..image_width {
            let px_cntr = cntr_frst_px + (pixel_delta_hor * i as f64) + (pixel_delta_ver * j as f64);
            let ray_dir = px_cntr - camera_center;
            let px_ray = Ray::new(camera_center,ray_dir);

            let px_color = color_at(px_ray, &world);
            img_data.push(px_color.write_color());
        }   
    }
    //Write to file
    fs::write("out.ppm", img_data.join("\n")).unwrap();
}
