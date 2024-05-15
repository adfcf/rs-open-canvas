pub mod canvas;
pub mod window;
pub mod app;

mod gl;

#[cfg(test)]
mod tests {

    use app::App;
    use window::Config;
    use canvas::Canvas;

    use canvas::Color;

    use self::app::Logic;

    use super::*;

    struct MyLogic {
        value: f64,
    }

    impl Logic for MyLogic {
        fn update_routine(&mut self, time: f64, _window: &window::Window) {
            self.value = (time.cos() * 255f64).abs();
        }
    
        fn render_routine(&self, canvas: &mut Canvas) {
            canvas.fill_ij(|i, j| {
                Color::from_rgb(self.value as u8, (j % 255) as u8, ((i + j) % 255) as u8)
            });
        }
    }

    impl MyLogic {
        fn new() -> Self {
            Self { value: 0.0 }
        }
    }

    #[test]
    fn it_works() {

        let mut my_logic = MyLogic::new();

        let mut app = App::new(Config::default());
        app.run(&mut my_logic);

    }
}
