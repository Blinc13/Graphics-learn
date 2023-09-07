use nalgebra::SimdPartialOrd;
use crate::{Canvas, Painter, Rasterizer};
use crate::ascii_canvas::ASCIICanvas;
use crate::painter::BasicPainter;
use crate::types::{FVec2, FVec3, FVec4, UVec2};

pub struct BasicRasterizer<P: Painter, C: Canvas = ASCIICanvas> {
    painter: P,
    z_buffer: Option<BasicPainter<C>>
}

impl<P: Painter, C: Canvas> BasicRasterizer<P, C> {
    pub fn new(painter: P, z_buffer: Option<C>) -> Self {
        if let Some(z_buffer) = z_buffer.as_ref() {
            if painter.canvas_resolution() != z_buffer.get_resolution() {
                panic!("Painter resolution and Z buffer resolution is different")
            }
        }

        let z_buffer = z_buffer.map(| b | BasicPainter::new(b));

        Self {
            painter,
            z_buffer
        }
    }

    pub fn into_inner(self) -> (P, Option<C>) {
        (self.painter, self.z_buffer.map(| b | b.into_inner()))
    }
}

impl<P: Painter, C: Canvas> Rasterizer for BasicRasterizer<P, C> {
    fn draw_triangle<V: Copy + Fn(FVec3) -> FVec4, F: Copy + Fn(FVec3, FVec3) -> FVec3>(&mut self, mut triangle: [FVec3; 3], vert: V, frag: F) {
        let res = self.painter.canvas_resolution().map(| r | r as f32);

        triangle.iter_mut()
            .for_each(| v | *v = compute_vertex(*v, vert));

        { // Backface culling
            let view = FVec3::new(0.0, 0.0, 1.0);

            let v1 = triangle[0] - triangle[1];
            let v2 = triangle[1] - triangle[2];

            let cross = v1.cross(&v2);

            if cross.dot(&view) < 0.0 {
                return
            }
        }

        let flat_triangle = [
            triangle[0].xy().add_scalar(0.5).component_mul(&res),
            triangle[1].xy().add_scalar(0.5).component_mul(&res),
            triangle[2].xy().add_scalar(0.5).component_mul(&res)
        ];

        if let Some(b) = self.z_buffer.as_mut() {
            b.fill_triangle(flat_triangle, | p, v | {
                let barycentric = calc_barycentric(p, flat_triangle);
                let depth = calc_avg(2, barycentric, triangle);

                if !(0.0f32..1.0f32).contains(&depth) {
                    return None
                }

                if depth < v.x {
                    Some(FVec3::new(depth, depth, depth))
                } else {
                    None
                }
            })
        }

        let z_buffer = self.z_buffer.as_ref().map(| p | p.as_inner());

        self.painter.fill_triangle(
            flat_triangle,
            | p, _ | {
                let barycentric = calc_barycentric(p, flat_triangle);
                let frag_cord = FVec3::new(
                    calc_avg(0, barycentric, triangle),
                    calc_avg(1, barycentric, triangle),
                    calc_avg(2, barycentric, triangle)
                );

                if let Some(z_buffer) = z_buffer {
                    let depth = z_buffer.read_pixel(UVec2::new(p.x as u32, p.y as u32));

                    if frag_cord.z == depth.x {
                        Some(frag(barycentric, frag_cord))
                    } else {
                        None
                    }
                } else {
                    Some(frag(barycentric, frag_cord))
                }
            }
        )
    }
}

#[inline(always)]
fn compute_vertex<V: Copy + Fn(FVec3) -> FVec4>(v: FVec3, vert: V) -> FVec3 {
    let v = vert(v);
    let s = v.xyz() / v.w;

    s
    //FVec3::new(s.x, s.y, v.z)
}

#[inline(always)]
fn calc_avg(i: usize, barycentric: FVec3, points: [FVec3; 3]) -> f32 {
    points[0][i] * barycentric.x + points[1][i] * barycentric.y + points[2][i] * barycentric.z
}

#[inline(always)]
fn calc_barycentric(point: FVec2, points: [FVec2; 3]) -> FVec3 {
    crate::math::courtesan_to_barycentric(point, points).simd_clamp(
        FVec3::new(0.0, 0.0, 0.0),
        FVec3::new(1.0, 1.0, 1.0)
    )
}