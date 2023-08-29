use std::io::stdout;
use std::time::Instant;
use crossterm::ExecutableCommand;
use graphics_learn::{Rasterizer, ascii_canvas::ASCIICanvas, painter::BasicPainter, rasterizer::BasicRasterizer, math::CamBasis, types::{
    FVec3,
    FVec4,
    UVec2,
    Mat4
}, Canvas};

const TRIANGLES: &[[FVec3; 3]] = &[
    [
        FVec3::new(-0.5, -0.5, -0.5),
        FVec3::new(0.5, -0.5, -0.5),
        FVec3::new(0.5,  0.5, -0.5)
    ],
    [
        FVec3::new(0.5,  0.5, -0.5),
        FVec3::new(-0.5,  0.5, -0.5),
        FVec3::new(-0.5, -0.5, -0.5)
    ],
    [
        FVec3::new(-0.5, -0.5,  0.5),
        FVec3::new(0.5, -0.5,  0.5),
        FVec3::new(0.5,  0.5,  0.5)
    ],
    [
        FVec3::new(0.5,  0.5,  0.5),
        FVec3::new(-0.5,  0.5,  0.5),
        FVec3::new(-0.5, -0.5,  0.5)
    ],
    [
        FVec3::new(-0.5,  0.5,  0.5),
        FVec3::new(-0.5,  0.5, -0.5),
        FVec3::new(-0.5, -0.5, -0.5)
    ],
    [
        FVec3::new(-0.5, -0.5, -0.5),
        FVec3::new(-0.5, -0.5,  0.5),
        FVec3::new(-0.5,  0.5,  0.5)
    ],
    [
        FVec3::new(0.5,  0.5,  0.5),
        FVec3::new(0.5,  0.5, -0.5),
        FVec3::new(0.5, -0.5, -0.5)
    ],
    [
        FVec3::new(0.5, -0.5, -0.5),
        FVec3::new(0.5, -0.5,  0.5),
        FVec3::new(0.5,  0.5,  0.5)
    ],
    [
        FVec3::new(-0.5, -0.5, -0.5),
        FVec3::new(0.5, -0.5, -0.5),
        FVec3::new(0.5, -0.5,  0.5)
    ],
    [
        FVec3::new(0.5, -0.5,  0.5),
        FVec3::new(-0.5, -0.5,  0.5),
        FVec3::new(-0.5, -0.5, -0.5)
    ],
    [
        FVec3::new(-0.5,  0.5, -0.5),
        FVec3::new(0.5,  0.5, -0.5),
        FVec3::new(0.5,  0.5,  0.5)
    ],
    [
        FVec3::new(0.5,  0.5,  0.5),
        FVec3::new(-0.5,  0.5,  0.5),
        FVec3::new(-0.5,  0.5, -0.5)
    ]
];

fn main() {
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
        FVec3::new(2.0, 2.0, 2.0)
        )
    );

    let mut camera = CamBasis::new();

    let mut total_time: f32 = 0.0;
    let mut now = Instant::now();

    loop {
        let delta = {
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
        let model_mat = Mat4::new_translation(&FVec3::new(0.0, 0.0, -10.0)) * Mat4::new_scaling(5.0) * Mat4::new_rotation(FVec3::new(0.4, total_time / 4.0, 0.0));

        let transform = proj_mat * cam_mat * model_mat;
        //view * cam * model

        TRIANGLES.iter()
            .copied()
            .for_each(|t| {
                // let t = [
                //     t[2],
                //     t[1],
                //     t[0]
                // ];

                rastr.draw_triangle(
                    t,
                    |v| transform * (FVec4::new(v.x, v.y, v.z, 1.0)),
                    | b, _ | b
                )
            });

        (painter, z_buffer) = rastr.into_inner();

        painter.as_inner().write(stdout()).unwrap();

        if let Some(z_buffer) = z_buffer.as_mut() {
            //z_buffer.write(stdout()).unwrap();
            z_buffer.fill(FVec3::new(2.0, 2.0, 2.0));
        }
        {
            let mut canvas = painter.into_inner();

            canvas.fill(FVec3::new(0.0, 0.0, 0.0));

            painter = BasicPainter::new(canvas);
        }

        stdout().execute(crossterm::cursor::MoveTo(0, 0)).unwrap();
    }
}