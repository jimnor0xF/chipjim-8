use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::rect::Rect;

const WINDOW_TITLE: &str = "chip8-rust-emu";
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
const PIXEL_WIDTH: u32 = 64;
const PIXEL_HEIGHT: u32 = 32;

pub struct Display {
    pub canvas: Canvas<sdl2::video::Window>,
}

impl Display {
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window(WINDOW_TITLE, WINDOW_WIDTH, WINDOW_HEIGHT)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();
        canvas.set_draw_color(Color::RGB(0,0,0));
        canvas.clear();
        canvas.present();

        Display { canvas }
    }

    pub fn draw(&mut self, x: i32, y: i32) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();

        let rect = Rect::new(x,y, PIXEL_WIDTH, PIXEL_HEIGHT);
        self.canvas.set_draw_color(Color::RGB(0, 255, 0));
        self.canvas.draw_rect(rect).unwrap();
        self.canvas.fill_rect(rect).unwrap();
        self.canvas.present();
    }
}
