use glam::Vec2;
use image::DynamicImage::ImageRgb8;
use graphics_learn::algorithms::trace_sphere;

fn main() {
    let mut image = image::RgbImage::new(1000, 1000);

    for x in 0..1000 {
        for y in 0..1000 {
            let cords = Vec2::new((x as f32 - 500.0), (y as f32 - 500.0));

            image.put_pixel(x as u32, y as u32, trace_sphere(cords));
        }
    }

    image.save("test.png").unwrap();
}
