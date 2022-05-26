use nannou::prelude::*;
use rand::Rng;

#[derive(Debug, Copy, Clone)]
struct Line {
    start_x: f32,
    start_y: f32,
    end_x: f32,
    end_y: f32,
    weight: f32,
}

#[derive(Debug, Copy, Clone)]
struct Cell {
    is_alive: bool,
}

struct Model {
    lines: Vec<Line>,
    cells: Vec<Vec<bool>>,
    app_width: f32,
    app_height: f32,
    step_size: usize,
    num_cells_x: usize,
    num_cells_y: usize,
    generator: rand::rngs::ThreadRng,
}

fn draw_grid(app: &App, step_size: usize) -> Vec<Line> {
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

fn draw_cell(x: usize, y: usize, alive: &bool, model: &Model, canvas: &Draw) {
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

fn main() {
    //nannou::app(model).update(update).simple_window(view).run();
    nannou::app(model).update(update).view(view).run();
}

fn model(app: &App) -> Model {
    // Set up the window size
    app.new_window().size(512, 512).build().unwrap();
    app.main_window().set_resizable(false);

    let step_size = 32;

    let lines = draw_grid(app, step_size);
    println!("Created {} lines", lines.len());

    let window = app.window_rect();
    //let width_i = window.w() as i32;
    //let height_i = window.h() as i32;
    let width = window.w() as f32;
    let height = window.h() as f32;

    let mut cells = Vec::new();

    let num_cells_x = width as i32 / step_size as i32;
    let num_cells_y = height as i32 / step_size as i32;

    let mut generator = rand::thread_rng();

    for _x in 0..num_cells_x {
        let mut rows = Vec::new();

        for _y in 0..num_cells_y {
            let rand_num = generator.gen_range(0, 2);
            rows.push(rand_num != 0);
        }
        cells.push(rows);
    }

    println!(
        "The first element of the 2D array is {:?}",
        cells.get(0).unwrap().get(0).unwrap()
    );

    println!("Canvas size is {}x{}", width, height);
    //println!("The 2D array is {:?}", cells);

    Model {
        lines,
        cells,
        app_width: width,
        app_height: height,
        step_size,
        num_cells_x: num_cells_x as usize,
        num_cells_y: num_cells_y as usize,
        generator,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    // Do the game of life
    for i in 0..model.num_cells_x {
        let rows = &mut model.cells[i];

        for j in 0..model.num_cells_y {
            let rand_num = model.generator.gen_range(0, 2);
            rows[j] = rand_num != 0;
        }

        model.cells[i] = rows.to_vec();
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let canvas = app.draw();

    // Draw the background grid
    for line in model.lines.iter() {
        canvas
            .line()
            .start(pt2(line.start_x, line.start_y))
            .end(pt2(line.end_x, line.end_y))
            .weight(line.weight)
            .color(BLACK);
    }

    // Draw the cells
    for (i, cell_row) in model.cells.iter().enumerate() {
        for (j, cell_value) in cell_row.iter().enumerate() {
            draw_cell(i, j, cell_value, model, &canvas);
        }
    }

    canvas.background().color(WHITE);
    canvas.to_frame(app, &frame).unwrap();
}
