use macroquad::{miniquad::window::screen_size, prelude::*};

const CURSOR_SIZE: f32 = 15.;

fn window_conf() -> Conf {
    Conf {
        window_title: env!("CARGO_PKG_NAME").into(),
        ..Default::default()
    }
}

#[derive(Debug)]
struct PuzzleCorner {
    row: usize,
    column: usize,
}

type PuzzleTrail = Vec<PuzzleCorner>;

#[derive(Debug)]
struct App {
    paused: bool,
    puzzle_trail: Option<PuzzleTrail>,
}

impl App {
    pub fn new() -> Self {
        Self {
            paused: true,
            puzzle_trail: None,
        }
    }

    pub fn handle_user_input(&mut self) {
        if is_key_pressed(KeyCode::Escape)
            || (self.paused && is_mouse_button_pressed(MouseButton::Left))
        {
            self.paused = !self.paused;
        }
    }
}

fn draw_puzzle(screen_center_x_px: f32, screen_center_y_px: f32) {
    const RECT_WIDTH_PX: f32 = 500.;
    const RECT_HEIGHT_PX: f32 = RECT_WIDTH_PX;

    let (rect_left_px, rect_top_px) = (
        screen_center_x_px - RECT_WIDTH_PX / 2.,
        screen_center_y_px - RECT_HEIGHT_PX / 2.,
    );

    const PUZZLE_BACKGROUND: Color = Color::from_rgba(255, 255, 255, 80);
    draw_rectangle(
        rect_left_px,
        rect_top_px,
        RECT_WIDTH_PX,
        RECT_HEIGHT_PX,
        PUZZLE_BACKGROUND,
    );

    const NUM_LINES: usize = 3;
    const GRID_LINE_THICKNESS: f32 = 10.;
    const GRID_LINE_COLOR: Color = Color::from_hex(0x888888);
    for i in 0..NUM_LINES + 1 {
        let x = rect_left_px + i as f32 * RECT_WIDTH_PX / NUM_LINES as f32;
        draw_line(
            x,
            rect_top_px,
            x,
            rect_top_px + RECT_HEIGHT_PX,
            GRID_LINE_THICKNESS,
            GRID_LINE_COLOR,
        );
        let y = rect_top_px + i as f32 * RECT_HEIGHT_PX / NUM_LINES as f32;
        draw_line(
            rect_left_px,
            y,
            rect_left_px + RECT_WIDTH_PX,
            y,
            GRID_LINE_THICKNESS,
            GRID_LINE_COLOR,
        );
    }

    draw_circle(
        rect_left_px,
        rect_top_px + RECT_HEIGHT_PX,
        CURSOR_SIZE * 1.5,
        GRID_LINE_COLOR,
    );

    const END_NUB_LENGTH: f32 = 40.;
    draw_line(
        rect_left_px + RECT_WIDTH_PX,
        rect_top_px,
        rect_left_px + RECT_WIDTH_PX,
        rect_top_px - END_NUB_LENGTH,
        GRID_LINE_THICKNESS,
        GRID_LINE_COLOR,
    );
}

fn draw_cursor() {
    let (mouse_x, mouse_y) = mouse_position();

    const CURSOR_COLOR: Color = Color::from_rgba(255, 255, 255, 200);
    draw_circle(mouse_x, mouse_y, CURSOR_SIZE, CURSOR_COLOR);
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut app = App::new();
    dbg!(&app);

    loop {
        app.handle_user_input();

        clear_background(BLACK);

        let (screen_width_px, screen_height_px) = screen_size();
        let (screen_center_x_px, screen_center_y_px) =
            (screen_width_px / 2., screen_height_px / 2.);

        draw_puzzle(screen_center_x_px, screen_center_y_px);
        if !app.paused {
            draw_cursor();
        }

        next_frame().await
    }
}
