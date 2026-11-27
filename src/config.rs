use macroquad;

const TITLE: &str           = "Flappy Bird";
const WINDOW_HEIGHT: i32    = 720;
const WINDOW_WIDTH: i32     = 1280;

pub const DT: f32 = 1.0 / 60.0;

pub fn window_conf() -> macroquad::window::Conf {
    macroquad::window::Conf {
        window_title: String::from(TITLE),
        window_height: WINDOW_HEIGHT,
        window_width: WINDOW_WIDTH,
        window_resizable: true,
        fullscreen: false,
        high_dpi: true,
        ..Default::default()
    }
}