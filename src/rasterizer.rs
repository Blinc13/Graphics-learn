use crate::{Canvas, Painter, Rasterizer};
use crate::ascii_canvas::ASCIICanvas;
use crate::math::courtesan_to_barycentric;
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

        let flat_triangle = [
            triangle[0].xy().add_scalar(0.5).component_mul(&res),
            triangle[1].xy().add_scalar(0.5).component_mul(&res),
            triangle[2].xy().add_scalar(0.5).component_mul(&res)
        ];

        let left = FVec2::new(-1.0, 0.0);

        if flat_triangle.windows(2).all(| w | w[0].angle(&left) >= w[1].angle(&left)) {
            return
        }

        // { // If one of vertecies outside viewport, denie fill
        //     let bounds_x = flat_triangle.iter().map(|v| v.x);
        //     let bounds_y = flat_triangle.iter().map(|v| v.y);
        //     let bounds_z = triangle.iter().map(| v | v.z);
        //
        //     let comp = | a: &f32, b: &f32 | a.total_cmp(b);
        //
        //     let (x_min, x_max) = (bounds_x.clone().min_by(comp).unwrap(), bounds_x.max_by(comp).unwrap());
        //     let (y_min, y_max) = (bounds_y.clone().min_by(comp).unwrap(), bounds_y.max_by(comp).unwrap());
        //     let (z_min, z_max) = (bounds_z.clone().min_by(comp).unwrap(), bounds_z.max_by(comp).unwrap());
        //
        //     if x_min < 0.0 || x_max > res.x || y_min < 0.0 || y_max > res.y || z_min < -1.0 || z_max > 2.0 {
        //         println!("Triangle not pass! {z_min} {z_max}");
        //
        //         return
        //     }
        // }

        if let Some(b) = self.z_buffer.as_mut() {
            b.fill_triangle(flat_triangle, | p, v | {
                let barycentric = courtesan_to_barycentric(p, flat_triangle);
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
                let barycentric = courtesan_to_barycentric(p, flat_triangle);
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