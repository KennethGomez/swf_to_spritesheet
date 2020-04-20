use image::ImageResult;

pub struct ImageBuffer {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

impl Clone for ImageBuffer {
    fn clone(&self) -> Self {
        ImageBuffer {
            width: self.width,
            height: self.height,
            data: self.data.clone()
        }
    }
}

impl ImageBuffer {
    pub fn from(width: u32, height: u32, data: Vec<u8>) -> ImageBuffer {
        ImageBuffer {
            width,
            height,
            data
        }
    }

    pub fn get_buffer(&self) -> &[u8] {
        &self.data
    }

    pub fn save(&self, path: String) -> ImageResult<()> {
        image::save_buffer(
            path,
            self.get_buffer(),
            self.width,
            self.height,
            image::ColorType::Rgba8,
        )
    }
}