use std::io::{stdout, Write};
use std::time::Instant;
use crossterm::QueueableCommand;
use smallvec::SmallVec;
use graphics_learn::canvas::BasicCanvas;
use graphics_learn::math::{AxisAlignedBox};
use graphics_learn::{Canvas, RayIntersection};
use graphics_learn::octree::{OctTree, OctTreeElement};
use graphics_learn::types::{FVec3, FVec4, Mat4, UVec2};

struct Voxel;

impl OctTreeElement for Voxel {
    fn recursive_ray_intersection(&self, pos: FVec3, extents: FVec3, origin: FVec3, direction: FVec3) -> Option<(f32, f32)> {
        AxisAlignedBox::from_position_and_extents(
            pos,
            extents
        ).intersect(origin, direction)
    }
}

fn main() {
    let size = crossterm::terminal::size()
        .map(| s | UVec2::new(s.0 as u32, s.1 as u32))
        .unwrap();
    let mut canvas = BasicCanvas::new(size, FVec3::new(0.1, 0.5, 1.0));

    let mut octree_builder = std::iter::from_fn(||
        Some(
            OctTree::new_from_childs(
                [
                    Some(Voxel),
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(Voxel),
                    Some(Voxel)
                ]
            )
        )
    );
    let octree = OctTree::new_from_childs(
        [
            octree_builder.next(),
            octree_builder.next(),
            octree_builder.next(),
            octree_builder.next(),
            octree_builder.next(),
            octree_builder.next(),
            octree_builder.next(),
            octree_builder.next()
        ]
    );

    let mut last_time = Instant::now();
    let mut total_time = 0.0;

    loop {
        {
            let passed = Instant::now();
            let dur = passed.duration_since(last_time);

            last_time = passed;
            total_time += dur.as_secs_f32();
        }

        let rotation = Mat4::new_rotation(FVec3::new(0.0, 0.0, 0.0));
        let cam_pos = FVec3::new((total_time / 10.0).sin() * 2.0, 0.0, 1.0);

        for x in 0..size.x {
            for y in 0..size.y {
                let pos = UVec2::new(x, y);
                let render_p = FVec4::new(
                    (x as f32 - size.x as f32 / 2.0) / size.x as f32 * 2.0,
                    (y as f32 - size.y as f32 / 2.0) / size.y as f32,
                    0.0,
                    0.0
                ) + FVec4::new(0.0, 0.0, 0.3, 1.0);
                let render_p = (rotation * render_p).xyz();

                let dir = (cam_pos - (render_p + cam_pos)).normalize();

                if let Some((min, max)) = octree.intersect(cam_pos, dir) {
                    if min < 0.0 {
                       continue;
                    }

                    canvas.put_pixel(pos, FVec3::new(max - min, 0.0, 0.0));
                }
            }
        }

        {
            let mut out = stdout();

            canvas.write_queued(&mut out).unwrap();
            out.queue(crossterm::cursor::MoveTo(0, 0)).unwrap();

            out.flush().unwrap();
        }

        canvas.fill(FVec3::new(0.1, 0.5, 1.0));
    }
}