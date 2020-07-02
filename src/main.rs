use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::File;
use std::io::{BufReader, Result};
use std::path::Path;
use std::time::Instant;

use swf::Tag;

use figure_extractor::images::images::{extract_image_from_lossless, Image};
use figure_extractor::images::spritesheet::create_spritesheet;

fn main() {
	let args: Vec<String> = env::args().collect();

	let swf_file_path = args
		.get(1)
		.expect("You must specify SWF path as first parameter");

	let output_path = get_output_path(&args)
		.expect("Error creating output path, please, specify it as the second argument");

	let file_path = Path::new(swf_file_path);

	extract_swf(file_path, output_path).unwrap();
}

fn get_output_path(args: &Vec<String>) -> Result<&Path> {
	let output_path = args.get(2);

	if output_path.is_none() {
		fs::create_dir_all("output")?;

		Ok(Path::new("output"))
	} else {
		Ok(Path::new(output_path.unwrap()))
	}
}

fn extract_swf(
	swf_file_path: &Path,
	output_path: &Path,
) -> Result<()> {
	let file_name = swf_file_path
		.file_stem()
		.expect("Unable to get file name from path")
		.to_str().unwrap();

	let mut symbols = None;
	let mut images: HashMap<u16, Image> = HashMap::new();
	let mut images_by_name: HashMap<String, Image> = HashMap::new();

	let file = File::open(swf_file_path)?;

	let reader = BufReader::new(file);
	let swf = swf::read_swf(reader)?;
	let now = Instant::now();

	for tag in &swf.tags {
		if let Tag::SymbolClass(symbol_class_links) = &tag {
			symbols = Some(symbol_class_links.clone());
		}

		if let Tag::DefineBitsLossless(lossless) = &tag {
			let image_lossless = extract_image_from_lossless(lossless)
				.expect(format!("Error creating image #{}", lossless.id).as_str());

			images.insert(image_lossless.id, image_lossless);
		}
	}

	for symbol in symbols.unwrap_or_default() {
		let image = images.get(&symbol.id);

		if image.is_some() {
			images_by_name.insert(symbol.class_name, image.unwrap().clone());
		}
	}

	let mut images = Vec::<Image>::new();

	for (_, image_by_name) in images_by_name {
		images.push(image_by_name)
	}

	let file_name = format!("{}/{}.png", output_path.to_str().unwrap(), file_name);
	let spritesheet = create_spritesheet(images)
		.expect("Error creating spritesheet: there're no output to create spritesheet");

	spritesheet
		.buffer
		.save(file_name)
		.expect("Error saving spritesheet");

	println!("elapsed {}ms", now.elapsed().as_millis());

	Ok(())
}
