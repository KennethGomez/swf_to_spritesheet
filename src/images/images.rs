use swf::DefineBitsLossless;
use flate2::read::ZlibDecoder;
use std::io::Read;

use crate::buffer::buffer::ImageBuffer;

extern crate image;

pub struct Image {
    pub id: u16,
    pub buffer: ImageBuffer,
}

impl Clone for Image {
    fn clone(&self) -> Self {
        Image {
            id: self.id,
            buffer: self.buffer.clone(),
        }
    }
}

impl Image {
    pub fn from(id: u16, width: u32, height: u32, data: Vec<u8>) -> Image {
        let buffer = ImageBuffer::from(width, height, data);

        let mut image = Image {
            id,
            buffer,
        };

        image.reorder_colors();

        image
    }

    pub fn reorder_colors(&mut self) {
        let mut reordered = Vec::<u8>::new();
        let mut iter = self.buffer.data.iter();

        for _ in 0..self.buffer.data.len() / 4 {
            let a = iter.next().unwrap();

            reordered.push(*iter.next().unwrap());
            reordered.push(*iter.next().unwrap());
            reordered.push(*iter.next().unwrap());
            reordered.push(*a);
        }

        self.buffer.data = reordered
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