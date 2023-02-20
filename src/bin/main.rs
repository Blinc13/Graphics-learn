use glam::{Vec2, Vec3};
use image::DynamicImage::ImageRgb8;
use image::Rgb;
use graphics_learn::Const::Colors::{BLUE, RED};
use graphics_learn::objects::ambient_light::AmbientLight;
use graphics_learn::objects::global_light::GlobalLight;
use graphics_learn::objects::point_light::PointLight;
use graphics_learn::objects::sphere::Sphere;
use graphics_learn::scene::Scene;

fn main() {
    let mut image = image::RgbImage::new(1000, 1000);
    let mut scene = Scene::new();

    scene.add_object(Box::new(
        Sphere::new(
            Vec3::new(1.0, -2.0, 15.0),
            Vec3::new(1.0, 1.0, 1.0),
            2.0,
            Rgb::from([0u8, 0u8, 100u8])
        )
    ));
    // scene.add_object(Box::new(
    //     Sphere::new(
    //         Vec3::new(0.0, 5001.0, 0.0),
    //         Vec3::new(1.0, 1.0, 1.0),
    //         5000.0,
    //         Rgb::from([50u8, 50u8, 0u8])
    //     )
    // ));
    scene.add_object(Box::new(
        Sphere::new(
            Vec3::new(1.0, 0.0, 18.0),
            Vec3::new(1.0, 1.0, 1.0),
            2.0,
            Rgb([100u8, 0u8, 0u8])
        )
    ));
    scene.add_light(Box::new(
        PointLight::new(Vec3::new(0.0, 10.0, 0.0), 50.0)
    ));
    scene.add_light(Box::new(
        AmbientLight::new(1.0)
    ));
    scene.add_light(Box::new(
        GlobalLight::new(Vec3::new(0.2, 0.6, 0.2), 10.0)
    ));

    for x in 0..1000 {
        for y in 0..1000 {
            let cords = Vec2::new((x as f32 - 500.0), (y as f32 - 500.0));

            image.put_pixel(x, y,scene.render_pixel(cords));
        }
    }

    image.save("test.png").unwrap();
}
