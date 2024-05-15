use super::{
    canvas::Canvas, 
    window::{Config, Window, ONE_SECOND}
};

pub struct App {
    window: Window,
    canvas: Canvas,
}

pub trait Logic {
    fn update_routine(&mut self, time: f64, window: &Window);
    fn render_routine(&self, canvas: &mut Canvas);
}

impl App {

    pub fn new(config: Config) -> App {

        let window = Window::new(config);

        let dimension = window.get_canvas_dimension();
        let canvas = Canvas::new(dimension.0, dimension.1);

        App {
            window,
            canvas,
        }

    }

    pub fn run<T>(&mut self, logic: &mut T) 
    where T: Logic
    {

        let mut frame_count = 0;
        let mut timer = self.window.now();
        let mut last_time = timer;

        while !self.window.should_close() {
            timer += self.window.now() - last_time;
            last_time = self.window.now();
            if timer >= ONE_SECOND {
                println!("FPS: {}", frame_count);
                frame_count = 0;
                timer = 0.0;
            }  
            self.window.poll_events();
            // START
            logic.update_routine(self.window.now(), &self.window);
            logic.render_routine(&mut self.canvas);
            // END
            self.window.swap_buffers(&self.canvas);
            frame_count += 1;
        }

    }

    pub fn get_window(&self) -> &Window {
        &self.window
    }

    pub fn toggle_cursor_mode(&mut self) {
        self.window.toggle_cursor_mode();
    }

}