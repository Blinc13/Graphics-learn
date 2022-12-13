use glam::{Vec2, Vec3};
use image::Rgb;
use crate::Const::Vec;

//<CO + tD, CO + tD> = r
pub fn intersect_sphere(origin: Vec3, direction: Vec3, sphere_pos: Vec3, size: Vec3, r: f32) -> (f32, f32) {
    let CO = (origin - sphere_pos) / size;
    let D = direction / size;

    let a = D.dot(D);
    let b = 2.0 * CO.dot(D);
    let c = CO.dot(CO) - r*r;

    let desc = (b*b) - (4.0 * a * c);

    if desc < 0.0 {
        return (f32::INFINITY, f32::INFINITY);
    }

    let desc = desc.sqrt();

    (
        (-b + desc) / (2.0 * a),
        (-b - desc) / (2.0 * a)
    )
}

pub fn trace_sphere(cords: Vec2) -> Rgb<u8> {
    let direction = Vec3 {
        x: cords.x * (1.0 / 1000.0),
        y: cords.y * (1.0 / 1000.0),
        z: 1.0
    };

    //println!("{} {}", direction.y, direction.x);

    let (t1, t2) = intersect_sphere(
        Vec::ZERO,
        direction,
        Vec::FORWARD * 50.0,
        Vec3::new(1.0, 1.4, 1.0),
        10.0
    );

    if t1 != f32::INFINITY && t2 != f32::INFINITY {
        Rgb::from([t1 as u8, t1 as u8, t1 as u8])
    } else {
        Rgb::from([0u8, 0u8, 0u8])
    }
}