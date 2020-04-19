use swf::DefineBitsLossless;
use flate2::read::ZlibDecoder;
use std::io::Read;
use image::ImageResult;

extern crate image;

pub struct Image {
    pub id: u16,
    pub width: u32,
    pub height: u32,
    data: Vec<u8>,
}

impl Clone for Image {
    fn clone(&self) -> Self {
        Image {
            id: self.id,
            width: self.width,
            height: self.height,
            data: self.data.clone()
        }
    }
}

impl Image {
    pub fn from(id: u16, width: u32, height: u32, data: Vec<u8>) -> Image {
        let mut image = Image {
            id,
            width,
            height,
            data,
        };

        image.reorder_colors();

        image
    }

    pub fn reorder_colors(&mut self) {
        let mut reordered = Vec::<u8>::new();
        let mut iter = self.data.iter();

        for _ in 0..self.data.len() / 4 {
            let a = iter.next().unwrap();

            reordered.push(*iter.next().unwrap());
            reordered.push(*iter.next().unwrap());
            reordered.push(*iter.next().unwrap());
            reordered.push(*a);
        }

        self.data = reordered
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

pub fn extract_image_from_lossless(lossless: &DefineBitsLossless) -> Image {
    let id: u16 = lossless.id;
    let width: u32 = lossless.width as u32;
    let height: u32 = lossless.height as u32;

    let decompressed = zlib_decode(&lossless.data);

    Image::from(id, width, height, decompressed)
}

fn zlib_decode(data: &Vec<u8>) -> Vec<u8> {
    let mut decompressed: Vec<u8> = Vec::new();

    let bytes: &[u8] = &data;

    ZlibDecoder::new(bytes).read_to_end(&mut decompressed).unwrap();

    decompressed
}