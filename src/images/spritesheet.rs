use std::fs::File;
use std::io::Write;
use std::path::Path;

use image::{ImageBuffer, RgbaImage};

use crate::images::image::Image;

extern crate sheep;

pub struct SpriteSheet {
	name: String,
	pub buffer: RgbaImage,
	pub meta: Option<sheep::SerializedSpriteSheet>,
}

impl SpriteSheet {
	pub fn save<P>(
		&self,
		out_path: P,
	) where
		P: AsRef<Path>,
	{
		let output = out_path.as_ref();

		let spritesheet_path = output.join(format!("{}.png", self.name));
		let meta_path = output.join(format!("{}.meta.json", self.name));

		self.buffer
			.save(spritesheet_path)
			.expect("Error saving spritesheet image");

		if self.meta.is_some() {
			let mut meta_file = File::create(meta_path).expect("Failed to create meta file");
			let meta_str = serde_json::to_string(&self.meta).expect("Failed to encode meta file");

			meta_file
				.write_all(meta_str.as_bytes())
				.expect("Failed to write meta file");
		}
	}
}

pub fn create_spritesheet(
	images: Vec<Image>,
	name: String,
) -> Option<SpriteSheet> {
	if images.len() == 0 {
		None
	} else {
		let mut sprites = Vec::<sheep::InputSprite>::new();

		for image in images {
			let input_sprite = sheep::InputSprite {
				dimensions: image.buffer.dimensions(),
				bytes: image.buffer.into_raw(),
			};

			sprites.push(input_sprite)
		}

		let results = sheep::pack::<sheep::SimplePacker>(sprites, 4, ());

		let sprite_sheet = results
			.into_iter()
			.next()
			.expect("Should have returned a spritesheet");

		let meta = sheep::encode::<sheep::AmethystFormat>(&sprite_sheet, ());

		let buffer = image::RgbaImage::from_vec(
			sprite_sheet.dimensions.0,
			sprite_sheet.dimensions.1,
			sprite_sheet.bytes,
		)
		.expect("Failed to construct image from sprite sheet bytes");

		Some(SpriteSheet {
			name,
			buffer,
			meta: Some(meta),
		})
	}
}

pub fn create_spritesheet_native(
	images: Vec<Image>,
	name: String,
) -> Option<SpriteSheet> {
	if images.len() == 0 {
		None
	} else {
		let width = images
			.iter()
			.max_by_key(|i| i.buffer.width())?
			.buffer
			.width();

		let mut height: u32 = 0;
		let mut data: Vec<u8> = Vec::<u8>::new();

		for image in images.iter() {
			height += image.buffer.height();

			let old_data = image.buffer.clone().into_raw();
			let bf: image::RgbaImage = image::ImageBuffer::from_raw(
				image.buffer.width(),
				image.buffer.height(),
				old_data.to_vec(),
			)
			.unwrap();
			let mut new_img: image::RgbaImage =
				image::ImageBuffer::new(width, image.buffer.height());

			for (x, y, rgba) in bf.enumerate_pixels() {
				new_img.put_pixel(x, y, *rgba);
			}

			data.extend(new_img.into_raw())
		}

		Some(SpriteSheet {
			name,
			buffer: ImageBuffer::from_vec(width, height, data).unwrap(),
			meta: None,
		})
	}
}
