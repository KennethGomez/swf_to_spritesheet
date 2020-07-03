use std::collections::HashMap;

use crate::images::image::Image;

pub struct SwfObject {
	pub images: HashMap<String, Image>,
}
