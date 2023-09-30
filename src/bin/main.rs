use std::io::{stdout, Write};
use std::mem;
use std::time::Instant;
use crossterm::QueueableCommand;
use graphics_learn::canvas::BasicCanvas;
use graphics_learn::math::{AxisAlignedBox};
use graphics_learn::{Canvas, RayIntersection, IntersectionData};
use graphics_learn::octree::{OctTree, OctTreeElement};
use graphics_learn::types::{FVec3, FVec4, Mat4, UVec2};

struct Voxel(FVec3);

impl OctTreeElement for Voxel {
    type Meta = FVec3;
    type RayIter<'a, F: Fn(IntersectionData<()>) -> bool + Copy> = RayIter;
    const SIZE: FVec3 = FVec3::new(1.0, 1.0, 1.0);

    fn intersect_in_place(
            &self,
            pos: FVec3,
            extents: FVec3,
            origin: FVec3,
            direction: FVec3
        ) -> Option<IntersectionData<Self::Meta>>
    {
        AxisAlignedBox::from_position_and_extents(
            pos,
            extents
        ).intersect(origin, direction).map(| i | i.with_meta(| _ | self.0))
    }

    fn build_ray_iterator<'a, F: Fn(IntersectionData<()>) -> bool + Copy>(
            &'a self,
            pos: FVec3,
            extents: FVec3,
            origin: FVec3,
            direction: FVec3,
            _filter: F
        ) -> Self::RayIter<'a, F>
    {
        RayIter {
            int: self.intersect_in_place(pos, extents, origin, direction)
        }
    }
}

struct RayIter {
    int: Option<IntersectionData<FVec3>>
}

impl Iterator for RayIter {
    type Item = IntersectionData<FVec3>;

    fn next(&mut self) -> Option<Self::Item> {
        mem::replace(&mut self.int, None)
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
                    Some(Voxel(FVec3::new(1.0, 0.0, 0.0))),
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(Voxel(FVec3::new(0.1, 0.8, 0.0))),
                    None
                ]
            )
        )
    );
    let mut octree_builder = std::iter::from_fn(move ||
        Some(
            OctTree::new_from_childs(
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
            )
        )
    );
    let mut octree_builder = std::iter::from_fn(move ||
        Some(
            OctTree::new_from_childs(
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
            )
        )
    );
    // let mut octree_builder = std::iter::from_fn(move ||
    //     Some(
    //         OctTree::new_from_childs(
    //             [
    //                 octree_builder.next(),
    //                 octree_builder.next(),
    //                 octree_builder.next(),
    //                 octree_builder.next(),
    //                 octree_builder.next(),
    //                 octree_builder.next(),
    //                 octree_builder.next(),
    //                 octree_builder.next()
    //             ]
    //         )
    //     )
    // );
    let mut octree_builder = octree_builder.map(| t | Box::new(t));
    let mut octree_builder = std::iter::from_fn(move ||
        Some(
            OctTree::new_from_childs(
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
            )
        )
    );
    let mut octree_builder = std::iter::from_fn(move ||
        Some(
            OctTree::new_from_childs(
                [
                    None,
                    octree_builder.next(),
                    octree_builder.next(),
                    octree_builder.next(),
                    octree_builder.next(),
                    octree_builder.next(),
                    octree_builder.next(),
                    octree_builder.next()
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

    println!("{}", mem::size_of_val(&octree));

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
        let cam_pos = FVec3::new((total_time / 10.0).sin() * 2.0, 0.0, 0.6);
        let light_pos = FVec3::new(2.0, 1.0, 0.0);

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
                let int = octree.intersect(
                    cam_pos,
                    dir
                ).filter(| int | int.min > 0.0);

                // let mut ray_iter = octree.build_ray_iterator(
                //     FVec3::zeros(),
                //     FVec3::new(0.5, 0.5, 0.5),
                //     cam_pos,
                //     dir,
                //     | i | i.max > 0.0
                // );


                if let Some(int) = int {
                    let int_point = cam_pos + dir * int.min;
                    let light_dir = (light_pos - int_point).normalize();

                    let mut light_int = octree.build_ray_iterator(
                        FVec3::zeros(),
                        FVec3::new(0.5, 0.5, 0.5),
                        int_point + int.in_normal * 0.01,
                        light_dir,
                        | i | i.max > 0.0
                    ).filter(| i | i.min > 0.0);

                    let light_mul = match light_int.next() {
                        Some(_) => 0.1,
                        None => 1.0,
                    };

                    canvas.put_pixel(pos, int.meta * light_mul)
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