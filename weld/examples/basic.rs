extern crate weld;

use weld::window::WindowFactory;

fn main() {
    let mut window = WindowFactory::new("Demo");
    window.run();
}