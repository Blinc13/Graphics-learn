use glam::{Vec2, Vec3};
use image::DynamicImage::ImageRgb8;
use graphics_learn::algorithms::trace_sphere;
use graphics_learn::Const::Colors::{BLUE, RED};
use graphics_learn::objects::sphere::Sphere;
use graphics_learn::scene::Scene;

fn main() {
    let mut image = image::RgbImage::new(1000, 1000);
    let mut scene = Scene::new();

    scene.add_object(Box::new(
        Sphere::new(
            Vec3::new(0.0, 0.0, 50.0),
            Vec3::new(1.0, 1.0, 1.0),
            10.0,
            BLUE
        )
    ));

    for x in 0..1000 {
        for y in 0..1000 {
            let cords = Vec2::new((x as f32 - 500.0), (y as f32 - 500.0));

            image.put_pixel(x, y,scene.render_pixel(cords));
        }
    }

    image.save("test.png").unwrap();
}
