use std::fs::File;
use std::io::BufReader;
use std::time::Instant;
use std::collections::HashMap;

use swf::{Tag, SymbolClassLink};

use figure_extractor::images::{images, spritesheet};
use figure_extractor::images::images::Image;

extern crate image;

fn main() {
    let symbols = None;
    let images = HashMap::new();
    let images_by_name = HashMap::new();

    extract_swf(symbols, images, images_by_name);
}

fn extract_swf(
    mut symbols: Option<Vec<SymbolClassLink>>,
    mut images: HashMap<u16, images::Image>,
    mut images_by_name:HashMap<String, images::Image>,
) {
    let file = File::open("D:\\Rust\\figure_extractor\\Hair_F_Bob.swf").unwrap();

    let reader = BufReader::new(file);
    let swf = swf::read_swf(reader).unwrap();
    let now = Instant::now();

    for tag in &swf.tags {
        if let Tag::SymbolClass(symbol_class_links) = &tag {
            symbols = Some(symbol_class_links.clone());
        }

        if let Tag::DefineBitsLossless(lossless) = &tag {
            let image = images::extract_image_from_lossless(lossless);

            images.insert(image.id, image);
        }
    }

    if symbols.is_some() {
        for symbol in symbols.unwrap() {
            let image = images.get(&symbol.id);

            if image.is_some() {
                images_by_name.insert(symbol.class_name, image.unwrap().clone());
            }
        }
    }

    let mut images = Vec::<Image>::new();

    for (_, image) in images_by_name {
        images.push(image)
    }

    let file_name = format!("images/{}.png", "test");
    let spritesheet = spritesheet::create_spritesheet(images);

    if spritesheet.is_some() {
        spritesheet.unwrap().buffer.save(file_name).unwrap()
    }

    // for (name, image) in images_by_name {
    //     let file_name = format!("images/{}.png", name);
    //
    //     image.buffer.save(file_name).unwrap();
    // }

    println!("elapsed {}ms", now.elapsed().as_millis());
}
