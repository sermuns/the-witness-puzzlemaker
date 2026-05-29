use std::cmp::Ordering;

use macroquad::{miniquad::window::screen_size, prelude::*};

const CURSOR_SIZE: f32 = 15.;
const CURSOR_COLOR: Color = Color::from_rgba(255, 255, 255, 200);

const PUZZLE_START_CIRCLE_SIZE: f32 = CURSOR_SIZE * 1.5;
const PUZZLE_INACTIVE_BACKGROUND: Color = Color::from_hex(0x555555);
const PUZZLE_ACTIVE_BACKGROUND: Color = Color::from_hex(0x333333);

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

    pub fn as_px(&self) -> (f32, f32) {
        let (screen_center_x_px, screen_center_y_px) = (screen_size().0 / 2., screen_size().1 / 2.);

        let (puzzle_left_px, puzzle_top_px) =
            get_puzzle_left_and_top_px(screen_center_x_px, screen_center_y_px);

        (
            puzzle_left_px + self.column as f32 * PUZZLE_WIDTH_PX / PUZZLE_NUM_COLUMNS as f32,
            puzzle_top_px + self.row as f32 * PUZZLE_HEIGHT_PX / PUZZLE_NUM_ROWS as f32,
        )
    }

    pub fn closest() -> Option<Self> {
        let (mouse_x, mouse_y) = mouse_position();
        let (screen_center_x_px, screen_center_y_px) = (screen_size().0 / 2., screen_size().1 / 2.);

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

    pub fn is_being_touched_by_cursor(&self) -> bool {
        let (puzzle_corner_x_px, puzzle_corner_y_px) = self.as_px();

        let (mouse_x, mouse_y) = mouse_position();
        let dx = mouse_x - puzzle_corner_x_px;
        let dy = mouse_y - puzzle_corner_y_px;
        dx * dx + dy * dy <= (1.5 * PUZZLE_START_CIRCLE_SIZE).powi(2)
    }

    pub fn straight_trail_to(&self, other: &PuzzleCorner) -> Option<Vec<PuzzleCorner>> {
        match (self.column.cmp(&other.column), self.row.cmp(&other.row)) {
            (Ordering::Equal, Ordering::Greater) => Some(
                (other.row..=self.row)
                    .map(|row| PuzzleCorner::new(self.column, row))
                    .collect(),
            ),
            (Ordering::Equal, Ordering::Less) => Some(
                (self.row..=other.row)
                    .map(|row| PuzzleCorner::new(self.column, row))
                    .collect(),
            ),
            (Ordering::Greater, Ordering::Equal) => Some(
                (other.column..=self.column)
                    .map(|column| PuzzleCorner::new(column, self.row))
                    .collect(),
            ),
            (Ordering::Less, Ordering::Equal) => Some(
                (self.column..=other.column)
                    .map(|column| PuzzleCorner::new(column, self.row))
                    .collect(),
            ),
            _ => None,
        }
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

    pub fn handle_user_input(&mut self) {
        if is_mouse_button_pressed(MouseButton::Right) || is_key_pressed(KeyCode::Escape) {
            self.puzzle_trail.clear();
            return;
        }

        // FIXME: unreadable shit
        if self
            .puzzle_trail
            .iter()
            .nth_back(1)
            .is_some_and(|c| c.is_being_touched_by_cursor())
        {
            self.puzzle_trail.pop();
        } else if let Some(last_corner) = self.puzzle_trail.last()
            && let Some(closest) = PuzzleCorner::closest()
            && closest.is_being_touched_by_cursor()
            && !self.puzzle_trail.contains(&closest)
            && let Some(straight_trail) = &mut last_corner.straight_trail_to(&closest)
        {
            self.puzzle_trail.append(straight_trail);
        } else if is_mouse_button_pressed(MouseButton::Left) {
            let start = PuzzleCorner::new(0, PUZZLE_NUM_ROWS);
            if start.is_being_touched_by_cursor() {
                self.puzzle_trail.push(start);
            }
        }
    }
}

fn draw_puzzle(puzzle_trail: &[PuzzleCorner]) {
    let (screen_center_x_px, screen_center_y_px) = (screen_size().0 / 2., screen_size().1 / 2.);

    let (puzzle_left_px, puzzle_top_px) =
        get_puzzle_left_and_top_px(screen_center_x_px, screen_center_y_px);
    let puzzle_right_px = puzzle_left_px + PUZZLE_WIDTH_PX;
    let puzzle_bottom_px = puzzle_top_px + PUZZLE_HEIGHT_PX;

    draw_rectangle(
        puzzle_left_px,
        puzzle_top_px,
        PUZZLE_WIDTH_PX,
        PUZZLE_HEIGHT_PX,
        if puzzle_trail.is_empty() {
            PUZZLE_INACTIVE_BACKGROUND
        } else {
            PUZZLE_ACTIVE_BACKGROUND
        },
    );

    const GRID_LINE_THICKNESS: f32 = 15.;
    const GRID_LINE_COLOR: Color = Color::from_hex(0x888888);
    for i in 0..PUZZLE_NUM_ROWS + 1 {
        let x = puzzle_left_px + i as f32 * PUZZLE_WIDTH_PX / PUZZLE_NUM_ROWS as f32;
        draw_line(
            x,
            puzzle_top_px,
            x,
            puzzle_bottom_px,
            GRID_LINE_THICKNESS,
            GRID_LINE_COLOR,
        );
        let y = puzzle_top_px + i as f32 * PUZZLE_HEIGHT_PX / PUZZLE_NUM_ROWS as f32;
        draw_line(
            puzzle_left_px,
            y,
            puzzle_right_px,
            y,
            GRID_LINE_THICKNESS,
            GRID_LINE_COLOR,
        );
    }

    const END_NUB_LENGTH: f32 = 40.;
    draw_line(
        puzzle_right_px,
        puzzle_top_px,
        puzzle_right_px,
        puzzle_top_px - END_NUB_LENGTH,
        GRID_LINE_THICKNESS,
        GRID_LINE_COLOR,
    );

    draw_circle(
        puzzle_left_px,
        puzzle_bottom_px,
        CURSOR_SIZE * 1.5,
        if puzzle_trail.is_empty() {
            GRID_LINE_COLOR
        } else {
            PUZZLE_TRAIL_COLOR
        },
    );

    let (mouse_x_px, mouse_y_px) = mouse_position();

    let Some(last_corner) = puzzle_trail.last() else {
        draw_circle(mouse_x_px, mouse_y_px, CURSOR_SIZE, CURSOR_COLOR);
        return;
    };

    let (last_corner_x_px, last_corner_y_px) = last_corner.as_px();
    let (projected_mouse_x, projected_mouse_y) =
        if (mouse_x_px - last_corner_x_px).abs() > (mouse_y_px - last_corner_y_px).abs() {
            (
                mouse_x_px.clamp(puzzle_left_px, puzzle_right_px),
                last_corner_y_px,
            )
        } else {
            (
                last_corner_x_px,
                mouse_y_px.clamp(puzzle_top_px, puzzle_bottom_px),
            )
        };

    for (i, [corner1, corner2]) in puzzle_trail.array_windows().enumerate() {
        let (corner1_x_px, corner1_y_px) = corner1.as_px();
        let (corner2_x_px, corner2_y_px) = corner2.as_px();

        draw_text(
            format!("({i}) {},{}", corner2.column, corner2.row).as_str(),
            corner2_x_px + 10.,
            corner2_y_px - 10.,
            20.,
            GRID_LINE_COLOR,
        );

        draw_line(
            corner1_x_px,
            corner1_y_px,
            corner2_x_px,
            corner2_y_px,
            GRID_LINE_THICKNESS,
            PUZZLE_TRAIL_COLOR,
        );
    }

    let (last_corner_x_px, last_corner_y_px) = last_corner.as_px();
    draw_line(
        last_corner_x_px,
        last_corner_y_px,
        projected_mouse_x,
        projected_mouse_y,
        GRID_LINE_THICKNESS,
        PUZZLE_TRAIL_COLOR,
    );

    draw_circle(
        projected_mouse_x,
        projected_mouse_y,
        CURSOR_SIZE,
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
        app.handle_user_input();

        clear_background(BLACK);

        draw_puzzle(&app.puzzle_trail);

        next_frame().await
    }
}
