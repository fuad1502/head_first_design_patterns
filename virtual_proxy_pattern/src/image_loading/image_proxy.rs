use super::ascii_image::ASCIIImage;
use super::Image;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

pub struct ImageProxy {
    image_proxy_impl: Arc<Mutex<ImageProxyImpl>>,
}

impl ImageProxy {
    pub fn new(file_path: PathBuf) -> Self {
        let image_proxy_impl = Arc::new(Mutex::new(ImageProxyImpl {
            image: None,
            file_path,
        }));
        ImageProxy { image_proxy_impl }
    }
}

impl Image for ImageProxy {
    fn paint(&self) -> String {
        self.image_proxy_impl.paint()
    }
}

struct ImageProxyImpl {
    image: Option<ASCIIImage>,
    file_path: PathBuf,
}

impl ImageProxyImpl {
    pub fn set_image(&mut self, image: ASCIIImage) {
        self.image = Some(image);
    }
}

impl Image for Arc<Mutex<ImageProxyImpl>> {
    fn paint(&self) -> String {
        let file_path = self.lock().unwrap().file_path.clone();
        match &self.lock().unwrap().image {
            Some(image) => image.paint(),
            None => {
                let self_clone = Arc::clone(&self);
                thread::spawn(move || {
                    let image = ASCIIImage::new(file_path);
                    self_clone.lock().unwrap().set_image(image);
                });
                String::from("Loading image...")
            }
        }
    }
}
