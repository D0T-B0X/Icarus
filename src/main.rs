mod config;
mod renderer;
mod physics;

use renderer::Renderer;
use config::window_conf;

use macroquad::prelude::*;
use macroquad::miniquad::window;
use macroquad::prelude::KeyCode;

#[macroquad::main(window_conf)]
async fn main() {
    let render: Renderer = Renderer::init().await;

    loop {
        if is_key_pressed(KeyCode::Space) {
            window::quit();
            break;
        }

        render.draw().await;
        next_frame().await;
    }
}