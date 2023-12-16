use virtual_proxy_pattern::image_loading;

use image_loading::image_proxy::ImageProxy;
use image_loading::renderer::Renderer;

use std::path::PathBuf;
use std::thread;
use std::time::Duration;

#[test]
fn test() {
    println!("======[ image_loading_test.rs start ]======");
    let mut file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    file_path.push("resources/ascii_art.txt");
    let image = ImageProxy::new(file_path);
    let renderer = Renderer::new(Box::new(image));
    renderer.start();
    thread::sleep(Duration::from_secs(2));
    renderer.stop();
    println!("======[ image_loading_test.rs end ]======");
}
