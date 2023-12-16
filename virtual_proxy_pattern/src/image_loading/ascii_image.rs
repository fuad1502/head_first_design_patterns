use super::Image;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

pub struct ASCIIImage {
    image: String,
}

impl ASCIIImage {
    pub fn new(file_path: PathBuf) -> Self {
        // Delayed loading of image
        thread::sleep(Duration::from_secs(1));
        let image = std::fs::read_to_string(file_path).unwrap();
        ASCIIImage { image }
    }
}

impl Image for ASCIIImage {
    fn paint(&self) -> String {
        self.image.clone()
    }
}
