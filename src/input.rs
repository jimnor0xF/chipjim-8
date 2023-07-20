use sdl2::EventPump;

pub struct Input {
    event_pump: EventPump,
}

impl Input {
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        let event_pump = sdl_context.event_pump().unwrap();
        Input { event_pump }
    }

    pub fn poll(&mut self) -> bool {
        for event in self.event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => return false,
                _ => {}
            }
        }
        true
    }
}
