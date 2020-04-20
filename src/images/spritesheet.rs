use image::ImageResult;
use image::imageops::resize;

use crate::images::images::Image;
use crate::buffer::buffer::ImageBuffer;
use self::image::imageops::FilterType;

extern crate image;

pub struct SpriteSheet {
    pub buffer: ImageBuffer,
}

impl SpriteSheet {}

pub fn create_spritesheet(images: Vec<Image>) -> Option<SpriteSheet> {
    if images.len() == 0 {
        None
    } else {
        let width = images.iter().max_by_key(|i| i.buffer.width).unwrap().buffer.width;

        let mut height: u32 = 0;
        let mut data: Vec<u8> = Vec::<u8>::new();

        for image in images.iter() {
            height += image.buffer.height + 10;

            let old_data = image.buffer.get_buffer();
            let bf: image::RgbaImage = image::ImageBuffer::from_raw(image.buffer.width, image.buffer.height, old_data.to_vec()).unwrap();
            let mut new_img: image::RgbaImage = image::ImageBuffer::new(width, image.buffer.height + 10);

            for (x, y, rgba) in bf.enumerate_pixels() {
                new_img.put_pixel(x, y, *rgba);
            }

            data.extend(new_img.into_raw())
        }

        Some(SpriteSheet {
            buffer: ImageBuffer::from(width, height, data)
        })
    }
}