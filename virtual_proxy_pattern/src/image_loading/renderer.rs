use super::Image;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time;
use std::time::Duration;
use termion;

pub struct Renderer {
    internal_renderer: Arc<Mutex<InternalRenderer>>,
}

impl Renderer {
    pub fn new(image: Box<dyn Image + Send>) -> Self {
        let internal_renderer = Arc::new(Mutex::new(InternalRenderer {
            image,
            stop_flag: false,
            start_time: time::SystemTime::now(),
        }));
        Renderer { internal_renderer }
    }
    pub fn start(&self) {
        let renderer = Arc::clone(&self.internal_renderer);
        renderer.lock().unwrap().set_start();
        thread::spawn(move || loop {
            renderer.lock().unwrap().paint();
            thread::sleep(Duration::from_millis(10));
            if renderer.lock().unwrap().is_stopped() {
                break;
            }
        });
    }
    pub fn stop(&self) {
        self.internal_renderer.lock().unwrap().set_stop();
    }
}

struct InternalRenderer {
    image: Box<dyn Image + Send>,
    stop_flag: bool,
    start_time: time::SystemTime,
}

impl InternalRenderer {
    pub fn paint(&self) {
        println!(
            "{}{}Elapsed time: {:?}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            self.start_time.elapsed().unwrap().as_secs_f32()
        );
        println!("{}", self.image.paint());
    }
    pub fn set_stop(&mut self) {
        self.stop_flag = true;
    }
    pub fn set_start(&mut self) {
        self.stop_flag = false;
    }
    pub fn is_stopped(&self) -> bool {
        self.stop_flag
    }
}
