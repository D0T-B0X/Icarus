pub use macroquad::prelude::*;

use super::config::DT;

const BIRD_BYTES: &[u8] = include_bytes!("../sprites/bird.png");
const PIPE_TOP_BYTES: &[u8] = include_bytes!("../sprites/pipe_top.png");
const PIPE_LEN_BYTES: &[u8] = include_bytes!("../sprites/pipe_len.png");
const CLOUD_ONE_BYTES: &[u8] = include_bytes!("../sprites/cloud_one.png");
const CLOUD_TWO_BYTES: &[u8] = include_bytes!("../sprites/cloud_two.png");
const CLOUD_THREE_BYTES: &[u8] = include_bytes!("../sprites/cloud_three.png");

pub struct Renderer {
    camera: Camera2D,
    bird_tex: Texture2D,
    pipe_top_tex: Texture2D,
    pipe_len_tex: Texture2D,
    clouds_tex: Vec<Texture2D>
}

impl Renderer {

    /// initialize game camera and textures
    pub async fn init() -> Self {
        let mut camera = Camera2D::from_display_rect(
            Rect::new(0., 0., 1. ,1.)
        );
        camera.zoom.y *= -1.0;

        let load_png = |bytes: &[u8]| {
            let tex = Texture2D::from_file_with_format(bytes, Some(ImageFormat::Png));
            tex.set_filter(FilterMode::Nearest);
            tex
        };

        Renderer {
            camera,
            bird_tex: load_png(BIRD_BYTES),
            pipe_top_tex: load_png(PIPE_TOP_BYTES),
            pipe_len_tex: load_png(PIPE_LEN_BYTES),
            clouds_tex: vec![
                load_png(CLOUD_ONE_BYTES),
                load_png(CLOUD_TWO_BYTES),
                load_png(CLOUD_THREE_BYTES)
            ]
        }
    }

    /// render frame updates: screen clear
    pub async fn draw(&self) {
        clear_background(SKYBLUE);
        set_camera(&self.camera);
    }
}