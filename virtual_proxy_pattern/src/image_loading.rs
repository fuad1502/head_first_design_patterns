pub mod ascii_image;
pub mod image_proxy;
pub mod renderer;

pub trait Image {
    fn paint(&self) -> String;
}
