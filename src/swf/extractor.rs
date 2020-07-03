use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Result};
use std::path::{Path, PathBuf};

use swf::{Swf, SymbolClassLink, Tag};

use crate::images::image::{extract_image_from_lossless, Image};
use crate::swf::object::SwfObject;

pub struct Extractor {
	swf_path: PathBuf,
}

impl Extractor {
	pub fn new<P>(swf_path: P) -> Extractor
	where
		P: AsRef<Path>,
	{
		Extractor {
			swf_path: swf_path.as_ref().to_path_buf(),
		}
	}

	pub fn extract_swf(&self) -> SwfObject {
		let mut tmp_symbols: Vec<SymbolClassLink> = Vec::new();
		let mut tmp_images: HashMap<u16, Image> = HashMap::new();

		let swf = self.read_swf().expect("Error reading SWF file");

		self.extract_tags_into(swf.tags, &mut tmp_symbols, &mut tmp_images);

		let images = self.bind_images_and_symbols(tmp_symbols, tmp_images);

		SwfObject { images }
	}

	fn read_swf(&self) -> Result<Swf> {
		let file = File::open(&self.swf_path)?;
		let reader = BufReader::new(file);

		swf::read_swf(reader)
	}

	fn extract_tags_into(
		&self,
		tags: Vec<Tag>,
		symbols: &mut Vec<SymbolClassLink>,
		images: &mut HashMap<u16, Image>,
	) {
		for tag in tags {
			match tag {
				Tag::SymbolClass(symbol_class_links) => {
					symbols.extend(symbol_class_links);
				}
				Tag::DefineBitsLossless(lossless) => {
					let id = lossless.id;

					let image_lossless = extract_image_from_lossless(lossless)
						.expect(format!("Error creating image #{}", id).as_str());

					images.insert(image_lossless.id, image_lossless);
				}
				_ => {}
			}
		}
	}

	fn bind_images_and_symbols(
		&self,
		symbols: Vec<SymbolClassLink>,
		mut u16_images: HashMap<u16, Image>,
	) -> HashMap<String, Image> {
		let mut images: HashMap<String, Image> = HashMap::new();

		for symbol in symbols {
			let image = u16_images.remove(&symbol.id);

			if image.is_some() {
				images.insert(symbol.class_name, image.unwrap());
			}
		}

		images
	}
}
