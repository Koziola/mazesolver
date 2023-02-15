use image::DynamicImage;

pub struct Maze {
    pub pixels: DynamicImage,
}

impl Maze {
    pub fn height(&self) -> i32 {
        return self.pixels.height() as i32;
    }

    pub fn width(&self) -> i32 {
        return self.pixels.width() as i32;
    }
}
