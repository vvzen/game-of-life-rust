use nannou::prelude::*;

#[derive(Debug, Copy, Clone)]
struct Line {
    start_x: f32,
    start_y: f32,
    end_x: f32,
    end_y: f32,
    weight: f32,
}

struct Cell {
    is_alive: bool,
}

struct Model {
    lines: Vec<Line>,
    cells: Vec<Vec<bool>>,
    app_width: f32,
    app_height: f32,
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

fn draw_cell(x: f32, y: f32, size: f32, alive: bool, model: &Model, canvas: &Draw) {
    let color = if !alive { BLACK } else { WHITE };

    // Convert from the 0,0 top-left system that I'm used to,
    // to the 0,0 is center system that nannou uses
    let x_as_index = x * size;
    let y_as_index = y * size;

    let real_x = x_as_index - (model.app_width * 0.5) + (size * 0.5);
    let real_y = -y_as_index + (model.app_height * 0.5) - (size * 0.5);

    //real_x = real_x * size;
    //real_y = real_y * size;

    //println!("Drawing cell at {}x{}", real_x, real_y);

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

    let lines = draw_grid(app, 32);
    println!("Created {} lines", lines.len());

    let window = app.window_rect();
    //let width_i = window.w() as i32;
    //let height_i = window.h() as i32;
    let width = window.w() as f32;
    let height = window.h() as f32;

    let mut cells = Vec::new();

    for _x in 0..10 {
        let mut rows = Vec::new();
        for _y in 0..10 {
            rows.push(false);
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
        lines: lines,
        cells: cells,
        app_width: width,
        app_height: height,
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let canvas = app.draw();

    for line in model.lines.iter() {
        //println!("Drawing line on x={}, and weight={}", line.x, line.weight);

        canvas
            .line()
            .start(pt2(line.start_x, line.start_y))
            .end(pt2(line.end_x, line.end_y))
            .weight(line.weight)
            .color(BLACK);
    }

    draw_cell(0.0, 0.0, 32.0, false, model, &canvas);
    draw_cell(1.0, 1.0, 32.0, false, model, &canvas);
    draw_cell(2.0, 2.0, 32.0, false, model, &canvas);

    canvas.background().color(WHITE);
    canvas.to_frame(app, &frame).unwrap();
}
