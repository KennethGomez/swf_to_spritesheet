use std::env;
use std::fs;
use std::io::Result;
use std::path::Path;
use std::time::Instant;

use swf_to_spritesheet::images::image::Image;
use swf_to_spritesheet::images::spritesheet::create_spritesheet;
use swf_to_spritesheet::swf::extractor::Extractor;

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
		.to_str()
		.unwrap();

	let now = Instant::now();

	let extractor = Extractor::new(swf_file_path);
	let swf = extractor.extract_swf();

	let mut images = Vec::<Image>::new();

	for (_, image_by_name) in swf.images {
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
