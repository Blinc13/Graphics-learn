use crossterm::style::{Color, SetForegroundColor};
use crate::Canvas;
use crate::types::{FVec3, UVec2};

pub struct BasicCanvas {
    data: Vec<Vec<FVec3>>,

    size: UVec2
}

impl BasicCanvas {
    pub fn new(size: UVec2, value: FVec3) -> Self {
        let data = vec![
            Vec::from_iter(std::iter::from_fn(|| Some(value)).take(size.x as usize)); size.y as usize
        ];

        Self {
            data,
            size
        }
    }

    pub fn write_queued<T: crossterm::QueueableCommand>(&self, mut dst: T) -> crossterm::Result<()> {
        use crossterm::style::{
            Print,
            Color,
            SetForegroundColor
        };

        for line in self.data.iter() {
            for pixel in line.iter() {
                let pixel = pixel * 255.0;
                let pixel = pixel.map(| c | c as u8);

                let color = Color::Rgb {
                    r: pixel.x,
                    g: pixel.y,
                    b: pixel.z
                };

                dst
                    .queue(SetForegroundColor(color))?
                    .queue(Print("█"))?;
            }

            dst.queue(Print("\n"))?;
        }

        Ok(())
    }
}

impl Canvas for BasicCanvas {
    fn get_resolution(&self) -> UVec2 {
        self.size
    }

    fn fill(&mut self, val: FVec3) {
        self.data.iter_mut()
            .map(| v | v.iter_mut())
            .flatten()
            .for_each(| v | *v = val)
    }

    fn put_pixel(&mut self, pos: UVec2, val: FVec3) {
        self.data[pos.y as usize][pos.x as usize] = val
    }

    fn read_pixel(&self, pos: UVec2) -> FVec3 {
        self.data[pos.y as usize][pos.x as usize]
    }
}