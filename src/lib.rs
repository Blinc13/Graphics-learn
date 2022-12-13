pub mod scene;
pub mod traits;
pub mod objects;

pub mod Const {
    pub mod Vec {
        use glam::Vec3;

        pub const UP: Vec3 = Vec3 {x: 0.0, y: 1.0, z: 0.0};
        pub const DOWN: Vec3 = Vec3 {x: 0.0, y: -1.0, z: 0.0};

        pub const LEFT: Vec3 = Vec3 {x: -1.0, y: 1.0, z: 0.0};
        pub const RIGHT: Vec3 = Vec3 {x: 1.0, y: 1.0, z: 0.0};

        pub const FORWARD: Vec3 = Vec3 {x: 0.0, y: 0.0, z: 1.0};
        pub const BACK: Vec3 = Vec3 {x: 0.0, y: 0.0, z: -1.0};

        pub const ZERO: Vec3 = Vec3 {x: 0.0, y: 0.0, z: 0.0};
    }

    pub mod Colors {
        use image::Rgb;

        pub const RED: Rgb<u8> = Rgb([255u8, 0u8, 0u8]);
        pub const GREEN: Rgb<u8> = Rgb([0u8, 255u8, 0u8]);
        pub const BLUE: Rgb<u8> = Rgb([0u8, 0u8, 255u8]);

        pub const BLACK: Rgb<u8> = Rgb([0u8, 0u8, 0u8]);
        pub const WHITE: Rgb<u8> = Rgb([255u8, 255u8, 255u8]);
    }
}