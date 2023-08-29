use crate::{Canvas, Painter};
use crate::types::{FVec2, FVec3, UVec2};

pub struct BasicPainter<C: Canvas> {
    canvas: C
}

impl<C: Canvas> BasicPainter<C> {
    fn draw_simple_line<F: Copy + Fn(FVec2, FVec3) -> Option<FVec3>>(&mut self, y: u32, x0: u32, x1: u32, pixel_fill: F) {
        for x in x0..x1 {
            let pos = UVec2::new(x, y);
            let pixel = self.canvas.read_pixel(pos);

            match pixel_fill(FVec2::new(x as f32, y as f32), pixel) {
                Some(color) => self.canvas.put_pixel(pos, color),
                _ => {}
            }
        }
    }

    pub fn new(canvas: C) -> Self {
        Self {canvas}
    }

    pub fn into_inner(self) -> C {
        self.canvas
    }

    pub fn as_inner(&self) -> &C {
        &self.canvas
    }
}

impl<C: Canvas> Painter for BasicPainter<C> {
    fn canvas_resolution(&self) -> UVec2 {
        self.canvas.get_resolution()
    }

    fn draw_line<F: Copy + Fn(FVec2, FVec3) -> Option<FVec3>>(&mut self, start: FVec2, end: FVec2, pixel_fill: F) {
        let pixel_count = start - end;
        let pixel_count = (pixel_count.x.abs() + pixel_count.y.abs()) as u32;

        for f in 0..pixel_count {
            let pos = crate::math::interpolate(start, end, (pixel_count - f) as f32 / pixel_count as f32);
            let upos = UVec2::new(pos.x as u32, pos.y as u32);

            let pixel = self.canvas.read_pixel(upos);

            match pixel_fill(pos, pixel) {
                Some(color) => self.canvas.put_pixel(upos, color),
                _ => ()
            }
        }
    }

    fn draw_triangle_lines<F: Copy + Fn(FVec2, FVec3) -> Option<FVec3>>(&mut self, vert: [FVec2; 3], pixel_fill: F) {
        self.draw_line(vert[0], vert[1], pixel_fill);
        self.draw_line(vert[1], vert[2], pixel_fill);
        self.draw_line(vert[2], vert[0], pixel_fill);
    }

    fn fill_triangle<F: Copy + Fn(FVec2, FVec3) -> Option<FVec3>>(&mut self, mut vert: [FVec2; 3], pixel_fill: F) {
        vert.sort_by(| a, b | a.y.total_cmp(&b.y));

        let res = self.canvas.get_resolution();

        let up = (vert[0].y as u32).clamp(0, res.y);
        let mid = (vert[1].y as u32).clamp(0, res.y);
        let down = (vert[2].y as u32).clamp(0, res.y);

        // Triangle spitted into 2 subtriangles, and fills them
        for y in up..mid {
            let mut x0 = calc_x_by_y(vert[0], vert[1], y).clamp(0, res.x);
            let mut x1 = calc_x_by_y(vert[0], vert[2], y).clamp(0, res.x);

            if x0 > x1 {
                x0 = x0 ^ x1;
                x1 = x1 ^ x0;
                x0 = x0 ^ x1;
            }

            self.draw_simple_line(y, x0, x1, pixel_fill)
        }
        for y in mid..down {
            let mut x0 = calc_x_by_y(vert[1], vert[2], y).clamp(0, res.x);
            let mut x1 = calc_x_by_y(vert[0], vert[2], y).clamp(0, res.x);

            if x0 > x1 {
                x0 = x0 ^ x1;
                x1 = x1 ^ x0;
                x0 = x0 ^ x1;
            }

            self.draw_simple_line(y, x0, x1, pixel_fill);
        }
    }
}

// Assumes what v1 is upper than v2
#[inline(always)]
fn calc_x_by_y(v1: FVec2, v2: FVec2, y: u32) -> u32 {
    let up = v1.y as u32;

    let d = v2.y - v1.y;
    let f = (d - (y - up) as f32) / d;

    crate::math::interpolate(v2.x, v1.x, f) as u32
}