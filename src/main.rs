use macroquad::{miniquad::window::screen_size, prelude::*};

const CURSOR_SIZE: f32 = 15.;
const PUZZLE_START_CIRCLE_SIZE: f32 = CURSOR_SIZE * 1.5;
const PUZZLE_TRAIL_COLOR: Color = Color::from_hex(0xcccc00);

const PUZZLE_WIDTH_PX: f32 = 500.;
const PUZZLE_HEIGHT_PX: f32 = PUZZLE_WIDTH_PX;

const PUZZLE_NUM_ROWS: usize = 3;
const PUZZLE_NUM_COLUMNS: usize = PUZZLE_NUM_ROWS;

#[derive(Debug, PartialEq)]
struct PuzzleCorner {
    column: usize,
    row: usize,
}

impl PuzzleCorner {
    pub fn new(column: usize, row: usize) -> Self {
        Self { column, row }
    }

    pub fn closest(
        (mouse_x, mouse_y): (f32, f32),
        screen_center_x_px: f32,
        screen_center_y_px: f32,
    ) -> Option<Self> {
        let (puzzle_left_px, puzzle_top_px) =
            get_puzzle_left_and_top_px(screen_center_x_px, screen_center_y_px);

        let puzzle_local_mouse_x = mouse_x - puzzle_left_px;
        let puzzle_local_mouse_y = mouse_y - puzzle_top_px;

        if puzzle_local_mouse_x.is_sign_negative()
            || puzzle_local_mouse_y.is_sign_negative()
            || puzzle_local_mouse_x > PUZZLE_WIDTH_PX
            || puzzle_local_mouse_y > PUZZLE_HEIGHT_PX
        {
            // out of bounds
            return None;
        }

        let column = (puzzle_local_mouse_x / (PUZZLE_WIDTH_PX / PUZZLE_NUM_COLUMNS as f32)).round();
        let row = (puzzle_local_mouse_y / (PUZZLE_HEIGHT_PX / PUZZLE_NUM_ROWS as f32)).round();

        Some(Self {
            column: column as usize,
            row: row as usize,
        })
    }

    pub fn is_being_touched_by_cursor(
        &self,
        (mouse_x, mouse_y): (f32, f32),
        screen_center_x_px: f32,
        screen_center_y_px: f32,
    ) -> bool {
        let (puzzle_left_px, puzzle_top_px) =
            get_puzzle_left_and_top_px(screen_center_x_px, screen_center_y_px);

        let puzzle_corner_x_px =
            puzzle_left_px + self.column as f32 * PUZZLE_WIDTH_PX / PUZZLE_NUM_COLUMNS as f32;
        let puzzle_corner_y_px =
            puzzle_top_px + self.row as f32 * PUZZLE_HEIGHT_PX / PUZZLE_NUM_ROWS as f32;

        let dx = mouse_x - puzzle_corner_x_px;
        let dy = mouse_y - puzzle_corner_y_px;
        dx * dx + dy * dy <= (1.5 * PUZZLE_START_CIRCLE_SIZE).powi(2)
    }
}

#[derive(Debug)]
struct App {
    puzzle_trail: Vec<PuzzleCorner>,
}

fn get_puzzle_left_and_top_px(screen_center_x_px: f32, screen_center_y_px: f32) -> (f32, f32) {
    (
        screen_center_x_px - PUZZLE_WIDTH_PX / 2.,
        screen_center_y_px - PUZZLE_HEIGHT_PX / 2.,
    )
}

impl App {
    pub fn new() -> Self {
        Self {
            puzzle_trail: Vec::new(),
        }
    }

    pub fn handle_user_input(&mut self, screen_center_x_px: f32, screen_center_y_px: f32) {
        if is_mouse_button_pressed(MouseButton::Right) || is_key_pressed(KeyCode::Escape) {
            self.puzzle_trail.clear();
            return;
        }

        if let Some(last_corner) = self.puzzle_trail.last()
            && let Some(closest) =
                PuzzleCorner::closest(mouse_position(), screen_center_x_px, screen_center_y_px)
            && closest.is_being_touched_by_cursor(
                mouse_position(),
                screen_center_x_px,
                screen_center_y_px,
            )
            && !self.puzzle_trail.contains(&closest)
        {
            self.puzzle_trail.push(closest);
        } else if is_mouse_button_pressed(MouseButton::Left)
            && PuzzleCorner::new(0, PUZZLE_NUM_ROWS).is_being_touched_by_cursor(
                mouse_position(),
                screen_center_x_px,
                screen_center_y_px,
            )
        {
            self.puzzle_trail
                .push(PuzzleCorner::new(0, PUZZLE_NUM_ROWS));
        }
    }

    pub fn draw_cursor(&self) {
        let (mouse_x, mouse_y) = mouse_position();

        const CURSOR_COLOR: Color = Color::from_rgba(255, 255, 255, 200);
        draw_circle(mouse_x, mouse_y, CURSOR_SIZE, CURSOR_COLOR);
    }
}

fn draw_puzzle(screen_center_x_px: f32, screen_center_y_px: f32, puzzle_trail: &[PuzzleCorner]) {
    let (puzzle_left_px, puzzle_top_px) =
        get_puzzle_left_and_top_px(screen_center_x_px, screen_center_y_px);

    const PUZZLE_BACKGROUND: Color = Color::from_rgba(255, 255, 255, 80);
    draw_rectangle(
        puzzle_left_px,
        puzzle_top_px,
        PUZZLE_WIDTH_PX,
        PUZZLE_HEIGHT_PX,
        PUZZLE_BACKGROUND,
    );

    const GRID_LINE_THICKNESS: f32 = 12.;
    const GRID_LINE_COLOR: Color = Color::from_hex(0x888888);
    for i in 0..PUZZLE_NUM_ROWS + 1 {
        let x = puzzle_left_px + i as f32 * PUZZLE_WIDTH_PX / PUZZLE_NUM_ROWS as f32;
        draw_line(
            x,
            puzzle_top_px,
            x,
            puzzle_top_px + PUZZLE_HEIGHT_PX,
            GRID_LINE_THICKNESS,
            GRID_LINE_COLOR,
        );
        let y = puzzle_top_px + i as f32 * PUZZLE_HEIGHT_PX / PUZZLE_NUM_ROWS as f32;
        draw_line(
            puzzle_left_px,
            y,
            puzzle_left_px + PUZZLE_WIDTH_PX,
            y,
            GRID_LINE_THICKNESS,
            GRID_LINE_COLOR,
        );
    }

    const END_NUB_LENGTH: f32 = 40.;
    draw_line(
        puzzle_left_px + PUZZLE_WIDTH_PX,
        puzzle_top_px,
        puzzle_left_px + PUZZLE_WIDTH_PX,
        puzzle_top_px - END_NUB_LENGTH,
        GRID_LINE_THICKNESS,
        GRID_LINE_COLOR,
    );

    draw_circle(
        puzzle_left_px,
        puzzle_top_px + PUZZLE_HEIGHT_PX,
        CURSOR_SIZE * 1.5,
        if puzzle_trail.is_empty() {
            GRID_LINE_COLOR
        } else {
            PUZZLE_TRAIL_COLOR
        },
    );

    let Some(last_corner) = puzzle_trail.last() else {
        return;
    };

    for [corner1, corner2] in puzzle_trail.array_windows() {
        draw_line(
            puzzle_left_px + corner1.column as f32 * PUZZLE_WIDTH_PX / PUZZLE_NUM_COLUMNS as f32,
            puzzle_top_px + corner1.row as f32 * PUZZLE_HEIGHT_PX / PUZZLE_NUM_ROWS as f32,
            puzzle_left_px + corner2.column as f32 * PUZZLE_WIDTH_PX / PUZZLE_NUM_COLUMNS as f32,
            puzzle_top_px + corner2.row as f32 * PUZZLE_HEIGHT_PX / PUZZLE_NUM_ROWS as f32,
            GRID_LINE_THICKNESS,
            PUZZLE_TRAIL_COLOR,
        );
    }

    let (mouse_x_px, mouse_y_px) = mouse_position();
    let (last_corner_x_px, last_corner_y_px) = (
        puzzle_left_px + last_corner.column as f32 * PUZZLE_WIDTH_PX / PUZZLE_NUM_COLUMNS as f32,
        puzzle_top_px + last_corner.row as f32 * PUZZLE_HEIGHT_PX / PUZZLE_NUM_ROWS as f32,
    );
    let (projected_mouse_x, projected_mouse_y) =
        if (mouse_x_px - last_corner_x_px).abs() > (mouse_y_px - last_corner_y_px).abs() {
            (mouse_x_px, last_corner_y_px)
        } else {
            (last_corner_x_px, mouse_y_px)
        };

    draw_line(
        puzzle_left_px + last_corner.column as f32 * PUZZLE_WIDTH_PX / PUZZLE_NUM_COLUMNS as f32,
        puzzle_top_px + last_corner.row as f32 * PUZZLE_HEIGHT_PX / PUZZLE_NUM_ROWS as f32,
        projected_mouse_x,
        projected_mouse_y,
        GRID_LINE_THICKNESS,
        PUZZLE_TRAIL_COLOR,
    );
}

fn window_conf() -> Conf {
    Conf {
        window_title: env!("CARGO_PKG_NAME").into(),
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut app = App::new();

    loop {
        let (screen_width_px, screen_height_px) = screen_size();
        let (screen_center_x_px, screen_center_y_px) =
            (screen_width_px / 2., screen_height_px / 2.);

        app.handle_user_input(screen_center_x_px, screen_center_y_px);

        clear_background(BLACK);

        draw_puzzle(screen_center_x_px, screen_center_y_px, &app.puzzle_trail);
        app.draw_cursor();

        next_frame().await
    }
}
