pub mod canvas;
pub mod window;
pub mod app;
pub mod mathsf;

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
    
        fn render_routine(&mut self, canvas: &mut Canvas) {
            canvas.clear(Color::white());
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

        let config = Config::new(String::from("Canvas"), 1.0 / 1.0, 100, 0.5, glfw::SwapInterval::None);

        let mut app = App::new(config);
        app.run(&mut my_logic);

    }
}
