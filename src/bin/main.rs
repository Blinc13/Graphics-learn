use std::io::{stdout, Write};
use graphics_learn::canvas::BasicCanvas;
use graphics_learn::math::{AxisAlignedBox, Plain};
use graphics_learn::{Canvas, RayIntersection};
use graphics_learn::types::{FVec2, FVec3, UVec2};

fn main() {
    let size = crossterm::terminal::size()
        .map(| s | UVec2::new(s.0 as u32, s.1 as u32))
        .unwrap();
    let mut canvas = BasicCanvas::new(size, FVec3::new(0.1, 0.5, 1.0));


    {
        let cam_pos = FVec3::new(0.0, 0.0, -3.0);
        let b = Plain {
            normal: FVec3::new(0.0, 0.0, 1.0).normalize(),
            dist: 0.0
        };

        for x in 0..size.x {
            for y in 0..size.y {
                let pos = UVec2::new(x, y);
                let render_p = FVec3::new(
                    (x as f32 - size.x as f32 / 2.0) / size.x as f32,
                    (y as f32 - size.y as f32 / 2.0) / size.y as f32,
                    0.0
                ) + FVec3::new(0.0, 0.0, 0.1) + cam_pos;

                let dir = (cam_pos - render_p).normalize();

                if let Some((min, max)) = b.intersect(cam_pos, dir) {
                    if min < 0.0 {
                        continue;
                    }

                    canvas.put_pixel(pos, FVec3::new(min / 10.0, 0.0, 0.0));
                }
            }
        }
    }


    let mut out = stdout();

    canvas.write_queued(&mut out).unwrap();

    out.flush().unwrap();
}