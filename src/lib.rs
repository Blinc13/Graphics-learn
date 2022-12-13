pub mod algorithms;

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

    pub mod Funcs {
        use glam::Vec3;

        pub fn length(vec: Vec3) -> f32 {
            let tmp = vec * vec;

            (tmp.x + tmp.y + tmp.z).sqrt()
        }

        pub fn dot(fvec: Vec3, svec: Vec3) -> f32 {
            fvec.length() * (svec.length() * fvec.angle_between(svec).cos())
        }
    }
}