#[macro_use]
extern crate glium;

mod core;

use core::Core;
use core::scene_manager::SceneManager;

fn main() {
    let mut core = Core::new();

    core.start();
}