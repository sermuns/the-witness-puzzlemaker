use macroquad::{miniquad::window::screen_size, prelude::*};

fn window_conf() -> Conf {
    Conf {
        window_title: env!("CARGO_PKG_NAME").into(),
        ..Default::default()
    }
}

#[derive(Default)]
struct App {
    paused: bool,
}

impl App {
    fn toggle_pause(&mut self) {
        if self.paused {
            self.paused = false;
            set_cursor_grab(true);
            show_mouse(false);
        } else {
            self.paused = true;
            set_cursor_grab(false);
            show_mouse(true);
        }
    }

    pub fn handle_user_input(&mut self) {
        if is_key_pressed(KeyCode::Escape)
            || (self.paused && is_mouse_button_pressed(MouseButton::Left))
        {
            self.toggle_pause();
        }
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut app = App::default();

    loop {
        app.handle_user_input();

        clear_background(BLACK);

        let (screen_width_px, screen_height_px) = screen_size();
        let (screen_center_x_px, screen_center_y_px) =
            (screen_width_px / 2., screen_height_px / 2.);

        let (rect_width_px, rect_height_px) = (800., 800.);

        let (rect_left_px, rect_top_px) = (
            screen_center_x_px - rect_width_px / 2.,
            screen_center_y_px - rect_height_px / 2.,
        );
        draw_rectangle(
            rect_left_px,
            rect_top_px,
            rect_width_px,
            rect_height_px,
            RED,
        );

        next_frame().await
    }
}
