use std::io::Read;

use flate2::read::ZlibDecoder;
use swf::DefineBitsLossless;

use image::{ImageBuffer, Pixel, RgbaImage};

pub struct Image {
	pub id: u16,
	pub buffer: RgbaImage,
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
	pub fn from(
		id: u16,
		width: u32,
		height: u32,
		data: Vec<u8>,
	) -> Option<Image> {
		let buffer = ImageBuffer::from_vec(width, height, data);

		if buffer.is_none() {
			None
		} else {
			let mut image = Image {
				id,
				buffer: buffer.unwrap(),
			};

			image.reorder_colors();

			Some(image)
		}
	}

	pub fn reorder_colors(&mut self) {
		for argb in self.buffer.pixels_mut() {
			let channels = argb.0;
			let rgba = argb.channels_mut();

			rgba[0] = channels[1];
			rgba[1] = channels[2];
			rgba[2] = channels[3];
			rgba[3] = channels[0];
		}
	}
}

pub fn extract_image_from_lossless(lossless: &DefineBitsLossless) -> Option<Image> {
	let id: u16 = lossless.id;
	let width: u32 = lossless.width as u32;
	let height: u32 = lossless.height as u32;

	let decompressed = zlib_decode(&lossless.data);

	Image::from(id, width, height, decompressed)
}

fn zlib_decode(data: &Vec<u8>) -> Vec<u8> {
	let mut decompressed: Vec<u8> = Vec::new();

	let bytes: &[u8] = &data;

	ZlibDecoder::new(bytes)
		.read_to_end(&mut decompressed)
		.unwrap();

	decompressed
}
