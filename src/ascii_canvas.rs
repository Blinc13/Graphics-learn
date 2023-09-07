use crossterm::style::{Color, Print, SetForegroundColor};
use crate::Canvas;
use crate::types::*;

pub struct ASCIICanvas {
    data: Vec<Vec<FVec3>>,
    resolution: UVec2
}

impl ASCIICanvas {
    pub fn new(resolution: UVec2, initial_val: FVec3) -> Self {
        let data = vec![
            Vec::from_iter(std::iter::from_fn(move || Some(initial_val)).take(resolution.x as usize)); resolution.y as usize
        ];

        Self {
            resolution,
            data
        }
    }

    pub fn write<T: std::io::Write>(&self, mut dst: T) -> crossterm::Result<()> {
        for line in self.data.iter() {
            for pixel in line.iter() {
                let color = Color::Rgb {
                    r: (pixel.x * 255.0) as u8,
                    g: (pixel.y * 255.0) as u8,
                    b: (pixel.z * 255.0) as u8
                };

                crossterm::queue!(
                    dst,
                    SetForegroundColor(color),
                    Print("█")
                )?
            }

            crossterm::queue!(
                dst,
                Print("\n")
            )?
        }

        Ok(())
    }
}

impl Canvas for ASCIICanvas {
    fn get_resolution(&self) -> UVec2 {
        self.resolution
    }

    fn read_pixel(&self, pos: UVec2) -> FVec3 {
        self.data[pos.y as usize][pos.x as usize]
    }

    fn put_pixel(&mut self, pos: UVec2, val: FVec3) {
        self.data[pos.y as usize][pos.x as usize] = val
    }

    fn fill(&mut self, val: FVec3) {
        self.data.iter_mut()
            .map(| i | i.iter_mut())
            .flatten()
            .for_each(| c | *c = val)
    }
}