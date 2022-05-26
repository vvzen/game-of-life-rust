use nannou::prelude::*;

pub struct Model {
    pub lines: Vec<Line>,
    pub cells: Vec<Vec<bool>>,
    pub app_width: f32,
    pub app_height: f32,
    pub step_size: usize,
    pub num_cells_x: usize,
    pub num_cells_y: usize,
    pub generator: rand::rngs::ThreadRng,
}

#[derive(Debug, Copy, Clone)]
pub struct Line {
    pub start_x: f32,
    pub start_y: f32,
    pub end_x: f32,
    pub end_y: f32,
    pub weight: f32,
}

#[derive(Debug, Copy, Clone)]
pub struct Cell {
    pub is_alive: bool,
}

pub fn draw_grid(app: &App, step_size: usize) -> Vec<Line> {
    let mut lines = Vec::new();
    let mut horizontal_lines = Vec::new();
    let mut vertical_lines = Vec::new();

    let window = app.window_rect();
    let width = window.w() as i32;
    let height = window.w() as i32;

    let start_w = -width / 2;
    let end_w = width / 2;

    let start_h = -height / 2;
    let end_h = height / 2;

    // Horizontal lines
    for i in (start_h..end_h).step_by(step_size) {
        let current_y = i as f32;

        //let show_thicker_line = match horizontal_lines.len() % 10 {
        //    1 => true,
        //    _ => false,
        //};

        //let current_weight = if show_thicker_line { 2.0 } else { 0.5 } as f32;
        let current_weight = 0.5;

        let line_props = Line {
            start_x: start_w as f32,
            end_x: end_w as f32,
            start_y: current_y,
            end_y: current_y,
            weight: current_weight,
        };

        horizontal_lines.push(line_props);
        lines.push(line_props);
    }

    // Vertical lines
    for j in (start_w..end_w).step_by(step_size) {
        let current_x = j as f32;

        //let show_thicker_line = match vertical_lines.len() % 10 {
        //    1 => true,
        //    _ => false,
        //};
        //let current_weight = if show_thicker_line { 2.0 } else { 0.5 } as f32;
        let current_weight = 0.5;

        let line_props = Line {
            start_x: current_x,
            end_x: current_x,
            start_y: start_h as f32,
            end_y: end_h as f32,
            weight: current_weight,
        };

        vertical_lines.push(line_props);
        lines.push(line_props);
    }

    return lines;
}

pub fn draw_cell(x: usize, y: usize, alive: &bool, model: &Model, canvas: &Draw) {
    let size = model.step_size as f32;

    // Convert from the 0,0 top-left system that I'm used to,
    // to the 0,0-is-center system that nannou uses
    let x_scaled = x as f32 * size;
    let y_scaled = y as f32 * size;
    let real_x = x_scaled - (model.app_width * 0.5) + (size * 0.5);
    let real_y = -y_scaled + (model.app_height * 0.5) - (size * 0.5);

    //println!("Drawing cell at {}x{}", real_x, real_y);
    let color = if !alive { BLACK } else { WHITE };

    canvas
        .quad()
        .w(size)
        .h(size)
        .x_y(real_x, real_y)
        .color(color);
}