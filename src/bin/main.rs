use std::io::{BufWriter, stdout, Write};
use std::time::Instant;
use image::GenericImageView;
use nalgebra::SimdPartialOrd;
use graphics_learn::{Rasterizer, ascii_canvas::ASCIICanvas, painter::BasicPainter, rasterizer::BasicRasterizer, math::CamBasis, types::{
    FVec3,
    FVec4,
    UVec2,
    Mat4
}, Canvas};
use graphics_learn::types::FVec2;

fn main() {
    let mut stdout = BufWriter::new(
        stdout().lock()
    );

    let texture = {
        image::open("dirt.jpg").unwrap()
    };
    let text_size = texture.dimensions();
    let text_size = FVec2::new((text_size.0 - 1) as f32, (text_size.1 - 1) as f32);

    let size = crossterm::terminal::size().map(| s | UVec2::new(s.0 as u32, s.1 as u32)).unwrap();
    let mut painter = BasicPainter::new(
        ASCIICanvas::new(
            size,
            FVec3::new(0.0, 0.0, 0.0)
        )
    );
    let mut z_buffer = Some(
        ASCIICanvas::new(
            size,
        FVec3::new(100.0, 100.0, 100.0)
        )
    );

    let obj = obj::Obj::load("model.obj").unwrap();

    let mut camera = CamBasis::new();
    camera.translate(FVec3::new(0.0, 1.0, 0.0));
    camera.rotate(camera.x, -0.4);

    let mut total_time: f32 = 0.0;
    let mut now = Instant::now();

    loop {
        let _delta = {
            let passed = Instant::now();
            let delta = passed.duration_since(now).as_secs_f32();

            now = passed;
            total_time += delta;

            delta
        };

        let mut rastr = BasicRasterizer::new(
            painter,
            z_buffer
        );

        let cam_mat = camera.build_look_at_matrix();
        let proj_mat = Mat4::new_perspective(size.x as f32 * 0.4 / size.y as f32, 90.0, 0.1, 4000.0);
        let model_mat = Mat4::new_translation(&FVec3::new(0.0, 0.0, -10.0)) * Mat4::new_scaling(5.0)
            * Mat4::new_rotation(FVec3::new(0.0, total_time / 4.0, 0.0))
            * Mat4::new_rotation(FVec3::new(3.14, 0.0, 0.0));

        let transform = proj_mat * cam_mat * model_mat;

        obj.data.objects
            .iter()
            .map(| o | o.groups.iter())
            .flatten()
            .map(| g | g.polys.iter())
            .flatten()
            .map(| p | p.0.windows(3))
            .flatten()
            .map(| t | {
                let pos = {
                    let pos_0 = obj.data.position[t[0].0];
                    let pos_1 = obj.data.position[t[1].0];
                    let pos_2 = obj.data.position[t[2].0];

                    [
                        FVec3::new(pos_0[0], pos_0[1], pos_0[2]),
                        FVec3::new(pos_1[0], pos_1[1], pos_1[2]),
                        FVec3::new(pos_2[0], pos_2[1], pos_2[2])
                    ]
                };
                let uv = {
                    let uv_0 = obj.data.texture[t[0].1.unwrap()];
                    let uv_1 = obj.data.texture[t[1].1.unwrap()];
                    let uv_2 = obj.data.texture[t[2].1.unwrap()];

                    [
                        FVec2::new(uv_0[0], uv_0[1]),
                        FVec2::new(uv_1[0], uv_1[1]),
                        FVec2::new(uv_2[0], uv_2[1])
                    ]
                };

                (pos, uv)
            })
            .for_each(| (v, uv) |
                rastr.draw_triangle(
                    v,
                    |v| transform * FVec4::new(v.x, v.y, v.z, 1.0),
                    | b, _ | {
                        let uv = uv[0] * b.x + uv[1] * b.y + uv[2] * b.z;
                        let uv = uv.component_mul(&text_size)
                            .simd_min(text_size);
                        let c = texture.get_pixel(uv.x as u32, uv.y as u32);

                        FVec3::new(c.0[0] as f32 / 255.0, c.0[1] as f32 / 255.0, c.0[2] as f32 / 255.0)
                    }
                )
            );

        (painter, z_buffer) = rastr.into_inner();

        painter.as_inner().write(&mut stdout).unwrap();

        if let Some(z_buffer) = z_buffer.as_mut() {
            //z_buffer.write(stdout()).unwrap();
            z_buffer.fill(FVec3::new(2.0, 2.0, 2.0));
        }
        {
            let mut canvas = painter.into_inner();

            canvas.fill(FVec3::new(0.0, 0.0, 0.0));

            painter = BasicPainter::new(canvas);
        }

        crossterm::queue!(
            stdout,
            crossterm::cursor::MoveTo(0, 0)
        );

        stdout.flush().unwrap();
    }
}